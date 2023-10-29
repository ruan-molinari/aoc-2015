use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
fn part1(input: &str) -> i32 {
    input.chars().fold(0, |sum, c| match c {
        '(' => sum + 1,
        ')' => sum - 1,
        _ => unreachable!()
    })
}

#[aoc(day1, part2)]
fn part2(input: &str) -> usize {
    let mut sum: u32 = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => sum += 1,
            ')' => if let Some(s) = sum.checked_sub(1) {
                sum = s
            } else {
                return i + 1;
            },
            _ => unreachable!()
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn part1_chars() {
        assert_eq!(part1("((()))"), 0);
        assert_eq!(part1("))((()))"), -2);
        assert_eq!(part1("(((("), 4);
    }

    #[test]
    fn part2_chars() {
        assert_eq!(part2(")"), 1);
        assert_eq!(part2("(()))"), 5);
    }

}
