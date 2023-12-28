use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use pathfinding::directed::bfs::bfs_reach;

fn moves(map: &[Vec<u8>], x: isize, y: isize, wrapping: bool) -> Vec<(isize, isize)> {
    [(0, 1), (0, -1), (-1, 0), (1, 0)]
        .into_iter()
        .map(|(dx, dy)| (x as isize + dx, y + dy))
        .filter(|(x1, y1)| {
            let x1 = if wrapping {
                x1.rem_euclid(map[0].len() as isize)
            } else {
                *x1
            } as usize;
            let y1 = if wrapping {
                y1.rem_euclid(map.len() as isize)
            } else {
                *y1
            } as usize;
            map.get(y1).and_then(|l| l.get(x1)).copied().unwrap_or(b'#') != b'#'
        })
        .collect_vec()
}

fn reachable_exact(map: &[Vec<u8>], x0: isize, y0: isize, steps: usize, wrapping: bool) -> usize {
    bfs_reach((x0, y0, steps), |(x, y, s)| {
        if *s > 0 {
            moves(&map, *x, *y, wrapping)
                .into_iter()
                .map(|(x, y)| (x, y, s - 1))
                .collect_vec()
        } else {
            vec![]
        }
    })
    .filter(|(_, _, s)| *s == 0)
    .count()
}

fn reachable_even_wrapping(
    map: &[Vec<u8>],
    x0: isize,
    y0: isize,
    steps: usize,
) -> HashSet<(isize, isize)> {
    let mut current_layer = HashSet::new();
    let mut visited = HashSet::new();
    let mut solutions = HashSet::new();
    if steps % 2 == 0 {
        solutions.insert((x0, y0));
    }
    current_layer.insert((x0, y0));
    for step in 0..steps {
        visited = visited.union(&current_layer).copied().collect();
        current_layer = current_layer
            .iter()
            .flat_map(|(x0, y0)| moves(&map, *x0, *y0, true))
            .filter(|pair| !visited.contains(pair))
            .collect();
        if (steps - step) % 2 == 1 {
            solutions.extend(current_layer.iter().copied())
        }
    }
    solutions
}

fn parse(input: &str) -> (Vec<Vec<u8>>, isize, isize) {
    let map = input.lines().map(|l| l.as_bytes().to_vec()).collect_vec();
    let (x0, y0) = map
        .iter()
        .enumerate()
        .find_map(|(y, l)| l.iter().position(|c| *c == b'S').map(|x| (x, y)))
        .unwrap();
    (map, x0 as isize, y0 as isize)
}

fn p2(map: &[Vec<u8>], x0: isize, y0: isize, steps: usize) -> usize {
    let len = map.len() as isize;
    let two_len = 2 * len;
    let growth = steps / two_len as usize - 6;
    let base = steps - growth * two_len as usize;
    let mut blocks = HashMap::<(usize, usize), usize>::default();
    for (x, y) in reachable_even_wrapping(map, x0, y0, base as usize) {
        *blocks
            .entry((
                (x.abs() / (2 * len)) as usize,
                (y.abs() / (2 * len)) as usize,
            ))
            .or_default() += 1;
    }
    let xmax = blocks.keys().map(|p| p.0).max().unwrap();
    let ymax = blocks.keys().map(|p| p.1).max().unwrap();
    for y in 0..=ymax {
        for x in 0..=xmax {
            print!("{:8}", blocks.get(&(x, y)).copied().unwrap_or_default());
        }
        println!();
    }
    /* given example from aoc, steps = 1000
     *      605     626     626     626     626     511      58
     *      626     648     648     648     542     106       0
     *      626     648     648     542     106       0       0
     *      626     648     542     106       0       0       0
     *      626     542     106       0       0       0       0
     *      515     106       0       0       0       0       0
     *       58       0       0       0       0       0       0
     */
    // my input needs 3 diagonals before it saturates the blocks
    /*
     *  118444  118706  118706  118706  117850   79365    7512
     *  118706  118968  118968  118112   80483   10391       0
     *  118706  118968  118112   80483   10391       0       0
     *  118706  118112   80483   10391       0       0       0
     *  117850   80483   10391       0       0       0       0
     *   79365   10391       0       0       0       0       0
     *    7511       0       0       0       0       0       0
     */
    let fixed = blocks[&(0, 0)]
        + blocks[&(xmax - 2, 0)]
        + blocks[&(xmax - 1, 0)]
        + blocks[&(xmax, 0)]
        + blocks[&(0, ymax - 2)]
        + blocks[&(0, ymax - 1)]
        + blocks[&(0, ymax)];
    let top = blocks[&(1, 0)] * (3 + growth);
    let left = blocks[&(0, 1)] * (3 + growth);
    let diag_inner = blocks[&(1, ymax - 3)] * (3 + growth);
    let diag_mid = blocks[&(1, ymax - 2)] * (4 + growth);
    let diag_outer = blocks[&(1, ymax - 1)] * (5 + growth);
    let full = blocks[&(1, 1)] * (growth + 3) * (growth + 2) / 2;
    fixed + top + left + diag_inner + diag_mid + diag_outer + full
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap().trim().to_string();
    let (map, x0, y0) = parse(&input);
    let p1 = reachable_exact(&map, x0, y0, 64, false);
    dbg!(p1);
    dbg!(p2(&map, x0, y0, 26501365));
}

#[test]
fn test2() {
    let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
"
    .trim();
    let (map, x0, y0) = parse(&input);
    assert_eq!(p2(&map, x0, y0, 1000), 668697);
}
