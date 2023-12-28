use itertools::Itertools;
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    dbg!(run(&input));
    Ok(())
}

fn run(input: &str) -> (usize, usize) {
    let mut bricks: Vec<Vec<(usize, usize, usize)>> = input
        .lines()
        .map(|l| {
            let (x0, y0, z0, x1, y1, z1) = l
                .split(['~', ','])
                .map(|x| x.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            let len = x1 + y1 + z1 - x0 - y0 - z0;
            assert!(x1 >= x0);
            assert!(y1 >= y0);
            assert!(z1 >= z0);
            let (dx, dy, dz) = (
                (x1 != x0) as usize,
                (y1 != y0) as usize,
                (z1 != z0) as usize,
            );
            assert!(len == 0 || dx + dy + dz == 1);
            (0..=len)
                .map(|n| (x0 + n * dx, y0 + n * dy, z0 + n * dz))
                .collect_vec()
        })
        .collect();
    settle(&mut bricks);
    let p1 = (0..bricks.len())
        .filter(|ix| find_falling(&bricks, Some(*ix)).is_empty())
        .count();
    let p2 = (0..bricks.len())
        .map(|ix| {
            let mut orig = bricks.clone();
            orig.remove(ix);
            let mut settled = orig.clone();
            settle(&mut settled);
            orig.iter()
                .zip(settled.iter())
                .filter(|(a, b)| a != b)
                .count()
        })
        .sum();
    (p1, p2)
}

fn settle(bricks: &mut [Vec<(usize, usize, usize)>]) {
    loop {
        let falling = find_falling(&bricks, None);
        if falling.len() == 0 {
            break;
        }
        for ix in falling {
            bricks[ix].iter_mut().for_each(|xyz| xyz.2 -= 1);
        }
    }
}

fn find_falling(bricks: &[Vec<(usize, usize, usize)>], ignoring: Option<usize>) -> Vec<usize> {
    let occupied: HashMap<(usize, usize, usize), usize> = bricks
        .iter()
        .enumerate()
        .flat_map(|(ix, brick)| brick.iter().copied().map(move |b| (b, ix)))
        .collect();
    bricks
        .iter()
        .enumerate()
        .filter(|(ix, b)| {
            b.iter().all(|xyz| {
                let down = (xyz.0, xyz.1, xyz.2 - 1);
                down.2 > 0 && [ignoring.as_ref(), Some(ix), None].contains(&occupied.get(&down))
            })
        })
        .map(|(ix, _)| ix)
        .collect_vec()
}

#[test]
fn t() {
    let input = "
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
    assert_eq!(run(input.trim()).0, 5);
}
