use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

use super::Day;

pub struct Day23;

impl Day for Day23 {
    type ParsedData = HashMap<String, Vec<String>>;

    type Part1Result = i32;

    type Part2Result = String;

    fn parse(input_directory: String) -> Result<Self::ParsedData, String> {
        let mut connections = HashMap::new();
        let re = Regex::new(r"(\w+)-(\w+)").unwrap();
        BufReader::new(File::open(input_directory + "day23test.txt").expect("File Error"))
            .lines()
            .for_each(|line| {
                re.captures_iter(&line.unwrap()).for_each(|c| {
                    let comp1 = c.get(1).unwrap().as_str().to_string();
                    let comp2 = c.get(2).unwrap().as_str().to_string();
                    connections
                        .entry(comp1.clone())
                        .or_insert_with(Vec::new)
                        .push(comp2.clone());
                    connections
                        .entry(comp2)
                        .or_insert_with(Vec::new)
                        .push(comp1);
                })
            });

        Ok(connections)
    }

    fn part_1(input: &Self::ParsedData) -> Self::Part1Result {
        let computers: Vec<&String> = input.keys().collect();
        let mut groups = HashSet::new();

        for computer in computers {
            let neighbours = input.get(computer).unwrap();

            for neighbour1 in neighbours {
                for neighbour2 in neighbours {
                    if neighbour1 == neighbour2 {
                        continue;
                    }

                    if input.get(neighbour1).unwrap().contains(neighbour2) {
                        let mut group =
                            vec![computer.as_str(), neighbour1.as_str(), neighbour2.as_str()];
                        group.sort();
                        groups.insert(group);
                    }
                }
            }
        }

        groups
            .iter()
            .filter(|computers| {
                computers
                    .iter()
                    .filter(|computer| computer.chars().next().unwrap() == 't')
                    .collect::<Vec<&&str>>()
                    .len()
                    > 0
            })
            .count() as i32
    }

    fn part_2(input: &Self::ParsedData) -> Self::Part2Result {
        let computers: Vec<&String> = input.keys().collect();

        let mut longest = 0;
        let mut current = Vec::new();

        for computer in computers {
            if let (_, Some(path)) = dfs(input, computer, HashSet::new()) {
                if path.len() > longest {
                    longest = path.len();
                    current = path;
                }
            }
        }

        String::new()
    }
}

fn dfs(
    input: &HashMap<String, Vec<String>>,
    computer: &str,
    visited: HashSet<&str>,
) -> (bool, Option<Vec<String>>) {
    if visited.contains(computer) {
        return (false, None);
    }

    (false, None)
}
