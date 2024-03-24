pub fn trim(chars: &[u16]) -> String {
    for (i, c) in chars.iter().enumerate() {
        if *c == 0 {
            return String::from_utf16_lossy(&chars[..i]).to_string();
        }
    }

    String::from_utf16_lossy(chars).to_string()
}