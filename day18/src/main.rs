use pathfinding::directed::bfs::bfs_reach;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn part(input: &str, part2: bool) -> usize {
    let moves = input
        .lines()
        .map(|line| {
            if !part2 {
                let (dir, len, _rgb) = line.split_whitespace().collect_tuple().unwrap();
                let (dx, dy) = match dir {
                    "R" => (1, 0),
                    "L" => (-1, 0),
                    "U" => (0, 1),
                    "D" => (0, -1),
                    _ => panic!(),
                };
                let len = len.parse::<usize>().unwrap();
                (dx, dy, len)
            } else {
                let rgb = line
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .trim_start_matches("(#");
                let len = usize::from_str_radix(&rgb.chars().take(5).join(""), 16).unwrap();
                let (dx, dy) = match rgb.trim_end_matches(")").chars().last().unwrap() {
                    '0' => (1, 0),
                    '1' => (0, -1),
                    '2' => (-1, 0),
                    '3' => (0, 1),
                    _ => panic!(),
                };
                (dx, dy, len)
            }
        })
        .collect_vec();
    let (mut x, mut y) = (0, 0);
    let mut x_boundaries = HashSet::<isize>::default();
    let mut y_boundaries = HashSet::<isize>::default();
    for &(dx, dy, len) in &moves {
        x += len as isize * dx;
        y += len as isize * dy;
        if dx != 0 {
            x_boundaries.insert(x - 1);
            x_boundaries.insert(x);
            x_boundaries.insert(x + 1);
        } else {
            y_boundaries.insert(y - 1);
            y_boundaries.insert(y);
            y_boundaries.insert(y + 1);
        };
    }
    let zx2x = x_boundaries.iter().sorted().unique().copied().collect_vec();
    let x2zx: HashMap<isize, usize> = zx2x.iter().enumerate().map(|p| (*p.1, p.0)).collect();
    let zy2y = y_boundaries.iter().sorted().unique().copied().collect_vec();
    let y2zy: HashMap<isize, usize> = zy2y.iter().enumerate().map(|p| (*p.1, p.0)).collect();
    let mut zdigged = HashSet::<(usize, usize)>::default();
    for &(dx, dy, len) in &moves {
        let (zx, zy) = (x2zx[&x], y2zy[&y]);
        x += len as isize * dx;
        y += len as isize * dy;
        let (zx1, zy1) = (x2zx[&x], y2zy[&y]);
        let zlen = zx1.abs_diff(zx) + zy1.abs_diff(zy);
        for n in 0..zlen {
            zdigged.insert((
                (zx as isize + n as isize * dx) as usize,
                (zy as isize + n as isize * dy) as usize,
            ));
        }
    }
    let ymin = zdigged.iter().map(|p| p.1).min().unwrap();
    let xmin = zdigged
        .iter()
        .filter(|p| p.1 == ymin)
        .map(|p| p.0)
        .min()
        .unwrap();
    let reach = bfs_reach((xmin + 1, ymin + 1), |&(x, y)| {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .map(move |(dx, dy)| ((x as isize + dx) as usize, (y as isize + dy) as usize))
            .filter(|loc| !zdigged.contains(&loc))
    })
    .map(|(zx, zy)| ((zx2x[zx + 1] - zx2x[zx]) * (zy2y[zy + 1] - zy2y[zy])) as usize)
    .sum::<usize>();
    let trench: usize = moves.iter().map(|t| t.2).sum();
    reach + trench
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    dbg!(part(&input, false));
    dbg!(part(&input, true));
    Ok(())
}
