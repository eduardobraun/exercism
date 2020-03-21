pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    if input.len() == 0 {
        return vec![];
    }

    for row in 0..input.len() {
        if input[row].len() == 0 || input[row].len() != input[0].len() {
            return vec![];
        }
    }

    let mut saddles: Vec<(usize, usize)> = vec![];
    for row in 0..input.len() {
        let rmax = input[row].iter().max().unwrap();
        for col in 0..input[0].len() {
            let cmin = col_min(input, col);
            if input[row][col] == cmin && input[row][col] == *rmax {
                saddles.push((row, col));
            }
        }
    }
    saddles
}

fn col_min(input: &[Vec<u64>], col: usize) -> u64 {
    let mut min: Option<u64> = None;
    for row in 0..input.len() {
        min = match min {
            Some(m) if input[row][col] < m => Some(input[row][col]),
            None => Some(input[row][col]),
            _ => min,
        }
    }
    min.unwrap()
}
