use std::fs;
use std::collections::HashMap;

struct TreeHashMap {
    map: HashMap<(i32, i32), bool>,
    rows: i32,
    cols: i32,
}

fn make_hashmap(contents: &str) -> TreeHashMap {
    let mut map: HashMap<(i32, i32), bool> = HashMap::new();
    let mut row = 0;
    let mut cols = 0;
    for line in contents.lines() {
        let mut col = 0;
        for cur in line.chars() {
            map.insert((row, col), cur == '.');
            col += 1;
        }
        row += 1;
        cols = col;
    }

    TreeHashMap {
        map: map,
        rows: row,
        cols: cols,
    }
}

fn get_num_trees(tree_map: &TreeHashMap, right: i32, down: i32) -> i32 {
    let map = &tree_map.map;
    let rows = tree_map.rows;
    let cols = tree_map.cols;

    let mut x = 0;
    let mut y = 0;
    let mut num_trees = 0;
    loop {
        x += right;
        y += down;
        if y >= rows {
            break;
        }
        let is_free = map.get(&(y % rows, x % cols)).unwrap();
        if !*is_free {
            num_trees += 1;
        }
        // println!("({}, {}) = {}", x, y, if *is_free { '.' } else { '#' });
    }

    num_trees
}

#[allow(dead_code)]
fn day3(contents: &str) {
    let tree_map = make_hashmap(&contents);
    let num_trees = get_num_trees(&tree_map, 3, 1);
    println!("number of trees: {}", num_trees);
}

#[allow(dead_code)]
fn day3x(contents: &str) {
    let tree_map = make_hashmap(&contents);
    let slopes: [(i32, i32); 5] = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];
    let result = slopes
        .iter()
        .map(|(right, down)| get_num_trees(&tree_map, *right, *down))
        .fold(1 as u64, |accu, elem| accu * (elem as u64));
    println!("result: {:?}", result);
}

fn main() {
    let contents = fs::read_to_string("day3.txt")
        .expect("something went wrong reading day3.txt");

    // day3(&contents);
    day3x(&contents);
}
