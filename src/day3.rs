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

#[aoc(day3, part2)]
fn part2(input: &[u8]) -> u32 {
    let mut positions = [(0, 0), (0, 0)];
    let mut houses = HashSet::<Position>::from([(0, 0)]);
    input.iter().enumerate().for_each(|(i, c)| {
        let pos_ref = &mut positions[i % 2];
        match c {
            b'<' => pos_ref.0 -= 1,
            b'^' => pos_ref.1 -= 1,
            b'>' => pos_ref.0 += 1,
            b'v' => pos_ref.1 += 1,
            _ => unreachable!()
        }
        houses.insert(*pos_ref);
    });
    houses.len() as u32
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn part1_tests() {
        assert_eq!(part1(b">"), 2);
        assert_eq!(part1(b"^>v<"), 4);
        assert_eq!(part1(b"^v^v^v^v^v"), 2);
    }

    #[test]
    fn part2_tests() {
        assert_eq!(part2(b"^v"), 3);
        assert_eq!(part2(b"^>v<"), 3);
        assert_eq!(part2(b"^v^v^v^v^v"), 11);
    }
}
