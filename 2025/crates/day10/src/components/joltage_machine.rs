use itertools::Itertools;
use z3::{ast::Int, Solver};

pub struct JoltageMachine {
    buttons: Vec<Vec<usize>>,
    joltage: Vec<u32>,
}

impl TryFrom<&str> for JoltageMachine {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let [_lights, buttons @ .., joltage] =
            value.split_ascii_whitespace().collect_vec().as_slice()
        {
            Ok(JoltageMachine {
                buttons: buttons
                    .iter()
                    .map(|b| {
                        b[1..(b.len() - 1)]
                            .split(',')
                            .filter_map(|p| p.parse::<usize>().ok())
                            .collect_vec()
                    })
                    .sorted_by_key(|b| b.len())
                    .rev()
                    .collect_vec(),
                joltage: joltage[1..(joltage.len() - 1)]
                    .split(',')
                    .filter_map(|p| p.parse().ok())
                    .collect_vec(),
            })
        } else {
            Err("invalid line".to_owned())
        }
    }
}

impl JoltageMachine {
    pub fn required_presses(&self) -> usize {
        let total = Int::fresh_const("x");
        let params = (0..self.buttons.len())
            .map(|ix| Int::fresh_const(format!("x_{ix}").as_str()))
            .collect_vec();

        let solver = Solver::new();
        solver.assert(params.iter().sum::<Int>().eq(&total));

        for param in params.iter() {
            solver.assert(param.ge(0));
        }

        for (eq_params, eq_total) in self.get_equations() {
            solver.assert(
                eq_params
                    .iter()
                    .filter_map(|ix| params.get(*ix))
                    .sum::<Int>()
                    .eq(eq_total),
            );
        }

        solver
            .solutions(total, false)
            .filter_map(|i| i.as_u64())
            .min()
            .map(|i| i as usize)
            .unwrap()
    }

    fn get_equations(&self) -> Vec<(Vec<usize>, u32)> {
        self.joltage
            .iter()
            .enumerate()
            .map(|(eix, jtg)| {
                (
                    self.buttons
                        .iter()
                        .enumerate()
                        .filter_map(|(bix, b)| if b.contains(&eix) { Some(bix) } else { None })
                        .collect_vec(),
                    *jtg,
                )
            })
            .collect_vec()
    }
}
