fn is_prime(i: u32) -> bool {
    if i == 2 {
        return true;
    }
    if i == 1 {
        return false;
    }
    if i % 2 == 0 {
        return false;
    }
    let mut j = 3u32;
    let stop = (i as f64).sqrt() as u32;
    while j <= stop {
        if i % j == 0 {
            return false;
        }
        j += 2;
    }
    true
}

fn next_prime(n: u32) -> u32 {
    match n {
        0 => 2,
        1 => 2,
        2 => 3,
        _ => {
            let mut np = n + 2;
            while !is_prime(np) {
                np += 2;
            }
            np
        }
    }
}

pub fn nth(n: u32) -> u32 {
    let mut p = 0u32;
    for _i in 0..=n {
        p = next_prime(p);
    }
    p
}
