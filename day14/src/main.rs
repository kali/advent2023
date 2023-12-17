use std::collections::HashMap;

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let map = input.lines().map(|s| s.as_bytes().to_vec()).collect_vec();
    let p1 = load(&mov(&map, 0, -1));
    dbg!(p1);
    dbg!(part2(map));
    Ok(())
}

fn part2(mut map: Vec<Vec<u8>>) -> usize {
    let dirs = [(0, -1), (-1, 0), (0, 1), (1, 0)];
    let mut history: HashMap<Vec<Vec<u8>>, usize> = HashMap::default();
    for step in 0.. {
        for (dx, dy) in dirs {
            map = mov(&map, dx, dy);
        }
        if let Some(enter_loop) = history.get(&map) {
            let extra_steps = (1000000000 - enter_loop) % (step - enter_loop);
            let final_step = enter_loop + extra_steps - 1;
            let map = history
                .iter()
                .find(|(_, v)| **v == final_step)
                .unwrap()
                .0;
            return load(map);
        } else {
            history.insert(map.clone(), step);
        }
    }
    unreachable!();
}

fn mov(map: &Vec<Vec<u8>>, dx: isize, dy: isize) -> Vec<Vec<u8>> {
    let mut map = map.clone();
    loop {
        let mut done_some = false;
        for x in 0..map[0].len() {
            for y in 0..map.len() {
                if map[y][x] == b'O' {
                    let (x1, y1) = (x as isize + dx, y as isize + dy);
                    if x1 >= 0 && y1 >= 0 && x1 < map[0].len() as isize && y1 < map.len() as isize {
                        if map[y1 as usize][x1 as usize] == b'.' {
                            map[y1 as usize][x1 as usize] = b'O';
                            map[y][x] = b'.';
                            done_some = true;
                        }
                    }
                }
            }
        }
        if !done_some {
            return map;
        }
    }
}

fn load(map: &Vec<Vec<u8>>) -> usize {
    let mut score = 0;
    for x in 0..map[0].len() {
        for y in 0..map.len() {
            if map[y][x] == b'O' {
                score += map.len() - y;
            }
        }
    }
    score
}

#[test]
fn t() {
    let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    let map = input.lines().map(|s| s.as_bytes().to_vec()).collect_vec();
    assert_eq!(part2(map), 64);
}
