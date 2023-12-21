use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    dbg!(run(&input));
    Ok(())
}

fn run(input: &str) -> (usize, usize) {
    let mut conj = HashMap::<String, HashMap<String, bool>>::default();
    let mut flips = HashMap::<String, bool>::default();
    let mut targets = HashMap::<&str, Vec<&str>>::default();
    for line in input.lines() {
        let (left, right) = line.split_once(" -> ").unwrap();
        let src = left.trim_start_matches(&['%', '&']);
        let dsts = right.split(", ").collect_vec();
        targets.insert(src, dsts);
        if left.chars().next().unwrap() == '&' {
            conj.insert(left.chars().skip(1).join(""), Default::default());
        } else if left.chars().next().unwrap() == '%' {
            flips.insert(left.chars().skip(1).join(""), false);
        }
    }
    for (src, dst) in targets.iter().flat_map(|(src, dst)| {
        dst.iter().map(|d| {
            (
                src.trim_start_matches(&['&', '%']).to_string(),
                d.to_string(),
            )
        })
    }) {
        if let Some(conj) = conj.get_mut(&dst.to_string()) {
            conj.insert(src.to_string(), false);
        }
    }

    assert!(targets["broadcaster"].len() == 4);
    // check: rx = conj(a, b, c, d);
    let (pre,) = targets
        .iter()
        .filter(|(_, dst)| dst.contains(&"rx"))
        .map(|p| p.0)
        .copied()
        .collect_tuple()
        .unwrap();
    assert!(conj.contains_key(&*pre));
    let prepre = targets
        .iter()
        .filter(|(_, dst)| dst.contains(&pre))
        .map(|p| p.0)
        .copied()
        .collect_vec();
    assert!(prepre.len() == 4);

    // we want rx low => all(a, b, c, d) high.
    // assume each of a b c d is periodic, with a single low pulse every period (after some warmup)

    let mut queue = VecDeque::new();
    let mut counts = [0, 0];
    let mut p1 = 0;
    let mut highs = vec![vec!(); 4];
    // assume 10000 cycles will be enough to observe periodicity
    for i in 0..10000 {
        counts[0] += 1;
        queue.push_back(("broadcaster", false));
        while let Some((from, high)) = queue.pop_front() {
            if high {
                if let Some(pp) = prepre.iter().position(|x| from == *x) {
                    highs[pp].push(i);
                }
            }
            for to in &targets[from] {
                counts[high as usize] += 1;
                if let Some(f) = flips.get_mut(&**to) {
                    if !high {
                        *f = !*f;
                        queue.push_back((to, *f));
                    }
                } else if let Some(c) = conj.get_mut(&**to) {
                    *c.get_mut(from).unwrap() = high;
                    let output = !c.values().all(|s| *s);
                    queue.push_back((to, output));
                }
            }
        }
        if i == 1000 {
            p1 = counts[0] * counts[1];
        }
    }
    /*
    println!("{highs:?}");
    for high in highs {
        assert!(high.len() >= 2);
        let period = high[1] - high[0];
        println!("{period}");
    }
    */
    // observed: periods are four primes around 4000, and trigger happens at last tick before period
    let p2:usize = highs.iter().map(|h| h[1] - h[0]).product::<usize>();
    (p1, p2)
}
