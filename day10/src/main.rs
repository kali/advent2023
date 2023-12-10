use std::collections::{HashMap, HashSet};

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let mut maze: HashMap<(isize, isize), char> = input
        .lines()
        .enumerate()
        .flat_map(move |(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), c))
        })
        .collect();
    macro_rules! at {
        ($x:expr, $y:expr) => {
            maze.get(&($x, $y)).copied().unwrap_or('.')
        };
    }
    let &(x0, y0) = maze.iter().find(|(_, v)| **v == 'S').unwrap().0;
    let left = "FL-".contains(at!(x0 - 1, y0));
    let right = "7J-".contains(at!(x0 + 1, y0));
    let up = "|F7".contains(at!(x0, y0 - 1));
    let fix = match (left, right, up) {
        (false, false, true) => '|',
        (true, true, false) => '-',
        (false, true, false) => 'F',
        (true, false, true) => 'J',
        (true, false, false) => '7',
        _ => 'L',
    };
    maze.insert((x0, y0), fix);
    let (dx0, dy0) = if left {
        (-1, 0)
    } else if right {
        (1, 0)
    } else {
        (0, 1)
    };
    let mut path = HashSet::<(isize, isize)>::default();
    path.insert((x0, y0));
    let mut state = (x0, y0, dx0, dy0);
    loop {
        let (x, y, dx, dy) = (state.0 + state.2, state.1 + state.3, state.2, state.3);
        path.insert((x, y));
        if (x, y) == (x0, y0) {
            break;
        };
        let (dx, dy) = match (at!(x, y), dx) {
            ('F', 0) | ('L', 0) => (1, 0),
            ('J', 0) | ('7', 0) => (-1, 0),
            ('F', _) | ('7', _) => (0, 1),
            ('J', _) | ('L', _) => (0, -1),
            _ => (dx, dy),
        };
        state = (x, y, dx, dy);
    }
    dbg!(path.len().div_ceil(2));
    let mut p2 = 0;
    for y in 0..input.lines().count() as isize {
        let mut inside = false;
        for x in 0..input.lines().next().unwrap().len() as isize {
            let on_path = path.contains(&(x, y));
            inside = inside ^ (on_path && "|LJ".contains(at!(x, y)));
            p2 += (inside && !on_path) as usize;
        }
    }
    dbg!(p2);
    Ok(())
}
