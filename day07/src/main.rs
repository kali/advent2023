use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    dbg!(play(&input, false));
    dbg!(play(&input, true));
    Ok(())
}

fn typ(hand: &str) -> usize {
    let grouping = hand
        .chars()
        .sorted()
        .group_by(|c| *c)
        .into_iter()
        .map(|g| g.1.count())
        .filter(|n| *n > 1)
        .sorted()
        .collect_vec();
    match &*grouping {
        &[5] => 6,
        &[4] => 5,
        &[2, 3] => 4,
        &[3] => 3,
        &[2, 2] => 2,
        &[2] => 1,
        _ => 0,
    }
}

fn optimise(input: &str) -> usize {
    let js = input.chars().filter(|c| *c == 'J').count();
    let others = input
        .chars()
        .filter(|c| *c != 'J')
        .sorted()
        .unique()
        .collect_vec();
    let to_try = others.len().pow(js as _);
    let mut max = typ(input);
    for mut ix in 0..to_try {
        let mut hand = input.to_string();
        for _ in 0..js {
            let repl = others[ix % others.len()];
            hand = hand.replacen('J', &repl.to_string(), 1);
            ix /= others.len();
        }
        max = max.max(typ(&hand));
    }
    max
}

fn play(input: &str, rules_2: bool) -> usize {
    let order = if rules_2 {
        "AKQT98765432J"
    } else {
        "AKQJT98765432"
    };
    input
        .lines()
        .map(|line| {
            let (hand, bet) = line.split_once(" ").unwrap();
            let sortable = hand
                .chars()
                .map(|c| order.chars().rev().position(|x| x == c).unwrap())
                .collect_vec();
            let t = if rules_2 { optimise(hand) } else { typ(hand) };
            (t, sortable, hand, bet.parse::<usize>().unwrap())
        })
        .sorted()
        .enumerate()
        .map(|(ix, (__, _, _, bet))| (ix + 1) * bet)
        .sum::<usize>()
}

#[test]
fn t() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    assert_eq!(play(&input, false), 6440);
    assert_eq!(play(&input, true), 5905);
}

#[test]
fn opt() {
    assert_eq!(optimise("8822J"), 4);
}
