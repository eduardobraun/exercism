fn is_prime(i: u64) -> bool {
    if i == 2 {
        return true;
    }
    if i == 1 {
        return false;
    }
    if i % 2 == 0 {
        return false;
    }
    let mut j = 3u64;
    let stop = (i as f64).sqrt() as u64;
    while j <= stop {
        if i % j == 0 {
            return false;
        }
        j += 2;
    }
    true
}

fn next_prime(n: u64) -> u64 {
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

pub fn factors(n: u64) -> Vec<u64> {
    let mut p = next_prime(0);
    let mut n = n;
    let mut v: Vec<u64> = vec![];
    while n > 1 {
        while n % p == 0 {
            n = n / p;
            v.push(p);
        }
        p = next_prime(p);
    }
    v
}
