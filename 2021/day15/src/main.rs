mod dedup_queue;

use std::{collections::BTreeMap, fs};

use dedup_queue::DedupQueue;

// TODO: use 'BinaryHeap' instead of custom collection?
// TODO: use Dijkstra algorithm instead of made up solution?
fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    let (cavern, sizing) = parse(input);

    find_shortest_path_len(cavern, sizing).to_string()
}

fn process_data_adv(input: String) -> String {
    let (base_cavern, base_sizing) = parse(input);
    let (cavern, sizing) = inflate_5(base_cavern, base_sizing);

    find_shortest_path_len(cavern, sizing).to_string()
}

fn find_shortest_path_len(cavern: Vec<u32>, sizing: CavernSize) -> u32 {
    let mut shortests = BTreeMap::from([(0usize, 0u32)]);

    let mut queue = DedupQueue::new();
    queue.push_front(0usize);

    while let Some(next) = queue.pop() {
        let path = shortests.get(&next).copied().unwrap();
        let neighs = get_neighbor_positions(next, &sizing);

        for &neigh in neighs.iter() {
            let neigh_path = path + cavern.get(neigh).unwrap();

            if let Some(existing) = shortests.get_mut(&neigh) {
                if *existing > neigh_path {
                    *existing = neigh_path;
                    queue.push_front(neigh);
                }
            } else {
                shortests.insert(neigh, neigh_path);
                queue.push_back(neigh);
            }
        }
    }

    shortests.get(&(sizing.area - 1)).copied().unwrap()
}

fn inflate_5(cavern: Vec<u32>, sizing: CavernSize) -> (Vec<u32>, CavernSize) {
    let horizontal: Vec<u32> = cavern[..]
        .chunks(sizing.dimension)
        .flat_map(|chunk| (0..5).flat_map(|i| chunk.iter().map(move |c| wrap_9(c + i))))
        .collect();

    let vertical = (0..5)
        .flat_map(|i| horizontal.iter().map(move |c| wrap_9(c + i)))
        .collect();

    let new_dim = sizing.dimension * 5;

    (
        vertical,
        CavernSize {
            dimension: new_dim,
            area: new_dim * new_dim,
        },
    )
}

fn wrap_9(val: u32) -> u32 {
    if val > 9 {
        val - 9
    } else {
        val
    }
}

fn parse(input: String) -> (Vec<u32>, CavernSize) {
    let &dimension = input
        .trim()
        .lines()
        .take(1)
        .map(|l| l.len())
        .collect::<Vec<usize>>()
        .first()
        .unwrap();

    let size = CavernSize {
        dimension,
        area: dimension * dimension,
    };

    let cavern = input
        .trim()
        .lines()
        .flat_map(|l| l.trim().chars())
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    (cavern, size)
}

fn get_neighbor_positions(position: usize, cavern: &CavernSize) -> Vec<usize> {
    let mut neighs = Vec::with_capacity(4);

    if position >= cavern.dimension {
        neighs.push(position - cavern.dimension);
    }

    if position % cavern.dimension != 0 {
        neighs.push(position - 1);
    }

    if position % cavern.dimension != (cavern.dimension - 1) {
        neighs.push(position + 1);
    }

    if position < cavern.area - cavern.dimension {
        neighs.push(position + cavern.dimension);
    }

    neighs
}

struct CavernSize {
    dimension: usize,
    area: usize,
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_CASE: &str = "1163751742
    1381373672
    2136511328
    3694931569
    7463417111
    1319128137
    1359912421
    3125421639
    1293138521
    2311944581
    ";

    #[test]
    fn base_check() {
        assert_eq!("40", process_data(TEST_CASE.to_string()));
    }

    #[test]
    fn adv_check() {
        assert_eq!("315", process_data_adv(TEST_CASE.to_string()));
    }
}
