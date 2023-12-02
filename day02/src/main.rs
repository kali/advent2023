use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    let games: Vec<(usize, Vec<(usize, usize, usize)>)> = input
        .lines()
        .map(|line| {
            let (id, sets) = line.split_once(": ").unwrap();
            let id = id.trim_start_matches("Game ").parse::<usize>().unwrap();
            let sets = sets
                .split("; ")
                .map(|set| {
                    let (mut r, mut g, mut b) = (0, 0, 0);
                    for pick in set.split(", ") {
                        let (n, color) = pick.split_once(' ').unwrap();
                        let n = n.parse::<usize>().unwrap();
                        match color {
                            "red" => r = n,
                            "green" => g = n,
                            "blue" => b = n,
                            _ => panic!(),
                        }
                    }
                    (r, g, b)
                })
                .collect_vec();
            (id, sets)
        })
        .collect();

    let p1: usize = games
        .iter()
        .filter(|game| {
            game.1
                .iter()
                .all(|&(r, g, b)| r <= 12 && g <= 13 && b <= 14)
        })
        .map(|game| game.0)
        .sum();
    dbg!(p1);

    let p2 = games
        .iter()
        .map(|game| {
            let r = game.1.iter().map(|p| p.0).max().unwrap();
            let g = game.1.iter().map(|p| p.1).max().unwrap();
            let b = game.1.iter().map(|p| p.2).max().unwrap();
            r * g * b
        })
        .sum::<usize>();
    dbg!(p2);

    Ok(())
}
