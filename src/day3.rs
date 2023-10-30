use std::collections::HashSet;

type Position = (i32, i32);

#[aoc(day3, part1)]
fn part1(input: &[u8]) -> u32 {
    let mut position = (0, 0);
    let mut houses = HashSet::<Position>::from([position]);
    input.iter().for_each(|c| {
        match c {
            b'<' => position.0 -= 1,
            b'^' => position.1 -= 1,
            b'>' => position.0 += 1,
            b'v' => position.1 += 1,
            _ => unreachable!()
        }
        houses.insert(position);
    });

    houses.len() as u32
}


#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn samples1() {
        assert_eq!(part1(b">"), 2);
        assert_eq!(part1(b"^>v<"), 4);
        assert_eq!(part1(b"^v^v^v^v^v"), 2);
    }
}
