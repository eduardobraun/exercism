fn atbash_encode_char(c: char) -> Option<char> {
    const ASCII_INVERT: u8 = b'a' + b'z';

    let c = (c as u8).to_ascii_lowercase();
    match c {
        b'a'..=b'z' => Some((ASCII_INVERT - c) as char),
        b'0'..=b'9' => Some(c as char),
        _ => None,
    }
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .filter_map(|c| atbash_encode_char(c))
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
        .filter_map(|c| atbash_encode_char(c))
        .collect::<String>()
}
