use itertools::Itertools;

fn find_reflection(map: &[Vec<bool>], ignore: Option<usize>) -> Option<usize> {
    for n in 1..map[0].len() {
        if map.iter().all(|line| {
            line.iter()
                .take(n)
                .rev()
                .zip(line.iter().skip(n))
                .all(|(a, b)| a == b)
        }) {
            if ignore == Some(n) {
                continue;
            }
            return Some(n);
        }
    }
    for n in 1..map.len() {
        if map
            .iter()
            .take(n)
            .rev()
            .zip(map.iter().skip(n))
            .all(|(a, b)| a == b)
        {
            if ignore == Some(100 * n) {
                continue;
            }
            return Some(100 * n);
        }
    }
    None
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let maps: Vec<Vec<Vec<bool>>> = input
        .lines()
        .map(|l| l.bytes().map(|b| b == b'#').collect_vec())
        .batching(|it| Some(it.take_while(|l| l.len() > 0).collect_vec()).filter(|m| m.len() > 0))
        .collect_vec();
    let mut p1 = 0;
    let mut p2 = 0;
    for mut map in maps {
        let raw = find_reflection(&map, None).unwrap();
        p1 += raw;
        'p2: for x in 0..map[0].len() {
            for y in 0..map.len() {
                map[y][x] = !map[y][x];
                if let Some(found) = find_reflection(&map, Some(raw)) {
                    p2 += found;
                    break 'p2;
                }
                map[y][x] = !map[y][x];
            }
        }
    }
    dbg!(p1);
    dbg!(p2);
    Ok(())
}
