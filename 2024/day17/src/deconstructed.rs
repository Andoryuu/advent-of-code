pub fn deconstruct_part_2() -> u64 {
    let target = [2, 4, 1, 1, 7, 5, 0, 3, 1, 4, 4, 5, 5, 5, 3, 0];
    let mut res = Vec::from([vec![]]);

    for (i, t) in target.into_iter().enumerate() {
        let (a, b, c) = to_bin(t);
        let mut new_res = Vec::new();
        for pattern in to_bin_pattern(a, b, c) {
            for item in &res {
                if let Some(merged) = try_merge(&pattern, item, i) {
                    new_res.push(merged);
                }
            }
        }
        res = new_res;
    }

    res.into_iter().map(from_bin).min().unwrap()
}

// merge two placeholdered bit arrays
fn try_merge(
    pattern: &[Option<bool>],
    into: &[Option<bool>],
    offset: usize,
) -> Option<Vec<Option<bool>>> {
    let offset = offset * 3;
    let mut res = Vec::from_iter(into.iter().take(offset).copied());
    for i in 0.. {
        let p = pattern.get(i);
        let t = into.get(i + offset);

        if let Some(&p) = p {
            if let Some(&t) = t {
                if let Some(p) = p {
                    if let Some(t) = t {
                        if p != t {
                            return None;
                        } else {
                            res.push(Some(p));
                        }
                    } else {
                        res.push(Some(p));
                    }
                } else if let Some(t) = t {
                    res.push(Some(t));
                } else {
                    res.push(None);
                }
            } else {
                res.push(p);
            }
        } else if let Some(&t) = t {
            res.push(t);
        } else {
            break;
        }
    }
    Some(res)
}

fn from_bin(mut bin: Vec<Option<bool>>) -> u64 {
    let mut res = 0;
    bin.reverse();
    for b in bin {
        res <<= 1;
        let b = b.unwrap_or(false);
        if b {
            res += 1;
        }
    }
    res
}

fn to_bin(n: u64) -> (bool, bool, bool) {
    (n & 4 > 0, n & 2 > 0, n & 1 > 0)
}

// matches reverse engineered `((reg_a >> (reg_a ^ 1) % 8) ^ reg_a ^ 5) % 8`
// computer for 0..7, with placeholders for undecided bits
fn to_bin_pattern(a: bool, b: bool, c: bool) -> Vec<Vec<Option<bool>>> {
    let mut res = vec![];

    if !b && c {
        res.push(vec![Some(false), Some(false), Some(false), Some(!a)]);
    }

    if a && !b && c {
        res.push(vec![Some(true), Some(false), Some(false)]);
    }

    res.push(vec![
        Some(false),
        Some(true),
        Some(false),
        Some(!c),
        Some(!b),
        Some(!a),
    ]);

    if !c {
        res.push(vec![
            Some(true),
            Some(true),
            Some(false),
            Some(!b),
            Some(!a),
        ]);
    }

    res.push(vec![
        Some(false),
        Some(false),
        Some(true),
        None,
        None,
        Some(!c),
        Some(b),
        Some(a),
    ]);
    res.push(vec![
        Some(true),
        Some(false),
        Some(true),
        None,
        Some(c),
        Some(b),
        Some(a),
    ]);
    res.push(vec![
        Some(false),
        Some(true),
        Some(true),
        None,
        None,
        None,
        None,
        Some(!c),
        Some(!b),
        Some(a),
    ]);
    res.push(vec![
        Some(true),
        Some(true),
        Some(true),
        None,
        None,
        None,
        Some(c),
        Some(!b),
        Some(a),
    ]);

    res
}
