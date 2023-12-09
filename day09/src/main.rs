use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let mut series = vec![];
    for line in input.lines() {
        let values = line
            .split_whitespace()
            .map(|s| s.parse::<isize>().unwrap())
            .collect_vec();
        let mut ds = vec![values];
        while ds.last().unwrap().iter().any(|&x| x != 0) {
            let d = ds
                .last()
                .unwrap()
                .windows(2)
                .map(|pair| pair[1] - pair[0])
                .collect_vec();
            ds.push(d);
        }
        series.push(ds);
    }
    let p1 = series
        .iter()
        .map(|series| series.iter().map(|d| d.last().unwrap()).sum::<isize>())
        .sum::<isize>();
    let p2 = series
        .iter()
        .map(|series| {
            series
                .iter()
                .zip([1, -1].into_iter().cycle())
                .map(|(d, sign)| d.first().unwrap() * sign)
                .sum::<isize>()
        })
        .sum::<isize>();
    dbg!(p1, p2);
    Ok(())
}
