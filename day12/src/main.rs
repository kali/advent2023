use itertools::Itertools;

fn groups(springs: &[u8]) -> Vec<usize> {
    springs
        .split(|c| *c != b'#')
        .map(|g| g.len())
        .filter(|x| *x > 0)
        .collect_vec()
}

fn solutions(springs: &str, spec: &[usize], mult: usize) -> usize {
    let spec = (0..mult).flat_map(|_| spec.iter().copied()).collect_vec();
    let springs = (0..mult).map(|_| springs).join("?").bytes().collect_vec();
    // weight, repr seq
    let mut paths: Vec<(usize, Vec<u8>)> = vec![(1, vec![])];
    for token in springs {
        let mut new = vec![];
        for prefix in &paths {
            if token != b'#' {
                new.push(prefix.clone());
                new.last_mut().unwrap().1.push(b'.');
            }
            if token != b'.' {
                new.push(prefix.clone());
                new.last_mut().unwrap().1.push(b'#');
            }
        }
        let new = new
            .into_iter()
            .filter(|new| {
                let g = groups(&new.1);
                if new.1.last().unwrap() == &b'.' {
                    g.iter().zip(spec.iter()).all(|(a, b)| a == b)
                } else {
                    g.iter()
                        .take(g.len() - 1)
                        .zip(spec.iter())
                        .all(|(a, b)| a == b)
                        && g.len() <= spec.len()
                        && g[g.len() - 1] <= spec[g.len() - 1]
                }
            })
            .collect_vec();
        let mut reduced = vec![];
        for path in new {
            if path.1.last().unwrap() == &b'#' {
                reduced.push(path);
            } else {
                let g = groups(&path.1);
                if let Some(pos) = reduced
                    .iter()
                    .position(|red| red.1.last().unwrap() == &b'.' && groups(&red.1) == g)
                {
                    reduced[pos].0 += path.0;
                } else {
                    reduced.push(path);
                }
            }
        }
        paths = reduced;
    }
    paths
        .iter()
        .filter(|(_, new)| groups(new) == spec)
        .map(|(c, _)| c)
        .sum()
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    for mult in [1, 5] {
        let mut count = 0;
        for line in input.lines() {
            // dbg!(line);
            let (springs, spec) = line.split_once(" ").unwrap();
            let spec = spec.split(",").map(|s| s.parse().unwrap()).collect_vec();
            count += solutions(springs, &spec, mult);
        }
        dbg!(count);
    }
    Ok(())
}

#[test]
fn t() {
    assert_eq!(solutions("???.###", &[1, 1, 3], 1), 1);
    assert_eq!(solutions("????.#...#...", &[4, 1, 1], 5), 16);
}
