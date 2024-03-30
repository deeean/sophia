use napi::{Error, Status, Result};

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