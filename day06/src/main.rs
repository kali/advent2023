use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let (times, bests) = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(|n| n.parse::<usize>().unwrap())
                .collect_vec()
        })
        .collect_tuple()
        .unwrap();
    let p1 = times
        .iter()
        .zip(bests.iter())
        .map(|(&time, &best)| (0..time).filter(|load| load * (time - load) > best).count())
        .product::<usize>();
    dbg!(p1);

    let time = times.iter().join("").parse::<usize>()?;
    let best = bests.iter().join("").parse::<usize>()?;
    let (left, right) = [(0, time / 2), (time, time / 2)]
        .into_iter()
        .map(|(mut looser, mut winner)| {
            while looser.abs_diff(winner) > 1 {
                let next = (looser + winner) / 2;
                if next * (time - next) < best {
                    looser = next;
                } else {
                    winner = next
                };
            }
            looser.min(winner)
        })
        .collect_tuple()
        .unwrap();
    let p2 = right - left;
    dbg!(p2);
    Ok(())
}
