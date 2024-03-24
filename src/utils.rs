pub fn trim(chars: &[u16]) -> String {
    for (i, c) in chars.iter().enumerate() {
        if *c == 0 {
            return String::from_utf16_lossy(&chars[..i]).to_string();
        }
    }

    String::from_utf16_lossy(chars).to_string()
}

pub fn encode_wide_string<S: AsRef<std::ffi::OsStr>>(string: S) -> Vec<u16> {
    std::os::windows::prelude::OsStrExt::encode_wide(string.as_ref())
        .chain(std::iter::once(0))
        .collect()
}