use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Display,
    hash::Hash,
};

use crate::day::Day;

pub struct Day08;

type Point = (u32, u32, u32);

#[derive(Clone, Eq, PartialEq)]
struct Connection {
    node_1: Point,
    node_2: Point,
}

impl Connection {
    fn dist_squared(&self) -> u64 {
        let (x_1, y_1, z_1) = self.node_1;
        let (x_2, y_2, z_2) = self.node_2;
        let d_x = (x_1 as i64 - x_2 as i64).pow(2);
        let d_y = (y_1 as i64 - y_2 as i64).pow(2);
        let d_z = (z_1 as i64 - z_2 as i64).pow(2);
        (d_x + d_y + d_z) as u64
    }
}

impl Ord for Connection {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist_squared().cmp(&self.dist_squared())
    }
}

impl PartialOrd for Connection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct UnionFind<T: Eq + Hash + Copy> {
    groups: HashMap<u32, Vec<T>>,
    group_for_element: HashMap<T, u32>,
    last_group_id: u32,
}

impl<T: Eq + Hash + Copy> UnionFind<T> {
    fn new() -> Self {
        UnionFind {
            groups: HashMap::new(),
            group_for_element: HashMap::new(),
            last_group_id: 0,
        }
    }

    fn merge_element_groups(&mut self, element_1: T, element_2: T) {
        let group_1_id = self.get_group_for_element(element_1);
        let group_2_id = self.get_group_for_element(element_2);
        self.merge_groups(group_1_id, group_2_id);
    }

    fn merge_groups(&mut self, group_1_id: u32, group_2_id: u32) {
        if group_1_id == group_2_id {
            return;
        }

        let mut group_2 = self.groups.remove(&group_2_id).unwrap();
        for &el in &group_2 {
            self.group_for_element.insert(el, group_1_id);
        }

        self.groups
            .get_mut(&group_1_id)
            .unwrap()
            .append(&mut group_2);
    }

    fn get_group_for_element(&mut self, el: T) -> u32 {
        self.group_for_element
            .entry(el)
            .or_insert_with(|| {
                self.last_group_id += 1;
                self.groups.insert(self.last_group_id, Vec::from([el]));
                self.last_group_id
            })
            .to_owned()
    }

    fn group_count(&self) -> usize {
        self.groups.len()
    }

    fn product_of_3_largest_group_sizes(&self) -> usize {
        let mut largest = 1;
        let mut second_largest = 1;
        let mut third_largest = 1;

        for group in self.groups.values() {
            if group.len() >= largest {
                third_largest = second_largest;
                second_largest = largest;
                largest = group.len();
            } else if group.len() >= second_largest {
                third_largest = second_largest;
                second_largest = group.len();
            } else if group.len() > third_largest {
                third_largest = group.len();
            }
        }

        largest * second_largest * third_largest
    }
}

pub struct Day08Part1Result {
    result: usize,
    heap: BinaryHeap<Connection>,
    union_find: UnionFind<Point>,
    unvisited_nodes: HashSet<Point>,
}

impl Display for Day08Part1Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.result.fmt(f)
    }
}

impl Day for Day08 {
    type Input = Vec<Point>;
    type Output1 = Day08Part1Result;
    type Output2 = u64;

    fn id() -> String {
        "08".to_string()
    }

    fn parse_input(input: &str) -> Self::Input {
        input
            .lines()
            .map(|l| l.split(",").map(|r| r.parse().unwrap()))
            .map(|mut it| (it.next().unwrap(), it.next().unwrap(), it.next().unwrap()))
            .collect()
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut unvisited_nodes = HashSet::from_iter(input.iter().cloned());
        let mut heap = BinaryHeap::new();

        for i in 0..input.len() - 1 {
            for j in i + 1..input.len() {
                let node_1 = input[i];
                let node_2 = input[j];
                heap.push(Connection { node_1, node_2 });
            }
        }

        let mut union_find = UnionFind::new();

        for _ in 0..1000 {
            let Connection { node_1, node_2 } = heap.pop().unwrap();
            unvisited_nodes.remove(&node_1);
            unvisited_nodes.remove(&node_2);
            union_find.merge_element_groups(node_1, node_2);
        }

        Day08Part1Result {
            result: union_find.product_of_3_largest_group_sizes(),
            heap,
            union_find,
            unvisited_nodes,
        }
    }

    fn part_2(_: &Self::Input, part_1_output: Self::Output1) -> Self::Output2 {
        let Day08Part1Result {
            mut unvisited_nodes,
            mut heap,
            mut union_find,
            ..
        } = part_1_output;

        loop {
            let Connection { node_1, node_2 } = heap.pop().unwrap();
            unvisited_nodes.remove(&node_1);
            unvisited_nodes.remove(&node_2);
            union_find.merge_element_groups(node_1, node_2);
            if unvisited_nodes.is_empty() && union_find.group_count() == 1 {
                return node_1.0 as u64 * node_2.0 as u64;
            }
        }
    }
}
