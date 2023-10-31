use std::str;
use hex::ToHex;
use md5::{Md5, Digest};

#[aoc(day4, part1)]
fn part1(input: &str) -> u32 {
    solver(input, |hash| hash[0..2] == [0u8; 2] && (hash[2] & 0xF0) == 0)
}

#[aoc(day4, part2)]
fn part2(input: &str) -> u32 {
    solver(input, |hash| hash[0..3] == [0u8; 3])
}

// this is my first attempt in solving the problem
// where `str_start` was something like "00000".
// this method was too slow though, so I took
// a different approach based on gobanos's solution
// (https://github.com/gobanos/advent-of-code-2015/blob/master/src/day4.rs)
fn _solver(input: &str, str_start: &str) -> u32 {
    let mut i = 0;
    loop {
        let mut hasher = Md5::new();
        hasher.update(input.to_string() + &i.to_string());
        let result = Into::<[u8; 16]>::into(hasher.finalize());
        let hexadec = result.encode_hex::<String>();
        if hexadec.starts_with(str_start) {
            return i;
        }
        i += 1;
    }
}

// this is the new implementation, it's ~50% faster,
// although roughly half a second is needed still.
// TODO: add concurrency to the loop
fn solver(input: &str, is_valid: fn(&[u8; 16]) -> bool) -> u32 {
    (1..=u32::MAX)
        .map(|i| {
            //let mut hasher = ;
            //hasher.chain_update(&i.to_string());
            let mut hasher = Md5::new_with_prefix(input);
            hasher.update(&i.to_string());
            let hash = Into::<[u8; 16]>::into(hasher.finalize());
            (i, is_valid(&hash))
        }).find(|&(_, h)| h)
        .map(|(i, _)| i)
        .expect("value overflowed u32")
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn part1_test() {
        assert_eq!(part1("abcdef"), 609043);
        assert_eq!(part1("pqrstuv"), 1048970);
    }
}
