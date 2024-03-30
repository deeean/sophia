use napi::{Error, Status, Result};
use napi::bindgen_prelude::BigInt;

pub fn bigint_to_i8(bigint: BigInt) -> i8 {
    bigint_to_i64(bigint) as i8
}

pub fn bigint_to_u8(bigint: BigInt) -> u8 {
    bigint_to_u64(bigint) as u8
}

pub fn bigint_to_i16(bigint: BigInt) -> i16 {
    bigint_to_i64(bigint) as i16
}

pub fn bigint_to_u16(bigint: BigInt) -> u16 {
    bigint_to_u64(bigint) as u16
}

pub fn bigint_to_i32(bigint: BigInt) -> i32 {
    bigint_to_i64(bigint) as i32
}

pub fn bigint_to_u32(bigint: BigInt) -> u32 {
    bigint_to_u64(bigint) as u32
}

pub fn bigint_to_i64(bigint: BigInt) -> i64 {
    let (value, _) = bigint.get_i64();
    value
}

pub fn bigint_to_u64(bigint: BigInt) -> u64 {
    let (_, value, _) = bigint.get_u64();
    value
}

pub fn bigint_to_usize(bigint: BigInt) -> usize {
    bigint_to_u64(bigint) as usize
}

pub fn encode_wide<S: AsRef<std::ffi::OsStr>>(string: S) -> Vec<u16> {
    std::os::windows::prelude::OsStrExt::encode_wide(string.as_ref())
        .chain(std::iter::once(0))
        .collect()
}

pub fn decode_wide(chars: &[u16]) -> String {
    String::from_utf16_lossy(chars)
        .trim_end_matches('\0')
        .to_string()
}

pub async fn handle_result<T>(task: tokio::task::JoinHandle<std::result::Result<T, String>>) -> Result<T> {
    match task.await {
        Ok(result) => match result {
            Ok(value) => Ok(value),
            Err(e) => Err(Error::new(
                Status::GenericFailure,
                format!("Operation failed: {:?}", e),
            )),
        },
        Err(e) => Err(Error::new(
            Status::GenericFailure,
            format!("Task join failed: {:?}", e),
        )),
    }
}