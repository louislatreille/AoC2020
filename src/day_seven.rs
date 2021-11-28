use std::collections::{HashSet, LinkedList};
use std::fmt;
use std::io::{prelude::*, BufReader};
use std::fs::File;
use regex::Regex;
use lazy_static::lazy_static;

pub fn entry() {
	println!("Starting challenges for day seven!");

	let bag_rules = read_bag_rules("./resources/day_seven_input.txt");
    println!("{}", bag_rules.len());

    let mut bag_nodes = LinkedList::new();
    for bag_rule in bag_rules {
        let bag_node = extract_bag_node(&bag_rule);
        //println!("{}", bag_node);
        bag_nodes.push_back(bag_node);
    }

    let shiny_gold_bag = BagType {
        adj: String::from("shiny"),
        color: String::from("gold"),
    };

    let can_eventually_contain_shiny_gold = find_bag_parents(&shiny_gold_bag, &bag_nodes.clone().into_iter().collect());
    println!("{}", can_eventually_contain_shiny_gold.len());
    
    let shiny_gold_bag_children = find_bags_inside(&shiny_gold_bag, &bag_nodes.clone().into_iter().collect(), &mut 1);
    println!("{}", shiny_gold_bag_children - 1);

    /*let mut top_nodes = build_tree(bag_nodes.clone());
    println!("{}", top_nodes.len());*/

    /*let mut current_length = 0;
    let mut num_iter = 0;
    while current_length != top_nodes.len() && top_nodes.len() != 1 && num_iter != 500 {
        println!("Top nodes length: {}", top_nodes.len());
        current_length = top_nodes.len();
        top_nodes = build_tree(top_nodes);
        num_iter += 1;
    }

    println!("{}", top_nodes.len());
    let mut top_node = top_nodes.pop_front().unwrap();
    println!("Top node: {}", top_node);*/

    /*let shiny_gold_bag = BagType {
        adj: String::from("shiny"),
        color: String::from("gold"),
    };

    let mut num = 0;

    for mut top_node in top_nodes.clone() {
        let all_shiny_gold_nodes = top_node.find_contain_level(&shiny_gold_bag, &mut num);
    }

    println!("Shiny gold bag found: {}", num);

    let mut can_contains: HashSet<BagType> = HashSet::new();

    for mut top_node in top_nodes.clone() {
        let all_shiny_gold_nodes = top_node.find_can_contain(&shiny_gold_bag, &mut can_contains);
    }

    println!("Shiny gold bag found: {}", can_contains.len());*/
}

fn read_bag_rules(filename: &str) -> Vec<String> {
	let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

	let mut bag_rules = vec!();
    for line in reader.lines() {
        let line = line.unwrap();
        bag_rules.push(line);
    }

    bag_rules
}

fn find_bag_parents(bag: &BagType, all_bag_nodes: &Vec<BagNode>) -> HashSet<BagType> {
    let mut to_return: HashSet<BagType> = HashSet::new();
    let mut to_find: HashSet<BagType> = HashSet::new();
    let mut found: HashSet<BagType> = HashSet::new();
    to_find.insert(bag.clone());

    loop {
        for bag_node in all_bag_nodes.clone() {
            for bag_to_find in to_find.clone() {
                if bag_node.can_contain(&bag_to_find) {
                    found.insert(bag_node.bag_type.clone());
                }
            }
        }

        to_return.extend(found.clone());
        to_find = found.clone();

        println!("Found {} parent bags", found.len());
        if found.len() == 0 {
            break;
        }

        found.clear();
    }
    
    to_return
}

fn find_bags_inside(bag: &BagType, all_bag_nodes: &Vec<BagNode>, previous_amount: &usize) -> usize {
    let to_find = BagNode {
        number: *previous_amount,
        bag_type: bag.clone(),
        can_contain: vec!()
    };

    let mut amount = *previous_amount;
    for bag_node in all_bag_nodes {
        if bag_node.bag_type.eq(&to_find.bag_type) && !bag_node.is_empty() {
            for found_bag in bag_node.can_contain.clone() {
                println!("{} * {}", to_find.number, found_bag.number);
                amount += find_bags_inside(&found_bag.bag_type, all_bag_nodes, &mut (to_find.number * found_bag.number));
            }
        }
    }

    println!("Total bags now at {}", amount);

    amount
}

fn build_tree(mut bag_nodes: LinkedList<BagNode>) -> LinkedList<BagNode> {
    let mut current_top_node = bag_nodes.pop_front().unwrap();
    let mut top_nodes: LinkedList<BagNode> = LinkedList::new();

    let mut misses = 0;
    while !bag_nodes.is_empty() {
        //println!("Length: {}", bag_nodes.len());
        let mut current_node = match bag_nodes.pop_front() {
            Some(node) => node,
            None => continue,
        };

        if bag_nodes.len() == misses {
            //println!("Went through the whole set twice, trying to reconcile top nodes");
            //println!("Top node: {}", current_top_node);
            top_nodes.push_back(current_top_node);

            
            //println!("Current node: {}", current_node);
            //println!("Creating new top node!");
            //println!("Length: {}", bag_nodes.len());
            
            current_top_node = current_node;
            misses = 0;
            continue;
        }

        let matches = current_top_node.find_all(&current_node.bag_type);
        let found_some = !matches.is_empty();
        for matchh in matches {
            if !matchh.is_empty() {
                panic!("Not expected")
            }

            *matchh = current_node.clone();
        }

        if found_some {
            misses = 0;
            continue;
        }

        let matches = current_node.find_all(&current_top_node.bag_type);
        if !matches.is_empty() {
            misses = 0;
            //println!("Found {} by switching top node", matches.len());
        } else {
            bag_nodes.push_back(current_node);
            misses += 1;
            continue;
        }
        
        for matchh in matches {
            if !matchh.is_empty() {
                panic!("Not expected")
            }

            *matchh = current_top_node.clone();
        }

        current_top_node = current_node;
    }

    top_nodes
}

fn build_tree_other(mut bag_nodes: LinkedList<BagNode>) -> LinkedList<BagNode> {
    let mut current_top_node = bag_nodes.pop_front().unwrap();
    let mut top_nodes: LinkedList<BagNode> = LinkedList::new();

    let mut misses = 0;
    while !bag_nodes.is_empty() {
        let mut current_node = match bag_nodes.pop_front() {
            Some(node) => node,
            None => continue,
        };

        if bag_nodes.len() == misses {
            //println!("Went through the whole set twice, trying to reconcile top nodes");
            //println!("Top node: {}", current_top_node);
            top_nodes.push_back(current_top_node);

            
            //println!("Current node: {}", current_node);
            //println!("Creating new top node!");
            //println!("Length: {}", bag_nodes.len());
            
            current_top_node = current_node;
            misses = 0;
            continue;
        }

        match current_top_node.find(&current_node.bag_type) {
            Some(node) => {
                if !node.is_empty() {
                    panic!("Not expected")
                }

                //println!("Found node! {}", current_node);
                *node = current_node;
                misses = 0;
                continue;
            },
            None => (),
        }
        match current_node.find(&current_top_node.bag_type) {
            Some(node) => {
                if !node.is_empty() {
                    panic!("Not expected")
                }

                //println!("Found top node into current node! {}", current_top_node);
                *node = current_top_node;
                current_top_node = current_node;
                misses = 0;
            },
            None => {
                bag_nodes.push_back(current_node);
                misses += 1;
            },
        }
    }

    top_nodes
}

lazy_static! {
    static ref RE_BAG: Regex = Regex::new(r"(?P<adj>\w+)\s(?P<color>\w+)\sbag[s]?").unwrap();
    static ref RE_BAG_NUMBER: Regex = Regex::new(r"(?P<num>\d+)\s(?P<bag>[\w\s]+)").unwrap();
}

#[derive(Hash)]
struct BagType {
    adj: String,
    color: String,
}

impl fmt::Display for BagType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} bag", self.adj, self.color)
    }
}

impl PartialEq for BagType {
    fn eq(&self, other: &Self) -> bool {
        self.adj == other.adj && self.color == other.color
    }
}

impl Eq for BagType {}

impl Clone for BagType {
    fn clone(&self) -> Self {
        Self { adj: self.adj.clone(), color: self.color.clone() }
    }
}

impl BagType {
    fn new(str: &str) -> BagType {
        

        let caps = RE_BAG.captures(str).unwrap();

        BagType {
            adj: caps["adj"].to_owned(),
            color: caps["color"].to_owned()
        }
    }
}

struct BagNode {
    number: usize,
    bag_type: BagType,
    can_contain: Vec<BagNode>,
}

impl fmt::Display for BagNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut children = "".to_owned();

        match self.is_empty() {
            true => children += "none",
            false => {
                for bag_node in &self.can_contain {
                    children += &format!("{}", bag_node).to_owned();
                    children += ", ";
                }
            }
        }

        write!(f, "{} {} contain {}", self.number, self.bag_type, children)
    }
}

impl Clone for BagNode {
    fn clone(&self) -> Self {
        Self { number: self.number.clone(), bag_type: self.bag_type.clone(), can_contain: self.can_contain.clone() }
    }
}

impl BagNode {
    fn is(&self, other: &BagType) -> bool {
        return self.bag_type.eq(other);
    }

    fn can_contain(&self, other: &BagType) -> bool {
        let mut to_return = false;
        for bag in self.can_contain.clone() {
            to_return = to_return || bag.bag_type.eq(other);
        }
        to_return
    }

    fn is_empty(&self) -> bool {
        self.can_contain.is_empty()
    }
    
    fn find(&mut self, bag_type: &BagType) -> Option<&mut BagNode> {
        if self.bag_type.eq(bag_type) {
            return Some(self);
        } else if self.is_empty() {
            return None;
        } else {
            for child in &mut self.can_contain {
                match child.find(bag_type) {
                    Some(matched) => return Some(matched),
                    None => continue,
                }

            }
        }

        None
    }

    fn find_all(&mut self, bag_type: &BagType) -> Vec<&mut BagNode> {
        let mut to_return = vec!();
        if self.bag_type.eq(bag_type) {
            to_return.push(self);
        } else if self.is_empty() {
            return to_return;
        } else {
            for child in &mut self.can_contain {
                let mut matched = child.find_all(bag_type);
                to_return.append(&mut matched);
            }
        }

        to_return
    }

    fn find_contain_level<'a>(&mut self, bag_type: &BagType, inc: &'a mut i32) -> bool {
        if self.bag_type.eq(bag_type) {
            println!("{}", self);
            return true;
        } else if self.is_empty() {
            return false;
        } else {
            let mut initial = false;
            for child in &mut self.can_contain {
                initial = initial || child.find_contain_level(bag_type, inc);
            }

            if initial {
                println!("{}", self.bag_type);
                *inc += 1;
            }

            initial
        }
    }

    fn find_can_contain<'a>(&mut self, bag_type: &BagType, inc: &'a mut HashSet<BagType>) -> bool {
        if self.bag_type.eq(bag_type) {
            return true;
        } else if self.is_empty() {
            return false;
        } else {
            let mut can_contain = false;
            for child in &mut self.can_contain {
                can_contain = can_contain || child.find_can_contain(bag_type, inc);
            }

            if can_contain {
                inc.insert(self.bag_type.clone());
            }

            can_contain
        }
    }
}

struct NavigableBagNode<'a> {
    node: &'a BagNode,
    parent: Option<&'a NavigableBagNode<'a>>,
}

impl<'a> NavigableBagNode<'a> {
    fn child(&self, index: usize) -> NavigableBagNode {
        NavigableBagNode {
            node: &self.node.can_contain[index],
            parent: Some(self)
        }
    }
}

fn extract_bag_node(str: &str) -> BagNode {
    let split_contain: Vec<&str> = str.split("contain").map(|s| s.trim()).collect();
    let split_commas: Vec<&str> = split_contain.get(1).unwrap().split(',').map(|s| s.trim()).collect();

    let bag = BagType::new(split_contain.get(0).unwrap());
    let mut child_bags: Vec<BagNode> = vec!();
    

    for bag_number in split_commas {
        match RE_BAG_NUMBER.captures(bag_number) {
            Some(caps) => {
                child_bags.push(BagNode {
                    number: caps["num"].parse::<usize>().unwrap(),
                    bag_type: BagType::new(&caps["bag"]),
                    can_contain: vec!()
                })
            },
            None => continue,
        };
    }

    BagNode {
        number: 1,
        bag_type: bag,
        can_contain: child_bags
    }
}