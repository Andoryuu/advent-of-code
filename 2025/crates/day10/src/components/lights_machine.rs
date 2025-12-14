use itertools::Itertools;

pub struct LightsMachine {
    target_lights: u32,
    buttons: Vec<u32>,
}

impl TryFrom<&str> for LightsMachine {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let [lights, buttons @ .., _joltage] =
            value.split_ascii_whitespace().collect_vec().as_slice()
        {
            let target_lights = lights
                .chars()
                .filter_map(|c| match c {
                    '.' => Some(false),
                    '#' => Some(true),
                    _ => None,
                })
                .collect_vec();

            let size = target_lights.len();

            Ok(LightsMachine {
                target_lights: vec_to_bits(target_lights),
                buttons: buttons
                    .iter()
                    .map(|b| {
                        b[1..(b.len() - 1)]
                            .split(',')
                            .filter_map(|p| p.parse::<usize>().ok())
                            .fold(vec![false; size], |mut acc, ix| {
                                acc[ix] = true;
                                acc
                            })
                    })
                    .map(vec_to_bits)
                    .collect(),
            })
        } else {
            Err("invalid line".to_owned())
        }
    }
}

fn vec_to_bits(v: Vec<bool>) -> u32 {
    v.into_iter().fold(0, |mut acc, s| {
        acc <<= 1;
        if s {
            acc += 1;
        }
        acc
    })
}

impl LightsMachine {
    pub fn required_presses(&self) -> usize {
        if self.buttons.contains(&self.target_lights) {
            return 1;
        }

        (2..self.buttons.len())
            .find(|n| {
                self.buttons.iter().combinations(*n).any(|v| {
                    self.target_lights
                        == v.iter().copied().copied().reduce(|acc, b| acc ^ b).unwrap()
                })
            })
            .unwrap()
    }
}
