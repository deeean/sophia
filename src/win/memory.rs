use napi::bindgen_prelude::*;
use napi_derive::napi;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;
use windows::Win32::System::Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS};
use windows::Win32::System::ProcessStatus::{EnumProcessModules, GetModuleFileNameExW};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_ACCESS_RIGHTS, PROCESS_ALL_ACCESS, PROCESS_CREATE_PROCESS, PROCESS_CREATE_THREAD, PROCESS_DELETE, PROCESS_DUP_HANDLE, PROCESS_QUERY_INFORMATION, PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_READ_CONTROL, PROCESS_SET_INFORMATION, PROCESS_SET_LIMITED_INFORMATION, PROCESS_SET_QUOTA, PROCESS_SET_SESSIONID, PROCESS_SYNCHRONIZE, PROCESS_TERMINATE, PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE, PROCESS_WRITE_DAC, PROCESS_WRITE_OWNER};
use crate::utils::{bigint_to_u8, bigint_to_i8, bigint_to_u16, bigint_to_i16, bigint_to_u32, bigint_to_u64, bigint_to_i32, decode_wide, handle_result, bigint_to_i64, bigint_to_usize};

#[napi(object)]
#[derive(Debug)]
pub struct Process {
    pub pid: u32,
    pub name: String,
}

#[napi]
pub enum ProcessAccess {
    AllAccess,
    CreateProcess,
    CreateThread,
    Delete,
    DupHandle,
    QueryInformation,
    QueryLimitedInformation,
    ReadControl,
    SetInformation,
    SetLimitedInformation,
    SetQuota,
    SetSessionId,
    Synchronize,
    Terminate,
    VmOperation,
    VmRead,
    VmWrite,
    WriteDac,
    WriteOwner,
}

impl From<ProcessAccess> for PROCESS_ACCESS_RIGHTS {
    fn from(access: ProcessAccess) -> Self {
        match access {
            ProcessAccess::AllAccess => PROCESS_ALL_ACCESS,
            ProcessAccess::CreateProcess => PROCESS_CREATE_PROCESS,
            ProcessAccess::CreateThread => PROCESS_CREATE_THREAD,
            ProcessAccess::Delete => PROCESS_DELETE,
            ProcessAccess::DupHandle => PROCESS_DUP_HANDLE,
            ProcessAccess::QueryInformation => PROCESS_QUERY_INFORMATION,
            ProcessAccess::QueryLimitedInformation => PROCESS_QUERY_LIMITED_INFORMATION,
            ProcessAccess::ReadControl => PROCESS_READ_CONTROL,
            ProcessAccess::SetInformation => PROCESS_SET_INFORMATION,
            ProcessAccess::SetLimitedInformation => PROCESS_SET_LIMITED_INFORMATION,
            ProcessAccess::SetQuota => PROCESS_SET_QUOTA,
            ProcessAccess::SetSessionId => PROCESS_SET_SESSIONID,
            ProcessAccess::Synchronize => PROCESS_SYNCHRONIZE,
            ProcessAccess::Terminate => PROCESS_TERMINATE,
            ProcessAccess::VmOperation => PROCESS_VM_OPERATION,
            ProcessAccess::VmRead => PROCESS_VM_READ,
            ProcessAccess::VmWrite => PROCESS_VM_WRITE,
            ProcessAccess::WriteDac => PROCESS_WRITE_DAC,
            ProcessAccess::WriteOwner => PROCESS_WRITE_OWNER,
        }
    }
}
#[napi]
#[derive(Debug)]
pub struct OpenedProcess {
    handle: HANDLE,
    base_address: usize,
}

pub fn get_base_address(handle: HANDLE) -> std::result::Result<usize, std::io::Error> {
    let mut lph_module = windows::Win32::Foundation::HMODULE::default();
    let mut cb_needed = 0u32;

    if let Err(_) = unsafe {
        EnumProcessModules(
            handle,
            &mut lph_module,
            std::mem::size_of_val(&lph_module) as u32,
            &mut cb_needed,
        )
    } {
        return Err(std::io::Error::last_os_error());
    }

    let mut sz_module_name = [0u16; 256];

    unsafe {
        GetModuleFileNameExW(handle, lph_module, &mut sz_module_name);
    }

    Ok(lph_module.0 as usize)
}

fn read_memory_inner<T: Default>(handle: HANDLE, address: usize) -> std::result::Result<T, std::io::Error> {
    let mut result = T::default();

    if let Err(_) = unsafe {
        ReadProcessMemory(
            handle,
            address as *const _,
            &mut result as *mut _ as *mut _,
            std::mem::size_of::<T>(),
            None,
        )
    } {
        return Err(std::io::Error::last_os_error());
    }

    Ok(result)
}

fn read_memory_chain_inner<T: Default>(
    handle: HANDLE,
    base_address: usize,
    offsets: &[usize],
) -> std::result::Result<T, std::io::Error> {
    let mut address = read_memory_inner::<usize>(handle, base_address)?;
    let last_index = offsets.len() - 1;

    for i in 0..last_index {
        address = read_memory_inner::<usize>(handle, address + offsets[i])?;
    }

    Ok(read_memory_inner::<T>(handle, address + offsets[last_index])?)
}

async fn read_memory<T: Default + Send + 'static>(
    handle: HANDLE,
    address: u64,
) -> Result<T> where T: Send + 'static, {
    let task = tokio::spawn(async move {
        match read_memory_inner::<T>(handle, address as usize) {
            Ok(result) => Ok(result),
            Err(err) => Err(format!("Failed to read memory: {:?}", err)),
        }
    });

    handle_result(task).await
}

async fn read_memory_chain<T: Default + Send + 'static>(
    handle: HANDLE,
    base_address: u64,
    offsets: Vec<u64>,
) -> Result<T> where T: Send + 'static, {
    let task = tokio::spawn(async move {
        let offsets: Vec<usize> = offsets.iter().map(|&x| x as usize).collect();

        match read_memory_chain_inner::<T>(handle, base_address as usize, &offsets) {
            Ok(result) => Ok(result),
            Err(err) => Err(format!("Failed to read memory chain: {:?}", err)),
        }
    });

    handle_result(task).await
}

fn write_memory_inner<T: Default>(
    handle: HANDLE,
    address: usize,
    value: T,
) -> std::result::Result<(), std::io::Error> {
    if let Err(_) = unsafe {
        windows::Win32::System::Diagnostics::Debug::WriteProcessMemory(
            handle,
            address as *mut _,
            &value as *const _ as *const _,
            std::mem::size_of::<T>(),
            None,
        )
    } {
        return Err(std::io::Error::last_os_error());
    }

    Ok(())
}

fn write_memory_chain_inner<T: Default>(
    handle: HANDLE,
    base_address: usize,
    offsets: &[usize],
    value: T,
) -> std::result::Result<(), std::io::Error> {
    let mut address = read_memory_inner::<usize>(handle, base_address)?;
    let last_index = offsets.len() - 1;

    for i in 0..last_index {
        address = read_memory_inner::<usize>(handle, address + offsets[i])?;
    }

    write_memory_inner::<T>(handle, address + offsets[last_index], value)
}

async fn write_memory<T: Default + Send + 'static>(
    handle: HANDLE,
    address: u64,
    value: T,
) -> Result<()> where T: Send + 'static, {
    let task = tokio::spawn(async move {
        match write_memory_inner::<T>(handle, address as usize, value) {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Failed to write memory: {:?}", err)),
        }
    });

    handle_result(task).await
}

async fn write_memory_chain<T: Default + Send + 'static>(
    handle: HANDLE,
    base_address: u64,
    offsets: Vec<u64>,
    value: T,
) -> Result<()> where T: Send + 'static, {
    let task = tokio::spawn(async move {
        let offsets: Vec<usize> = offsets.iter().map(|&x| x as usize).collect();

        match write_memory_chain_inner::<T>(handle, base_address as usize, &offsets, value) {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Failed to write memory chain: {:?}", err)),
        }
    });

    handle_result(task).await
}

#[napi]
impl OpenedProcess {
    #[napi]
    pub async fn read_memory_bool(&self, address: BigInt) -> Result<bool> {
        let address = bigint_to_u64(address);
        read_memory::<bool>(self.handle, address).await
    }

    #[napi]
    pub async fn read_memory_chain_bool(&self, base_address: BigInt, offsets: Vec<BigInt>) -> Result<bool> {
        let base_address = self.base_address as u64 + bigint_to_u64(base_address);
        let offsets: Vec<u64> = offsets.into_iter().map(|x| bigint_to_u64(x)).collect();
        read_memory_chain::<bool>(self.handle, base_address, offsets).await
    }

    #[napi]
    pub async fn read_memory_uint8(&self, address: BigInt) -> Result<BigInt> {
        let address = bigint_to_u64(address);
        let result = read_memory::<u8>(self.handle, address).await?;
        Ok(BigInt {
            sign_bit: false,
            words: vec![result as u64],
        })
    }

    #[napi]
    pub async fn read_memory_chain_uint8(&self, base_address: BigInt, offsets: Vec<BigInt>) -> Result<BigInt> {
        let base_address = self.base_address as u64 + bigint_to_u64(base_address);
        let offsets: Vec<u64> = offsets.into_iter().map(|x| bigint_to_u64(x)).collect();
        let result = read_memory_chain::<u8>(self.handle, base_address, offsets).await?;
        Ok(BigInt {
            sign_bit: false,
            words: vec![result as u64],
        })
    }

    #[napi]
    pub async fn read_memory_int8(&self, address: BigInt) -> Result<BigInt> {
        let address = bigint_to_u64(address);
        let result = read_memory::<i8>(self.handle, address).await?;
        Ok(BigInt {
            sign_bit: result < 0,
            words: vec![result as u64],
        })
    }

    #[napi]
    pub async fn read_memory_chain_int8(&self, base_address: BigInt, offsets: Vec<BigInt>) -> Result<BigInt> {
        let base_address = self.base_address as u64 + bigint_to_u64(base_address);
        let offsets: Vec<u64> = offsets.into_iter().map(|x| bigint_to_u64(x)).collect();
        let result = read_memory_chain::<i8>(self.handle, base_address, offsets).await?;
        Ok(BigInt {
            sign_bit: result < 0,
            words: vec![result as u64],
        })
    }

    #[napi]
    pub async fn read_memory_uint16(&self, address: BigInt) -> Result<BigInt> {
        let address = bigint_to_u64(address);
        let result = read_memory::<u16>(self.handle, address).await?;
        Ok(BigInt {
            sign_bit: false,
            words: vec![result as u64],
        })
    }

    #[napi]
    pub async fn read_memory_chain_uint16(&self, base_address: BigInt, offsets: Vec<BigInt>) -> Result<BigInt> {
        let base_address = self.base_address as u64 + bigint_to_u64(base_address);
        let offsets: Vec<u64> = offsets.into_iter().map(|x| bigint_to_u64(x)).collect();
        let result = read_memory_chain::<u16>(self.handle, base_address, offsets).await?;
        Ok(BigInt {
            sign_bit: false,
            words: vec![result as u64],
        })
    }

    #[napi]
    pub async fn read_memory_int16(&self, address: BigInt) -> Result<BigInt> {
        let address = bigint_to_u64(address);
        let result = read_memory::<i16>(self.handle, address).await?;
        Ok(BigInt {
            sign_bit: result < 0,
            words: vec![result as u64],
        })
    }

    #[napi]
    pub async fn read_memory_chain_int16(&self, base_address: BigInt, offsets: Vec<BigInt>) -> Result<BigInt> {
        let base_address = self.base_address as u64 + bigint_to_u64(base_address);
        let offsets: Vec<u64> = offsets.into_iter().map(|x| bigint_to_u64(x)).collect();
        let result = read_memory_chain::<i16>(self.handle, base_address, offsets).await?;
        Ok(BigInt {
            sign_bit: result < 0,
            words: vec![result as u64],
        })
    }

    #[napi]
    pub async fn read_memory_uint32(&self, address: BigInt) -> Result<BigInt> {
        let address = bigint_to_u64(address);
        let result = read_memory::<u32>(self.handle, address).await?;
        Ok(BigInt {
            sign_bit: false,
            words: vec![result as u64],
        })
    }

    #[napi]
    pub async fn read_memory_chain_uint32(&self, base_address: BigInt, offsets: Vec<BigInt>) -> Result<BigInt> {
        let base_address = self.base_address as u64 + bigint_to_u64(base_address);
        let offsets: Vec<u64> = offsets.into_iter().map(|x| bigint_to_u64(x)).collect();
        let result = read_memory_chain::<u32>(self.handle, base_address, offsets).await?;
        Ok(BigInt {
            sign_bit: false,
            words: vec![result as u64],
        })
    }

    #[napi]
    pub async fn read_memory_int32(&self, address: BigInt) -> Result<BigInt> {
        let address = bigint_to_u64(address);
        let result = read_memory::<i32>(self.handle, address).await?;
        Ok(BigInt {
            sign_bit: result < 0,
            words: vec![result as u64],
        })
    }

    #[napi]
    pub async fn read_memory_chain_int32(&self, base_address: BigInt, offsets: Vec<BigInt>) -> Result<BigInt> {
        let base_address = self.base_address as u64 + bigint_to_u64(base_address);
        let offsets: Vec<u64> = offsets.into_iter().map(|x| bigint_to_u64(x)).collect();
        let result = read_memory_chain::<i32>(self.handle, base_address, offsets).await?;
        Ok(BigInt {
            sign_bit: result < 0,
            words: vec![result as u64],
        })
    }

    #[napi]
    pub async fn read_memory_uint64(&self, address: BigInt) -> Result<BigInt> {
        let address = bigint_to_u64(address);
        let result = read_memory::<u64>(self.handle, address).await?;
        Ok(BigInt {
            sign_bit: false,
            words: vec![result],
        })
    }

    #[napi]
    pub async fn read_memory_chain_uint64(&self, base_address: BigInt, offsets: Vec<BigInt>) -> Result<BigInt> {
        let base_address = self.base_address as u64 + bigint_to_u64(base_address);
        let offsets: Vec<u64> = offsets.into_iter().map(|x| bigint_to_u64(x)).collect();
        let result = read_memory_chain::<u64>(self.handle, base_address, offsets).await?;
        Ok(BigInt {
            sign_bit: false,
            words: vec![result],
        })
    }

    #[napi]
    pub async fn read_memory_int64(&self, address: BigInt) -> Result<BigInt> {
        let address = bigint_to_u64(address);
        let result = read_memory::<i64>(self.handle, address).await?;
        Ok(BigInt {
            sign_bit: result < 0,
            words: vec![result as u64],
        })
    }

    #[napi]
    pub async fn read_memory_chain_int64(&self, base_address: BigInt, offsets: Vec<BigInt>) -> Result<BigInt> {
        let base_address = self.base_address as u64 + bigint_to_u64(base_address);
        let offsets: Vec<u64> = offsets.into_iter().map(|x| bigint_to_u64(x)).collect();
        let result = read_memory_chain::<i64>(self.handle, base_address, offsets).await?;
        Ok(BigInt {
            sign_bit: result < 0,
            words: vec![result as u64],
        })
    }

    #[napi]
    pub async fn read_memory_usize(&self, address: BigInt) -> Result<BigInt> {
        let address = bigint_to_u64(address);
        let result = read_memory::<usize>(self.handle, address).await?;
        Ok(BigInt {
            sign_bit: false,
            words: vec![result as u64],
        })
    }

    #[napi]
    pub async fn read_memory_chain_usize(&self, base_address: BigInt, offsets: Vec<BigInt>) -> Result<BigInt> {
        let base_address = self.base_address as u64 + bigint_to_u64(base_address);
        let offsets: Vec<u64> = offsets.into_iter().map(|x| bigint_to_u64(x)).collect();
        let result = read_memory_chain::<usize>(self.handle, base_address, offsets).await?;
        Ok(BigInt {
            sign_bit: false,
            words: vec![result as u64],
        })
    }

    #[napi]
    pub async fn read_memory_float32(&self, address: BigInt) -> Result<f32> {
        let address = bigint_to_u64(address);
        read_memory::<f32>(self.handle, address).await
    }

    #[napi]
    pub async fn read_memory_chain_float32(&self, base_address: BigInt, offsets: Vec<BigInt>) -> Result<f32> {
        let base_address = self.base_address as u64 + bigint_to_u64(base_address);
        let offsets: Vec<u64> = offsets.into_iter().map(|x| bigint_to_u64(x)).collect();
        read_memory_chain::<f32>(self.handle, base_address, offsets).await
    }

    #[napi]
    pub async fn read_memory_float64(&self, address: BigInt) -> Result<f64> {
        let address = bigint_to_u64(address);
        read_memory::<f64>(self.handle, address).await
    }

    #[napi]
    pub async fn read_memory_chain_float64(&self, base_address: BigInt, offsets: Vec<BigInt>) -> Result<f64> {
        let base_address = self.base_address as u64 + bigint_to_u64(base_address);
        let offsets: Vec<u64> = offsets.into_iter().map(|x| bigint_to_u64(x)).collect();
        read_memory_chain::<f64>(self.handle, base_address, offsets).await
    }

    #[napi]
    pub async fn write_memory_bool(&self, address: BigInt, value: bool) -> Result<()> {
        let address = bigint_to_u64(address);
        write_memory::<bool>(self.handle, address, value).await
    }

    #[napi]
    pub async fn write_memory_chain_bool(&self, base_address: BigInt, offsets: Vec<BigInt>, value: bool) -> Result<()> {
        let base_address = self.base_address as u64 + bigint_to_u64(base_address);
        let offsets: Vec<u64> = offsets.into_iter().map(|x| bigint_to_u64(x)).collect();
        write_memory_chain::<bool>(self.handle, base_address, offsets, value).await
    }

    #[napi]
    pub async fn write_memory_uint8(&self, address: BigInt, value: BigInt) -> Result<()> {
        let address = bigint_to_u64(address);
        let value = bigint_to_u8(value);
        write_memory::<u8>(self.handle, address, value).await
    }

    #[napi]
    pub async fn write_memory_chain_uint8(&self, base_address: BigInt, offsets: Vec<BigInt>, value: BigInt) -> Result<()> {
        let base_address = self.base_address as u64 + bigint_to_u64(base_address);
        let offsets: Vec<u64> = offsets.into_iter().map(|x| bigint_to_u64(x)).collect();
        let value = bigint_to_u8(value);
        write_memory_chain::<u8>(self.handle, base_address, offsets, value).await
    }

    #[napi]
    pub async fn write_memory_int8(&self, address: BigInt, value: BigInt) -> Result<()> {
        let address = bigint_to_u64(address);
        let value = bigint_to_i8(value);
        write_memory::<i8>(self.handle, address, value).await
    }

    #[napi]
    pub async fn write_memory_chain_int8(&self, base_address: BigInt, offsets: Vec<BigInt>, value: BigInt) -> Result<()> {
        let base_address = self.base_address as u64 + bigint_to_u64(base_address);
        let offsets: Vec<u64> = offsets.into_iter().map(|x| bigint_to_u64(x)).collect();
        let value = bigint_to_i8(value);
        write_memory_chain::<i8>(self.handle, base_address, offsets, value).await
    }

    #[napi]
    pub async fn write_memory_uint16(&self, address: BigInt, value: BigInt) -> Result<()> {
        let address = bigint_to_u64(address);
        let value = bigint_to_u16(value);
        write_memory::<u16>(self.handle, address, value).await
    }

    #[napi]
    pub async fn write_memory_chain_uint16(&self, base_address: BigInt, offsets: Vec<BigInt>, value: BigInt) -> Result<()> {
        let base_address = self.base_address as u64 + bigint_to_u64(base_address);
        let offsets: Vec<u64> = offsets.into_iter().map(|x| bigint_to_u64(x)).collect();
        let value = bigint_to_u16(value);
        write_memory_chain::<u16>(self.handle, base_address, offsets, value).await
    }

    #[napi]
    pub async fn write_memory_int16(&self, address: BigInt, value: BigInt) -> Result<()> {
        let address = bigint_to_u64(address);
        let value = bigint_to_i16(value);
        write_memory::<i16>(self.handle, address, value).await
    }

    #[napi]
    pub async fn write_memory_chain_int16(&self, base_address: BigInt, offsets: Vec<BigInt>, value: BigInt) -> Result<()> {
        let base_address = self.base_address as u64 + bigint_to_u64(base_address);
        let offsets: Vec<u64> = offsets.into_iter().map(|x| bigint_to_u64(x)).collect();
        let value = bigint_to_i16(value);
        write_memory_chain::<i16>(self.handle, base_address, offsets, value).await
    }

    #[napi]
    pub async fn write_memory_uint32(&self, address: BigInt, value: BigInt) -> Result<()> {
        let address = bigint_to_u64(address);
        let value = bigint_to_u32(value);
        write_memory::<u32>(self.handle, address, value).await
    }

    #[napi]
    pub async fn write_memory_chain_uint32(&self, base_address: BigInt, offsets: Vec<BigInt>, value: BigInt) -> Result<()> {
        let base_address = self.base_address as u64 + bigint_to_u64(base_address);
        let offsets: Vec<u64> = offsets.into_iter().map(|x| bigint_to_u64(x)).collect();
        let value = bigint_to_u32(value);
        write_memory_chain::<u32>(self.handle, base_address, offsets, value).await
    }

    #[napi]
    pub async fn write_memory_int32(&self, address: BigInt, value: BigInt) -> Result<()> {
        let address = bigint_to_u64(address);
        let value = bigint_to_i32(value);
        write_memory::<i32>(self.handle, address, value).await
    }

    #[napi]
    pub async fn write_memory_chain_int32(&self, base_address: BigInt, offsets: Vec<BigInt>, value: BigInt) -> Result<()> {
        let base_address = self.base_address as u64 + bigint_to_u64(base_address);
        let offsets: Vec<u64> = offsets.into_iter().map(|x| bigint_to_u64(x)).collect();
        let value = bigint_to_i32(value);
        write_memory_chain::<i32>(self.handle, base_address, offsets, value).await
    }

    #[napi]
    pub async fn write_memory_uint64(&self, address: BigInt, value: BigInt) -> Result<()> {
        let address = bigint_to_u64(address);
        let value = bigint_to_u64(value);
        write_memory::<u64>(self.handle, address, value).await
    }

    #[napi]
    pub async fn write_memory_chain_uint64(&self, base_address: BigInt, offsets: Vec<BigInt>, value: BigInt) -> Result<()> {
        let base_address = self.base_address as u64 + bigint_to_u64(base_address);
        let offsets: Vec<u64> = offsets.into_iter().map(|x| bigint_to_u64(x)).collect();
        let value = bigint_to_u64(value);
        write_memory_chain::<u64>(self.handle, base_address, offsets, value).await
    }

    #[napi]
    pub async fn write_memory_int64(&self, address: BigInt, value: BigInt) -> Result<()> {
        let address = bigint_to_u64(address);
        let value = bigint_to_i64(value);
        write_memory::<i64>(self.handle, address, value).await
    }

    #[napi]
    pub async fn write_memory_chain_int64(&self, base_address: BigInt, offsets: Vec<BigInt>, value: BigInt) -> Result<()> {
        let base_address = self.base_address as u64 + bigint_to_u64(base_address);
        let offsets: Vec<u64> = offsets.into_iter().map(|x| bigint_to_u64(x)).collect();
        let value = bigint_to_i64(value);
        write_memory_chain::<i64>(self.handle, base_address, offsets, value).await
    }

    #[napi]
    pub async fn write_memory_usize(&self, address: BigInt, value: BigInt) -> Result<()> {
        let address = bigint_to_u64(address);
        let value = bigint_to_usize(value);
        write_memory::<usize>(self.handle, address, value).await
    }

    #[napi]
    pub async fn write_memory_chain_usize(&self, base_address: BigInt, offsets: Vec<BigInt>, value: BigInt) -> Result<()> {
        let base_address = self.base_address as u64 + bigint_to_u64(base_address);
        let offsets: Vec<u64> = offsets.into_iter().map(|x| bigint_to_u64(x)).collect();
        let value = bigint_to_usize(value);
        write_memory_chain::<usize>(self.handle, base_address, offsets, value).await
    }

    #[napi]
    pub async fn write_memory_float32(&self, address: BigInt, value: f64) -> Result<()> {
        let address = bigint_to_u64(address);
        write_memory::<f32>(self.handle, address, value as f32).await
    }

    #[napi]
    pub async fn write_memory_chain_float32(&self, base_address: BigInt, offsets: Vec<BigInt>, value: f64) -> Result<()> {
        let base_address = self.base_address as u64 + bigint_to_u64(base_address);
        let offsets: Vec<u64> = offsets.into_iter().map(|x| bigint_to_u64(x)).collect();
        write_memory_chain::<f32>(self.handle, base_address, offsets, value as f32).await
    }

    #[napi]
    pub async fn write_memory_float64(&self, address: BigInt, value: f64) -> Result<()> {
        let address = bigint_to_u64(address);
        write_memory::<f64>(self.handle, address, value).await
    }

    #[napi]
    pub async fn write_memory_chain_float64(&self, base_address: BigInt, offsets: Vec<BigInt>, value: f64) -> Result<()> {
        let base_address = self.base_address as u64 + bigint_to_u64(base_address);
        let offsets: Vec<u64> = offsets.into_iter().map(|x| bigint_to_u64(x)).collect();
        write_memory_chain::<f64>(self.handle, base_address, offsets, value).await
    }
}

impl Drop for OpenedProcess {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.handle).unwrap();
        }
    }
}

#[napi]
pub async fn open_process(access: ProcessAccess, pid: u32) -> Result<OpenedProcess> {
    let task = tokio::spawn(async move {
        match unsafe { OpenProcess(access.into(), false, pid) } {
            Ok(handle) => {
                let base_address = match get_base_address(handle) {
                    Ok(address) => address,
                    Err(err) => {
                        return Err(format!("Failed to get base address: {:?}", err));
                    }
                };

                Ok(OpenedProcess {
                    handle,
                    base_address
                })
            },
            Err(err) => {
                return Err(format!("Failed to open process: {:?}", err));
            }
        }
    });

    handle_result(task).await
}

#[napi]
pub async fn get_processes() -> Result<Vec<Process>> {
    let task = tokio::spawn(async move {
        let mut processes = Vec::new();

        let snapshot = match unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) } {
            Ok(snapshot) => snapshot,
            Err(err) => {
                return Err(format!("Failed to create snapshot: {:?}", err));
            }
        };

        let mut process_entry = PROCESSENTRY32W::default();
        process_entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

        if let Err(err) = unsafe { Process32FirstW(snapshot, &mut process_entry) } {
            return Err(format!("Failed to get first process: {:?}", err));
        }

        loop {
            let curr = decode_wide(&process_entry.szExeFile);

            processes.push(Process {
                pid: process_entry.th32ProcessID,
                name: curr,
            });

            if let Err(_) = unsafe { Process32NextW(snapshot, &mut process_entry) } {
                break;
            }
        }

        if let Err(err) = unsafe { CloseHandle(snapshot) } {
            return Err(format!("Failed to close snapshot: {:?}", err));
        }

        Ok(processes)
    });

    handle_result(task).await
}