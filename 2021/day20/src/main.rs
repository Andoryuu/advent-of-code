use std::fs;

fn main() {
    let input = fs::read_to_string("./_data/input.txt").expect("oh noes");

    let output = process_data(input.clone());
    let adv_output = process_data_adv(input);

    println!("Result is: {}", output);
    println!("Adv result is: {}", adv_output);
}

fn process_data(input: String) -> String {
    let (enhancer, base_image) = parse(input);

    let enhanced = (0..2).fold(base_image, |img, _| step(&enhancer, img));

    enhanced.values.iter().filter(|b| **b).count().to_string()
}

fn process_data_adv(input: String) -> String {
    let (enhancer, base_image) = parse(input);

    let enhanced = (0..50).fold(base_image, |img, _| step(&enhancer, img));

    enhanced.values.iter().filter(|b| **b).count().to_string()
}

fn step(enhancer: &[bool], source: Image) -> Image {
    let conv_dim = source.dimension as isize;

    let new_values = (-1..=conv_dim)
        .flat_map(|y| (-1..=conv_dim).map(move |x| (x, y)))
        .map(|(x, y)| *enhancer.get(get_position_value(x, y, &source)).unwrap())
        .collect();

    let new_unknown = if source.unknown_values {
        enhancer.last().unwrap()
    } else {
        enhancer.first().unwrap()
    };

    Image {
        dimension: source.dimension + 2,
        unknown_values: *new_unknown,
        values: new_values,
    }
}

fn parse(input: String) -> (Vec<bool>, Image) {
    let enhancer: Vec<bool> = input
        .lines()
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(|c| c == '#')
        .collect();

    let dimension = input.lines().nth(2).unwrap().trim().len();

    let values = input
        .trim()
        .lines()
        .skip(2)
        .flat_map(|l| l.trim().chars())
        .map(|c| c == '#')
        .collect();

    (
        enhancer,
        Image {
            dimension,
            unknown_values: false,
            values,
        },
    )
}

fn get_position_value(position_x: isize, position_y: isize, reference: &Image) -> usize {
    let dimension = reference.dimension as isize;
    let size = dimension * dimension;
    let unknown = reference.unknown_values;

    let above = position_y - 1;
    let left = position_x - 1;
    let below = position_y + 1;
    let right = position_x + 1;

    let is_too_above = above < 0;
    let is_too_left = left < 0;
    let is_outside_x = position_x < 0 || position_x >= dimension;
    let is_too_right = right >= dimension;
    let is_outside_y = position_y < 0 || (position_y * dimension) >= size;
    let is_too_below = (below * dimension) >= size;

    let st_st = if is_too_above || is_too_left {
        unknown
    } else {
        let index = (left + (above * dimension)) as usize;
        *reference.values.get(index).unwrap()
    };

    let st_nd = if is_too_above || is_outside_x {
        unknown
    } else {
        let index = (position_x + (above * dimension)) as usize;
        *reference.values.get(index).unwrap()
    };

    let st_rd = if is_too_above || is_too_right {
        unknown
    } else {
        let index = (right + (above * dimension)) as usize;
        *reference.values.get(index).unwrap()
    };

    let nd_st = if is_outside_y || is_too_left {
        unknown
    } else {
        let index = (left + (position_y * dimension)) as usize;
        *reference.values.get(index).unwrap()
    };

    let nd_nd = if is_outside_y || is_outside_x {
        unknown
    } else {
        let index = (position_x + (position_y * dimension)) as usize;
        *reference.values.get(index).unwrap()
    };

    let nd_rd = if is_outside_y || is_too_right {
        unknown
    } else {
        let index = (right + (position_y * dimension)) as usize;
        *reference.values.get(index).unwrap()
    };

    let rd_st = if is_too_below || is_too_left {
        unknown
    } else {
        let index = (left + (below * dimension)) as usize;
        *reference.values.get(index).unwrap()
    };

    let rd_nd = if is_too_below || is_outside_x {
        unknown
    } else {
        let index = (position_x + (below * dimension)) as usize;
        *reference.values.get(index).unwrap()
    };

    let rd_rd = if is_too_below || is_too_right {
        unknown
    } else {
        let index = (right + (below * dimension)) as usize;
        *reference.values.get(index).unwrap()
    };

    let b_str: String = [
        st_st, st_nd, st_rd, nd_st, nd_nd, nd_rd, rd_st, rd_nd, rd_rd,
    ]
    .map(|b| if b { '1' } else { '0' })
    .iter()
    .collect();

    usize::from_str_radix(&b_str, 2).unwrap()
}

struct Image {
    dimension: usize,
    unknown_values: bool,
    values: Vec<bool>,
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_CASE: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

    #..#.
    #....
    ##..#
    ..#..
    ..###
    ";

    #[test]
    fn base_check() {
        assert_eq!("35", process_data(TEST_CASE.to_string()));
    }

    #[test]
    fn adv_check() {
        assert_eq!("3351", process_data_adv(TEST_CASE.to_string()));
    }
}
