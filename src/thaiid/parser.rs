pub fn decode_tis620(bytes: &[u8]) -> String {
    bytes
        .iter()
        .filter_map(|&b| {
            match b {
                0x20..=0x7E => Some(b as char), // ASCII characters
                0xA1..=0xFB => char::from_u32((b as u32 - 0xA0) + 0x0E00), // ISO 8859-11
                0xA0 => Some(' '), // Treat as space
                _ => None, // Discard other undefined or control characters
            }
        })
        .collect::<String>()
        .replace('#', " ") // if '#' is used as a specific placeholder for unreadable chars
        .trim()
        .to_string()
}
