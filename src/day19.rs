use std::collections::HashMap;
use pathfinding::prelude::dfs_reach;
use regex::Regex;
use TestOperation::{GT, LT};
use crate::read_lines::read_lines;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum TestOperation {
    GT,
    LT
}

impl TestOperation {
    fn from_char(c: char) -> TestOperation {
        match c {
            '>' => GT,
            '<' => LT,
            _ => panic!("Invalid char passed for test operation")
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Rule {
    property: Option<char>,
    test_operation: Option<TestOperation>,
    test_value: Option<i32>,
    outcome: String
}

impl Rule {
    fn from_str(string: &str) -> Rule {
        let rule_re = Regex::new(r"^(?<property>[a-zAR]+)(?<operation>[<>])(?<test_value>[0-9]+):(?<outcome>[a-zAR]+)$").unwrap();
        let no_condition_rule_re = Regex::new(r"^(?<outcome>[a-zAR]+)$").unwrap();

        if let Some(rule_capture) = rule_re.captures(string) {
            Rule {
                property: Some(rule_capture.name("property").unwrap().as_str().chars().next().unwrap().clone()),
                test_operation: Some(TestOperation::from_char(rule_capture.name("operation").unwrap().as_str().chars().next().unwrap().clone())),
                test_value: Some(rule_capture.name("test_value").unwrap().as_str().parse().unwrap()),
                outcome: rule_capture.name("outcome").unwrap().as_str().to_string().clone()
            }
        } else if let Some(rule_capture) = no_condition_rule_re.captures(string) {
            Rule {
                property: None,
                test_operation: None,
                test_value: None,
                outcome: rule_capture.name("outcome").unwrap().as_str().to_string().clone()
            }
        } else {
            panic!("Unable to parse string to make rule");
        }
    }

    fn apply_rule(&self, item: &Item) -> Option<String> {
        if let Some(p) = self.property {
            let item_value = item.get_by_char(p);
            let test_value = self.test_value.unwrap();
            if match self.test_operation.unwrap() {
                GT => item_value > test_value,
                LT => item_value < test_value,
            } {
                return Some(String::from(self.outcome.as_str()));
            }
        } else {
            return Some(String::from(self.outcome.as_str()));
        }
        None
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Filter {
    rules: Vec<Rule>
}

#[derive(Debug)]
struct Item {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

impl Item {
    fn get_by_char(&self, c: char) -> i32 {
        match c {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!("Invalid get_by_char on Item")
        }
    }

    fn get_item_sum(&self) -> i32 {
        self.x + self.m + self.a + self.s
    }

    fn from_str(s: &str) -> Item {
        let mut new_item = Item{x:0,m:0,a:0,s:0};

        for property in s.split(',') {
            let (_, value_str) = property.split_at(2);
            let value = value_str.parse().unwrap();
            match property.chars().next().unwrap() {
                'x' => new_item.x = value,
                'm' => new_item.m = value,
                'a' => new_item.a = value,
                's' => new_item.s = value,
                _ => panic!("Found unexpected property")
            }
        }
        new_item
    }
}

fn parse_input(filename: &str) -> (HashMap<String, Vec<Rule>>, Vec<Item>) {
    let rules_re = Regex::new(r"^(?<rule_name>[a-z]+)\{(?<rules>.*)}$").unwrap();
    let items_re = Regex::new(r"^\{(?<properties>.*)}$").unwrap();
    let mut filters = HashMap::new();
    let mut items = vec![];

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if let Some(rule_captures) = rules_re.captures(ip.clone().as_str()) {
                    let mut rules = vec![];

                    for rule_str in rule_captures.name("rules").unwrap().as_str().split(',') {
                        rules.push(Rule::from_str(rule_str));
                    }

                    let rule_name = String::from(rule_captures.name("rule_name").unwrap().as_str());
                    filters.insert(rule_name, rules);
                } else if let Some(item_capture) = items_re.captures(ip.clone().as_str()) {
                    items.push(Item::from_str(item_capture.name("properties").unwrap().as_str()))
                }
            }
        }
    }
    else {
        println!("File not read");
    }

    (filters, items)
}

fn run_filter(filter: &Vec<Rule>, item: &Item) -> String {
    let mut filter_iter = filter.iter();
    let mut outcome = None;
    while outcome.is_none() {
        let rule = filter_iter.next().unwrap();
        outcome = rule.apply_rule(item);
    }
    return outcome.unwrap();
}

fn filter_items_sum_accepted(filters: &HashMap<String, Vec<Rule>>, items: Vec<Item>) -> i32 {
    let mut sum = 0;

    for item in items {
        let mut target_rule = String::from("in");
        while target_rule != "A" && target_rule != "R" {
            target_rule = run_filter(&filters[&target_rule], &item);
        }
        if target_rule == "A" {
            sum += item.get_item_sum();
        }
    }

    sum
}

fn next_nodes(n: &String, rules: &HashMap<String, Vec<Rule>>) -> Vec<String>
{
    let mut next_nodes: Vec<String> = vec![];

    if rules.contains_key(n) {
        for r in rules.get(n).unwrap() {
            let outcome_string = r.outcome.clone();
            next_nodes.push(outcome_string);
        }
    }
    next_nodes
}

fn find_accepting_paths(rules: &HashMap<String, Vec<Rule>>, entry_point: &str) -> Vec<Vec<Rule>> {
    let mut paths: Vec<Vec<Rule>> = vec![];
    let start_node = String::from(entry_point);
    //let traversed_paths: HashMap<String, >

    todo!();
    //return paths;
}

pub fn run() {
    println!("Day 19 Part A");
    let input_filename = "inputs/day19/test.txt";

    let (rules, items) = parse_input(input_filename);
    dbg!(filter_items_sum_accepted(&rules, items));

    let paths = find_accepting_paths(&rules, "in");
}