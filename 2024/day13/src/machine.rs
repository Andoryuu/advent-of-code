use itertools::Itertools;
use num::Integer;

pub struct Machine {
    button_a: (isize, isize),
    button_b: (isize, isize),
    prize: (isize, isize),
}

impl Machine {
    pub fn try_calculate_tokens(self, prize_correction: isize) -> Option<isize> {
        let (a_x, a_y) = self.button_a;
        let (b_x, b_y) = self.button_b;
        let (p_x, p_y) = (
            self.prize.0 + prize_correction,
            self.prize.1 + prize_correction,
        );

        let (a, a_rem) = (p_x * b_y - p_y * b_x).div_rem(&(a_x * b_y - a_y * b_x));
        let (b, b_rem) = (p_x * a_y - p_y * a_x).div_rem(&(b_x * a_y - b_y * a_x));

        // who's rem?
        if a_rem == 0 && b_rem == 0 {
            Some(a * 3 + b)
        } else {
            None
        }
    }
}

impl TryFrom<(&str, &str, &str)> for Machine {
    type Error = String;

    fn try_from((a, b, p): (&str, &str, &str)) -> Result<Self, Self::Error> {
        try_parse_tuple(a)
            .zip(try_parse_tuple(b))
            .zip(try_parse_tuple(p))
            .map(|((button_a, button_b), prize)| Self {
                button_a,
                button_b,
                prize,
            })
            .ok_or("invalid machine".to_owned())
    }
}

fn try_parse_tuple(value: &str) -> Option<(isize, isize)> {
    value
        .split(['=', '+', ','])
        .filter_map(|p| p.parse().ok())
        .collect_tuple()
}
