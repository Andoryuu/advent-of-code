use std::{collections::BTreeSet, fs};

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    let heightmap = get_heightmap(input);

    get_low_points(&heightmap)
        .iter()
        .map(|(_, h)| h + 1)
        .sum::<u32>()
        .to_string()
}

fn process_data_adv(input: String) -> String {
    let heightmap = get_heightmap(input);

    let mut basins: Vec<usize> = get_low_points(&heightmap)
        .iter()
        .map(|(i, _)| get_basin_size(*i, &heightmap))
        .collect();

    basins.sort_unstable();
    basins.reverse();

    basins
        .get(0..3)
        .unwrap()
        .iter()
        .product::<usize>()
        .to_string()
}

fn get_basin_size(position: usize, heightmap: &HeightMap) -> usize {
    let mut bas = BTreeSet::new();
    let mut to_do = Vec::<(usize, u32)>::new();

    bas.insert(position);
    to_do.push((position, *heightmap.values.get(position).unwrap()));

    while let Some((pos, height)) = to_do.pop() {
        let neigh = get_neighbors(pos, heightmap);

        for (i, v) in neigh.iter() {
            if !bas.contains(i) && *v != 9 && *v > height {
                bas.insert(*i);
                to_do.push((*i, *v));
            }
        }
    }

    bas.len()
}

fn get_low_points(heightmap: &HeightMap) -> Vec<(usize, u32)> {
    heightmap
        .values
        .iter()
        .enumerate()
        .filter(|(i, h)| get_neighbors(*i, heightmap).iter().all(|(_, v)| v > h))
        .map(|(i, &h)| (i, h))
        .collect()
}

fn get_heightmap(input: String) -> HeightMap {
    let size = input.lines().next().unwrap().len();
    let values: Vec<u32> = input
        .trim()
        .lines()
        .map(|l| l.trim())
        .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
        .collect();

    HeightMap {
        size,
        total: values.len(),
        values,
    }
}

fn get_neighbors(position: usize, heightmap: &HeightMap) -> Vec<(usize, u32)> {
    let mut res = Vec::with_capacity(4);
    let size = heightmap.size;

    if position % size != 0 {
        let i = position - 1;
        res.push((i, *heightmap.values.get(i).unwrap()));
    }

    if position % size != size - 1 {
        let i = position + 1;
        res.push((i, *heightmap.values.get(i).unwrap()));
    }

    if position >= size {
        let i = position - size;
        res.push((i, *heightmap.values.get(i).unwrap()));
    }

    if position < (heightmap.total - size) {
        let i = position + size;
        res.push((i, *heightmap.values.get(i).unwrap()));
    }

    res
}

struct HeightMap {
    size: usize,
    total: usize,
    values: Vec<u32>,
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_CASE: &str = "2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    ";

    #[test]
    fn base_check() {
        assert_eq!("15", process_data(TEST_CASE.to_string()));
    }

    #[test]
    fn adv_check() {
        assert_eq!("1134", process_data_adv(TEST_CASE.to_string()));
    }
}
