fn proverb(list: &[&str]) -> String {
    match &list {
        &[] => "".to_string(),
        &[_a] => "".to_string(),
        &[a, b] => format!("For want of a {} the {} was lost.\n", a, b).to_string(),
        &[a, tail @ ..] => {
            format!("For want of a {} the {} was lost.\n", a, tail[0]).to_string() + &proverb(&tail)
        }
    }
}

pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        return "".to_string();
    }
    proverb(list) + &format!("And all for the want of a {}.", list[0])
}
