use std::collections::HashMap;

use itertools::Itertools;

fn p1(map: &[&[u8]], path: &mut Vec<(usize, usize)>) -> Option<Vec<(usize, usize)>> {
    let (x0, y0) = *path.last().unwrap();
    if y0 == map.len() {
        return Some(path.clone());
    }
    let current = map[y0][x0];
    let candidates: &[(isize, isize)] = match current {
        b'#' => return None,
        b'>' => &[(1, 0)],
        b'<' => &[(-1, 0)],
        b'v' => &[(0, 1)],
        b'^' => &[(0, -1)],
        _ => &[(1, 0), (-1, 0), (0, 1), (0, -1)],
    };
    let mut best: Option<Vec<(usize, usize)>> = None;
    for (dx, dy) in candidates {
        let (x, y) = ((x0 as isize + dx), (y0 as isize + dy));
        if y > 0 && !path.contains(&(x as usize, y as usize)) {
            path.push((x as usize, y as usize));
            if let Some(this) = p1(map, path) {
                if !best.as_ref().is_some_and(|b| b.len() > this.len()) {
                    best = Some(this)
                }
            }
            path.pop();
        }
    }
    best
}

fn is_node(map: &[&[u8]], x: usize, y: usize) -> bool {
    map[y][x] != b'#'
        && (y == 0
            || y == map.len() - 1
            || [(1, 0), (-1, 0), (0, 1), (0, -1)]
                .iter()
                .filter(|(dx, dy)| {
                    map[(y as isize + dy) as usize][(x as isize + dx) as usize] != b'#'
                })
                .count()
                > 2)
}

fn walk_to_node(
    map: &[&[u8]],
    x0: usize,
    y0: usize,
    dx0: isize,
    dy0: isize,
) -> ((usize, usize), usize) {
    let (x, y) = ((x0 as isize + dx0) as usize, (y0 as isize + dy0) as usize);
    if is_node(map, x, y) {
        ((x, y), 1)
    } else {
        let ((dx, dy),) = [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .filter(|&(dx, dy)| {
                let (x1, y1) = ((x as isize + dx), (y as isize + dy));
                map.get(y1 as usize)
                    .and_then(|line| line.get(x1 as usize))
                    .unwrap_or(&b'#')
                    != &b'#'
                    && (dx, dy) != (-dx0, -dy0)
            })
            .collect_tuple()
            .unwrap();
        let res = walk_to_node(map, x, y, dx, dy);
        (res.0, res.1 + 1)
    }
}

type Reduced = HashMap<(usize, usize), Vec<((usize, usize), usize)>>;
fn reduce(map: &[&[u8]]) -> Reduced {
    let mut reduced: Reduced = Default::default();
    for x in 0..map[0].len() {
        for y in 0..map.len() {
            if !is_node(map, x, y) {
                continue;
            }
            let edges = [(1, 0), (-1, 0), (0, 1), (0, -1)]
                .into_iter()
                .filter(|(dx0, dy0)| {
                    let (x1, y1) = ((x as isize + dx0), (y as isize + dy0));
                    map.get(y1 as usize)
                        .and_then(|line| line.get(x1 as usize))
                        .unwrap_or(&b'#')
                        != &b'#'
                })
                .map(|(dx, dy)| walk_to_node(map, x, y, dx, dy))
                .collect_vec();
            reduced.insert((x, y), edges);
        }
    }
    reduced
}

fn rec_reduced(map: &Reduced, path: &mut Vec<((usize, usize), usize)>) -> Option<usize> {
    let start = *path.last().unwrap();
    if map[&start.0].len() == 1 && start.0 .1 > 0 {
        return Some(path.iter().map(|p| p.1).sum());
    }
    let mut best: Option<usize> = None;
    for (next, cost) in &map[&start.0] {
        if !path.iter().any(|node| node.0 == *next) {
            path.push((*next, *cost));
            if let Some(this) = rec_reduced(map, path) {
                if !best.as_ref().is_some_and(|b| *b > this) {
                    best = Some(this)
                }
            }
            path.pop();
        }
    }
    best
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    dbg!(run(&input));
    Ok(())
}

fn run(input: &str) -> (usize, usize) {
    let map = input.lines().map(|l| l.as_bytes()).collect_vec();
    let x0 = map[0].iter().position(|c| *c == b'.').unwrap();
    let mut path = vec![(x0, 1)];
    let p1 = p1(&map, &mut path).unwrap().len() - 1;
    let reduced = reduce(&map);
    let p2 = rec_reduced(&&reduced, &mut vec![((1, 0), 0)]).unwrap();
    (p1, p2)
}

#[test]
fn t() {
    let input = "
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"
        .trim();
    assert_eq!(run(input), (94, 154));
}
