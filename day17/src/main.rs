use itertools::Itertools;
use pathfinding::directed::astar::astar;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let map = input
        .lines()
        .map(|v| v.bytes().map(|x| x - b'0').collect_vec())
        .collect_vec();
    for (min, max) in [(1, 3), (4, 10)] {
        // x y dx dy count
        let path = astar(
            &(0isize, 0isize, 0isize, 0isize, 0usize),
            |&(x, y, dx, dy, count)| {
                let mut next = vec![];
                for (dx1, dy1) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                    let (x, y) = (x + dx1, y + dy1);
                    if x < 0
                        || y < 0
                        || x >= map[0].len() as isize
                        || y >= map.len() as isize
                        || (dx1, dy1) == (-dx, -dy)
                        || (count == max && (dx1, dy1) == (dx, dy))
                        || (count < min && dx1 != dx)
                    {
                        continue;
                    }
                    let count = if dx1 == dx && dy1 == dy { count + 1 } else { 1 };
                    next.push((
                        (x, y, dx1, dy1, count),
                        map[y as usize][x as usize] as usize,
                    ));
                }
                next
            },
            |&(x, y, _, _, _)| map[0].len() - 1 - x as usize + map.len() - 1 - y as usize,
            |&(x, y, _, _, _)| x as usize == map[0].len() - 1 && y as usize == map.len() - 1,
        )
        .unwrap();
        dbg!(path.1);
    }
    Ok(())
}
