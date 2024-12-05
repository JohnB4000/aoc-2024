use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

use super::Day;

pub struct Day05;

impl Day for Day05 {
    type ParsedData = (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>);

    type Part1Result = i32;

    type Part2Result = i32;

    fn parse(input_directory: String) -> Result<Self::ParsedData, String> {
        let mut rules = HashMap::new();
        let mut updates = Vec::new();

        let lines: Vec<String> =
            BufReader::new(File::open(input_directory + "day05.txt").expect("File Error"))
                .lines()
                .map(|line| line.unwrap())
                .collect();

        let re = Regex::new(r"(\d+)\|(\d+)|(\d+(?:,\d+)*)").unwrap();
        lines.iter().for_each(|line| {
            re.captures_iter(line).for_each(|c| {
                if let Some(left) = c.get(1) {
                    rules
                        .entry(left.as_str().parse().unwrap())
                        .or_insert_with(Vec::new)
                        .push(c.get(2).unwrap().as_str().parse().unwrap());
                } else if let Some(entry) = c.get(3) {
                    updates.push(
                        entry
                            .as_str()
                            .split(',')
                            .map(|page| page.parse().unwrap())
                            .collect(),
                    )
                }
            })
        });

        Ok((rules, updates))
    }

    fn part_1(input: &Self::ParsedData) -> Self::Part1Result {
        let (rules, updates) = input;

        let mut sum = 0;

        'outer: for update in updates {
            for (l, r) in update.iter().zip(update.iter().skip(1)) {
                if rules.get(r).unwrap_or(&Vec::new()).contains(l) {
                    continue 'outer;
                }
            }
            sum += update[update.len() / 2];
        }

        sum
    }

    fn part_2(input: &Self::ParsedData) -> Self::Part2Result {
        let (rules, updates) = input;

        let mut sum = 0;

        let mut invalid_updates = Vec::new();

        'outer: for update in updates {
            for (l, r) in update.iter().zip(update.iter().skip(1)) {
                if rules.get(r).unwrap_or(&Vec::new()).contains(l) {
                    invalid_updates.push(update);
                    continue 'outer;
                }
            }
        }

        for update in invalid_updates {
            let mut new_order = Vec::new();
            for page in update {
                if new_order.is_empty() {
                    new_order.push(page);
                    continue;
                }

                let mut inserted = false;

                for i in 0..new_order.len() {
                    if !rules
                        .get(new_order[i])
                        .unwrap_or(&Vec::new())
                        .contains(page)
                    {
                        new_order.insert(i, page);
                        inserted = true;
                        break;
                    }
                }

                if !inserted {
                    new_order.push(page);
                }
            }

            sum += new_order[new_order.len() / 2];
        }

        sum
    }
}
