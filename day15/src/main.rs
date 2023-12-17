fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let p1 = input.split(",").map(|s| hash(s) as usize).sum::<usize>();
    dbg!(p1);
    let mut boxes: Vec<Vec<(String, usize)>> = vec![vec!(); 256];
    for step in input.split(",") {
        if step.ends_with("-") {
            let label = step.trim_end_matches("-");
            let boxx = hash(label) as usize;
            if let Some(pos) = boxes[boxx].iter().position(|slot| slot.0 == label) {
                boxes[boxx].remove(pos);
            }
        } else {
            let (label, focale) = step.split_once("=").unwrap();
            let focale = focale.parse::<usize>().unwrap();
            let boxx = hash(label) as usize;
            if let Some(pos) = boxes[boxx].iter().position(|slot| slot.0 == label) {
                boxes[boxx][pos].1 = focale;
            } else {
                boxes[boxx].push((label.to_string(), focale));
            }
        }
    }
    let p2 = boxes
        .iter()
        .enumerate()
        .flat_map(|(bix, boxx)| {
            boxx.iter()
                .enumerate()
                .map(move |(lix, l)| (bix + 1) * (lix + 1) * l.1)
        })
        .sum::<usize>();
    dbg!(p2);
    Ok(())
}

fn hash(s: &str) -> u8 {
    s.bytes()
        .fold(0u8, |acc, c| acc.wrapping_add(c as u8).wrapping_mul(17))
}
