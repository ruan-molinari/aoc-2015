use std::str;
use hex::ToHex;
use md5::{Md5, Digest};

#[aoc(day4, part1)]
fn part1(input: &str) -> u32 {
    let mut i = 0;
    loop {
        let mut hasher = Md5::new();
        hasher.update(input.to_string() + &i.to_string());
        let result = Into::<[u8; 16]>::into(hasher.finalize());
        let hexadec = result.encode_hex::<String>();
        if hexadec.starts_with("00000") {
            return i;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::{part1};

    #[test]
    fn part1_test() {
        assert_eq!(part1("abcdef"), 609043);
        assert_eq!(part1("pqrstuv"), 1048970);
    }
}
