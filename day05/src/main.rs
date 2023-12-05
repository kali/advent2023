use itertools::Itertools;

#[derive(Debug)]
struct Map(Vec<(usize, isize)>);

impl Map {
    fn from_triples(triples: Vec<(usize, usize, usize)>) -> Map {
        let mut it: Vec<(usize, isize)> = vec![];
        for &(dst, src, _) in &triples {
            it.push((src, dst as isize - src as isize));
        }
        for &(_, src, len) in &triples {
            if !it.iter().any(|pair| pair.0 == src + len) {
                it.push((src + len, 0));
            }
        }
        it.sort();
        if it[0].0 != 0 {
            it.insert(0, (0, 0));
        }
        Map(it)
    }

    fn locate(&self, x: usize) -> usize {
        self.0
            .binary_search_by_key(&x, |b| b.0)
            .unwrap_or_else(|e| e - 1)
    }

    fn tr(&self, x: usize) -> usize {
        let range = self.locate(x);
        (x as isize + self.0.get(range).map(|p| p.1).unwrap_or(0)) as usize
    }

    fn compose(&self, rhs: &Map) -> Map {
        let mut current = 0;
        let mut result: Vec<(usize, isize)> = vec![];
        loop {
            let prime = self.tr(current);
            result.push((current, (rhs.tr(prime) as isize - current as isize)));
            let locate_left = self.locate(current);
            let len_left = self.0.get(locate_left + 1).map(|x| x.0 - current);
            let locate_right = rhs.locate(prime);
            let len_right = rhs.0.get(locate_right + 1).map(|x| x.0 - prime);
            let len = [len_left, len_right].into_iter().flat_map(|x| x).min();
            if let Some(len) = len {
                current += len;
            } else {
                break;
            }
        }
        Map(result)
    }
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    dbg!(run(&input).unwrap());
    Ok(())
}

fn run(input: &str) -> anyhow::Result<(usize, usize)> {
    let mut input = input.lines();
    let seeds = input
        .next()
        .unwrap()
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect_vec();
    input.next();
    let mut maps = vec![];
    let mut map = vec![];
    for line in input {
        if line == "" {
            ()
        } else if line.contains("map") {
            if map.len() > 0 {
                maps.push(Map::from_triples(map.clone()));
                map.clear();
            }
        } else {
            map.push(
                line.split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect_tuple()
                    .unwrap(),
            );
        }
    }
    maps.push(Map::from_triples(map));
    let p1 = seeds
        .iter()
        .map(|s| maps.iter().fold(*s, |x, map| map.tr(x)))
        .min()
        .unwrap();
    let global = maps.iter().fold(Map(vec![(0, 0)]), |g, i| g.compose(i));
    let p2 = seeds
        .iter()
        .chunks(2)
        .into_iter()
        .map(|mut pair| {
            let start = *pair.next().unwrap();
            let len = *pair.next().unwrap();
            let left = global.tr(start);
            global
                .0
                .iter()
                .skip(global.locate(start))
                .take_while(|pair| pair.0 < start + len)
                .map(|pair| (pair.0 as isize + pair.1) as usize)
                .min()
                .unwrap()
                .min(left)
        })
        .min()
        .unwrap();
    Ok((p1, p2))
}

#[test]
fn t() {
    let r = run("seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
        .trim())
    .unwrap();
    assert_eq!(r, (35, 46));
}
