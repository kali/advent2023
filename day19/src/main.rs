use std::collections::HashMap;
use std::ops::Range;

use itertools::Itertools;

// xmas, is_greater_than, limit, dest
type Filter = (usize, bool, usize, String);

fn accepted(workflow: &HashMap<String, Vec<Filter>>, item: &[usize]) -> bool {
    let mut state = "in";
    while state != "R" && state != "A" {
        for rule in &workflow[state] {
            if (rule.1 && item[rule.0] > rule.2) || (!rule.1 && item[rule.0] < rule.2) {
                state = &rule.3;
                break;
            }
        }
    }
    state == "A"
}

fn rec_p2(
    workflow: &HashMap<String, Vec<Filter>>,
    state: &str,
    rule: usize,
    ranges: &[Range<usize>],
) -> usize {
    if state == "A" {
        return ranges.iter().map(|r| r.end - r.start).product::<usize>();
    } else if state == "R" {
        return 0;
    };
    let next_rule = &workflow[state][rule];
    if next_rule.2 == usize::MAX {
        rec_p2(workflow, &next_rule.3, 0, ranges)
    } else {
        if next_rule.1 {
            let mut left = ranges.to_vec();
            left[next_rule.0].end = next_rule.2 + 1;
            let left = rec_p2(workflow, state, rule + 1, &left);
            let mut right = ranges.to_vec();
            right[next_rule.0].start = next_rule.2 + 1;
            let right = rec_p2(workflow, &next_rule.3, 0, &right);
            left + right
        } else {
            let mut left = ranges.to_vec();
            left[next_rule.0].end = next_rule.2;
            let left = rec_p2(workflow, &next_rule.3, 0, &left);
            let mut right = ranges.to_vec();
            right[next_rule.0].start = next_rule.2;
            let right = rec_p2(workflow, state, rule + 1, &right);
            left + right
        }
    }
}

fn run(input: &str) -> (usize, usize) {
    let workflow: HashMap<String, Vec<Filter>> = input
        .lines()
        .take_while(|l| l.len() > 0)
        .map(|line| {
            let (name, rules) = line.split_once("{").unwrap();
            let filters = rules
                .trim_end_matches("}")
                .split(",")
                .map(|rule| {
                    if let Some((cond, dst)) = rule.split_once(":") {
                        let xmas = cond.bytes().next().unwrap();
                        let is_greater_than = cond.bytes().nth(1).unwrap() == b'>';
                        let lim = cond.chars().skip(2).join("").parse::<usize>().unwrap();
                        (
                            "xmas".bytes().position(|t| t == xmas).unwrap(),
                            is_greater_than,
                            lim,
                            dst.to_string(),
                        )
                    } else {
                        (0, false, usize::MAX, rule.to_string())
                    }
                })
                .collect_vec();
            (name.to_string(), filters)
        })
        .collect();

    let mut p1 = 0;
    for item in input.lines().skip_while(|l| l.len() > 0).skip(1) {
        let ratings = item
            .replace(['x', 'm', 'a', 's', '{', '}', '=', ','], " ")
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect_vec();
        if accepted(&workflow, &ratings) {
            p1 += ratings.iter().sum::<usize>();
        }
    }
    let p2 = rec_p2(&workflow, "in", 0, &vec![1..4001; 4]);
    (p1, p2)
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap().trim().to_string();
    dbg!(run(&input));
}

#[test]
fn t() {
    let input = r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
    assert_eq!(run(input), (19114, 167409079868000));
}
