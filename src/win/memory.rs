use napi::bindgen_prelude::*;
use napi_derive::napi;
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS};
use crate::utils::{decode_wide};

#[napi(object)]
#[derive(Debug)]
pub struct Process {
    pub pid: u32,
    pub name: String,
}

#[napi]
pub async fn get_processes() -> Result<Vec<Process>> {
    match tokio::spawn(async move {
        let mut processes = Vec::new();

        let snapshot = match unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) } {
            Ok(snapshot) => snapshot,
            Err(err) => {
                return Err(format!("CreateToolhelp32Snapshot failed: {:?}", err));
            }
        };

        let mut process_entry = PROCESSENTRY32W::default();
        process_entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

        if let Err(err) = unsafe { Process32FirstW(snapshot, &mut process_entry) } {
            return Err(format!("Process32First failed: {:?}", err));
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
            return Err(format!("CloseHandle failed: {:?}", err));
        }

        Ok(processes)
    }).await {
        Ok(processes) => {
            match processes {
                Ok(processes) => Ok(processes),
                Err(e) => Err(Error::new(
                    Status::GenericFailure,
                    format!("Error: {:?}", e),
                )),
            }
        },
        Err(e) => Err(Error::new(
            Status::GenericFailure,
            format!("Error: {:?}", e),
        )),
    }
}