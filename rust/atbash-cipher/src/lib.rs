fn ascii_invert_az(c: char) -> Option<char> {
    const ASCII_INVERTER: u8 = 'a' as u8 + 'z' as u8;
    match c {
        'a'..='z' => Some((ASCII_INVERTER - c as u8) as char),
        '0'..='9' => Some(c),
        _ => None,
    }
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .to_lowercase()
        .chars()
        .filter_map(|c| ascii_invert_az(c))
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter_map(|c| ascii_invert_az(c))
        .collect::<String>()
}
