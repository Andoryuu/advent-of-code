use itertools::Itertools;

use super::opcode::Opcode;

#[derive(Clone)]
pub struct Computer {
    register_a: usize,
    register_b: usize,
    register_c: usize,
    program: Vec<usize>,
    pointer: usize,
}

impl Computer {
    pub fn new(input: &str) -> Self {
        input
            .lines()
            .filter_map(|line| line.split_once(':').map(|(_, b)| b.trim()))
            .collect_tuple()
            .map(|(r_a, r_b, r_c, p)| Computer {
                register_a: r_a.parse::<usize>().unwrap(),
                register_b: r_b.parse::<usize>().unwrap(),
                register_c: r_c.parse::<usize>().unwrap(),
                program: p
                    .split(',')
                    .filter_map(|d| d.parse::<usize>().ok())
                    .collect_vec(),
                pointer: 0,
            })
            .unwrap()
    }

    pub fn process(mut self) -> String {
        let mut out = Vec::new();
        let program_len = self.program.len() - 1;
        while self.pointer < program_len {
            if let Some(o) = self.step() {
                out.push(o);
            }
        }
        out.into_iter().map(|d| d.to_string()).join(",")
    }

    fn step(&mut self) -> Option<usize> {
        let [opcode, value] = self.program[self.pointer..=self.pointer + 1] else {
            panic!("invalid pointer")
        };

        match Opcode::try_from(opcode).unwrap() {
            Opcode::Adv => {
                self.register_a >>= combo_op(self, value);
                self.pointer += 2;
                None
            }
            Opcode::Bxl => {
                self.register_b ^= value;
                self.pointer += 2;
                None
            }
            Opcode::Bst => {
                self.register_b = combo_op(self, value) % 8;
                self.pointer += 2;
                None
            }
            Opcode::Jnz => {
                if self.register_a != 0 {
                    self.pointer = value;
                } else {
                    self.pointer += 2;
                }
                None
            }
            Opcode::Bxc => {
                self.register_b ^= self.register_c;
                self.pointer += 2;
                None
            }
            Opcode::Out => {
                self.pointer += 2;
                Some(combo_op(self, value) % 8)
            }
            Opcode::Bdv => {
                self.register_b = self.register_a >> combo_op(self, value);
                self.pointer += 2;
                None
            }
            Opcode::Cdv => {
                self.register_c = self.register_a >> combo_op(self, value);
                self.pointer += 2;
                None
            }
        }
    }
}

fn combo_op(cpu: &Computer, value: usize) -> usize {
    match value {
        0..=3 => value,
        4 => cpu.register_a,
        5 => cpu.register_b,
        6 => cpu.register_c,
        _ => panic!("invalid operand"),
    }
}
