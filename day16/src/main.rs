use std::collections::HashSet;

use itertools::Itertools;

fn score(map: &[Vec<u8>], x: usize, y: usize, dx: isize, dy: isize) -> usize {
    let mut visited = HashSet::<(isize, isize, isize, isize)>::default();
    let mut todo = Vec::<(isize, isize, isize, isize)>::default();
    todo.push((x as isize, y as isize, dx, dy));
    while let Some((x, y, dx, dy)) = todo.pop() {
        if !visited.insert((x, y, dx, dy)) {
            continue;
        }
        let c = map[y as usize][x as usize];
        let next = match (c, dx, dy) {
            (b'|', _, 0) => vec![(0, -1), (0, 1)],
            (b'-', 0, _) => vec![(-1, 0), (1, 0)],
            (b'/', d, 0) => vec![(0, -d)],
            (b'/', 0, d) => vec![(-d, 0)],
            (b'\\', d, 0) => vec![(0, d)],
            (b'\\', 0, d) => vec![(d, 0)],
            _ => vec![(dx, dy)],
        };
        for (dx, dy) in next {
            let (x, y) = (x + dx, y + dy);
            if x >= 0 && y >= 0 && x < map[0].len() as isize && y < map.len() as isize {
                todo.push((x, y, dx, dy));
            }
        }
    }
    let visited: HashSet<_> = visited.iter().map(|(x, y, _, _)| (x, y)).collect();
    visited.len()
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let map = input.lines().map(|s| s.bytes().collect_vec()).collect_vec();
    let p1 = score(&map, 0, 0, 1, 0);
    dbg!(p1);
    let vert = (0..map[0].len())
        .map(|x| score(&map, x, 0, 0, 1).max(score(&map, x, map.len() - 1, 0, -1)))
        .max()
        .unwrap();
    let horiz = (0..map.len())
        .map(|y| score(&map, 0, y, 1, 0).max(score(&map, map[0].len() - 1, y, -1, 0)))
        .max()
        .unwrap();
    dbg!(vert.max(horiz));
    Ok(())
}
