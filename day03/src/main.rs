use std::collections::HashMap;
use std::iter::once;

use itertools::Itertools;

fn around(x: usize, y: usize, len: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..len + 2)
        .map(move |dx| (x - 1 + dx, y - 1))
        .chain((0..len + 2).map(move |dx| (x - 1 + dx, y + 1)))
        .chain(once((x - 1, y)))
        .chain(once((x + len, y)))
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let mut map = input
        .lines()
        .map(|line| {
            let mut line = line.as_bytes().to_vec();
            line.insert(0, b'.');
            line.push(b'.');
            line
        })
        .collect_vec();
    let dots = vec![b'.'; map[0].len()];
    map.insert(0, dots.clone());
    map.push(dots);
    let mut p1 = 0;
    let mut gears = HashMap::<(usize, usize), Vec<usize>>::new();
    for x in 1..map.len() - 1 {
        for y in 1..map.len() - 1 {
            if map[y][x].is_ascii_digit() && !map[y][x - 1].is_ascii_digit() {
                let len = (0..).find(|dx| !map[y][x + dx].is_ascii_digit()).unwrap();
                let n = std::str::from_utf8(&map[y][x..][..len])
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                if around(x, y, len).any(|(x, y)| map[y][x] != b'.') {
                    p1 += n;
                }
                for (gx, gy) in around(x, y, len) {
                    if map[gy][gx] == b'*' {
                        gears.entry((gx, gy)).or_default().push(n);
                    }
                }
            }
        }
    }
    dbg!(p1);
    let p2: usize = gears
        .values()
        .filter(|n| n.len() == 2)
        .map(|n| n[0] * n[1])
        .sum();
    dbg!(p2);
    Ok(())
}
