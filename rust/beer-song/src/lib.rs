fn part1(n: u32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\n".to_string(),
        _ => format!("{0} bottles of beer on the wall, {0} bottles of beer.\n", n),
    }
}

fn part2(n: u32) -> String {
    match n {
        0 => "Go to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "Take it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        2 => "Take one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),
        _ => format!(
            "Take one down and pass it around, {} bottles of beer on the wall.\n",
            n - 1
        ),
    }
}

pub fn verse(n: u32) -> String {
    part1(n) + &part2(n)
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(|n| verse(n))
        .collect::<Vec<_>>()
        .join("\n")
}
