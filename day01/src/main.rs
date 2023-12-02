fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let p1 = input
        .lines()
        .map(|line| {
            let first = line.chars().filter(|x| x.is_ascii_digit()).next().unwrap();
            let last = line
                .chars()
                .filter(|x| x.is_ascii_digit())
                .rev()
                .next()
                .unwrap();
            (first as usize - '0' as usize) * 10 + (last as usize - '0' as usize)
        })
        .sum::<usize>();
    dbg!(p1);
    let literals: Vec<&str> = "### one two three four five six seven eight nine"
        .split_whitespace()
        .collect();
    let p2 = input
        .lines()
        .map(|line| {
            let first = (0..line.len())
                .find_map(|skip| {
                    let s: String = line.chars().skip(skip).collect();
                    if s.chars().next().unwrap().is_ascii_digit() {
                        Some(s.chars().next().unwrap() as usize - '0' as usize)
                    } else {
                        literals.iter().position(|l| s.starts_with(l))
                    }
                })
                .unwrap();
            let last = (0..line.len())
                .find_map(|skip| {
                    let s:String = line
                        .chars()
                        .rev()
                        .skip(skip)
                        .collect::<String>()
                        .chars()
                        .rev()
                        .collect();
                    if s.chars().last().unwrap().is_ascii_digit() {
                        Some(s.chars().last().unwrap() as usize - '0' as usize)
                    } else {
                        literals.iter().position(|l| s.ends_with(l))
                    }
                })
                .unwrap();
            first * 10 + last
        })
        .sum::<usize>();
    dbg!(p2);
    Ok(())
}
