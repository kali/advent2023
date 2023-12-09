use itertools::Itertools;
use num::integer::lcm;
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let moves = input.lines().next().unwrap();
    let graph: HashMap<_, _> = input
        .lines()
        .skip(2)
        .map(|line| {
            let (key, rest) = line.split_once(" = (").unwrap();
            let (left, right) = rest.trim_end_matches(')').split_once(", ").unwrap();
            (key, (left, right))
        })
        .collect();
    let next = |pos: &str, mov: char| {
        if mov == 'L' {
            graph[pos].0
        } else {
            graph[pos].1
        }
    };
    let p1 = moves
        .chars()
        .cycle()
        .clone()
        .scan("AAA", |pos, mov| {
            *pos = next(pos, mov);
            Some(pos.to_string())
        })
        .position(|p| p == "ZZZ")
        .unwrap() + 1;
    dbg!(p1);

    let starts = graph
        .keys()
        .filter(|k| k.ends_with('A'))
        .copied()
        .sorted()
        .collect_vec();
    let mut ghosts = vec![];
    for entry in starts.iter().copied() {
        let mut pos = entry;
        let mut step = 0i64;
        let mut success = vec![];
        let mut history = HashMap::new();
        let (start, end) = 'l: loop {
            for (ix, mov) in moves.chars().enumerate() {
                pos = next(pos, mov);
                step += 1;
                if pos.ends_with("Z") {
                    success.push(step);
                }
                if let Some(start) = history.insert((ix, pos), step) {
                    break 'l (start, step);
                }
            }
        };
        let period = end - start;
        assert!(success.len() == 1);
        let success = success[0];
        assert!(success == period);
        ghosts.push(period);
    }
    let p2 = ghosts.iter().copied().reduce(lcm).unwrap();
    dbg!(p2);
    Ok(())
}
