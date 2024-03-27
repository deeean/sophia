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