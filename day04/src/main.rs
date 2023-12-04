use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let mut p1 = 0;
    let mut deck = vec![1; input.lines().count()];
    for (id, line) in input.lines().enumerate() {
        let (_, tail) = line.split_once(": ").unwrap();
        let (win, have) = tail.split_once(" | ").unwrap();
        let win = win.split_whitespace().collect_vec();
        let have = have.split_whitespace().collect_vec();
        let count = have.iter().filter(|x| win.contains(x)).count();
        if count > 0 {
            p1 += 2usize.pow(count as u32 - 1);
        }
        for i in 1..=count {
            deck[id + i] += deck[id]
        }
    }
    dbg!(p1);
    let p2 = deck.iter().sum::<usize>();
    dbg!(p2);
    Ok(())
}
