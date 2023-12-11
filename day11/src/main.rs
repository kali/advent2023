use std::collections::HashSet;

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let universe: HashSet<(usize, usize)> = input
        .lines()
        .enumerate()
        .flat_map(move |(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x, y))
        })
        .collect();
    let x_max = universe.iter().map(|pair| pair.0).max().unwrap();
    let empty_xs = (0..x_max)
        .filter(|x| universe.iter().all(|pair| pair.0 != *x))
        .collect_vec();
    let y_max = universe.iter().map(|pair| pair.1).max().unwrap();
    let empty_ys = (0..y_max)
        .filter(|y| universe.iter().all(|pair| pair.1 != *y))
        .collect_vec();
    for exp in [1, 999_999] {
        let dist = universe
            .iter()
            .combinations(2)
            .map(|pair| {
                let [a, b] = &*pair else { panic!() };
                let (xmin, xmax) = (a.0.min(b.0), a.0.max(b.0));
                let dx =
                    xmax - xmin + exp * empty_xs.iter().filter(|&&x| x > xmin && x < xmax).count();
                let (ymin, ymax) = (a.1.min(b.1), a.1.max(b.1));
                let dy =
                    ymax - ymin + exp * empty_ys.iter().filter(|&&y| y > ymin && y < ymax).count();
                dx + dy
            })
            .sum::<usize>();
        dbg!(dist);
    }
    Ok(())
}
