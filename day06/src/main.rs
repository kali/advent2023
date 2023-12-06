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
    // load*(time-load) = best    <=>    load^2 - time*load + best = 0
    let delta_sqrt = ((time * time - 4 * best) as f64).sqrt();
    let left = ((time as f64 - delta_sqrt) / 2.).floor() as usize;
    let right = ((time as f64 + delta_sqrt) / 2.).floor() as usize;
    dbg!(right - left);
    Ok(())
}
