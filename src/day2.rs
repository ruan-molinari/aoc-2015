#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<Present> {
    input
        .lines()
        .map(|l| {
            let mut present = l.trim().split('x').map(|f| f.parse().unwrap());

            Present {
                w: present.next().unwrap(),
                h: present.next().unwrap(),
                l: present.next().unwrap(),
            }
        }).collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Present]) -> u32 {
    input.iter().fold(0, |sum, p| {
        let pairs = [2*p.w*p.h, 2*p.h*p.l, 2*p.l*p.w];
        let area: u32 = pairs.iter().sum();
        let slack = pairs.iter().min().unwrap() / 2;
        sum + area + slack
    })
}

#[aoc(day2, part2)]
fn part2(input: &[Present]) -> u32 {
    input.iter().fold(0, |sum, p| {
        let bow_ribbon = p.w * p.h * p.l;
        let larger_size = &p.into_iter().max().unwrap();
        let wrapping_ribbon = match p.into_iter().position(|x| x == *larger_size){
            Some(0) => 2*p.h + 2*p.l,
            Some(1) => 2*p.w + 2*p.l,
            Some(2) => 2*p.w + 2*p.h,
            _ => unreachable!()
        };
        sum + wrapping_ribbon + bow_ribbon
    })
}

#[derive(Clone, Copy, Debug)]
struct Present {
    w: u32,
    h: u32,
    l: u32,
}

impl IntoIterator for Present {
    type Item = u32;
    type IntoIter = PresentIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        PresentIntoIterator {
            present: self,
            index: 0
        }
    }
}

struct PresentIntoIterator {
    present: Present,
    index: usize,
}

impl Iterator for PresentIntoIterator {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let result = match self.index {
            0 => self.present.w,
            1 => self.present.h,
            2 => self.present.l,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::day2::Present;
    use super::{part1, part2};

    const SAMPLES: &[Present] = &[
        Present {
            w: 2,
            h: 3,
            l: 4,
        },
        Present {
            w: 1,
            h: 1,
            l: 10,
        }
    ];

    #[test]
    fn part1_single_presents() {
        assert_eq!(part1(&[SAMPLES[0]]), 58);
        assert_eq!(part1(&[SAMPLES[1]]), 43);
    }

    #[test]
    fn part1_array_of_presents() {
        assert_eq!(part1(SAMPLES), 101);
    }

    #[test]
    fn part2_single_presents() {
        assert_eq!(part2(&[SAMPLES[0]]), 34);
        assert_eq!(part2(&[SAMPLES[1]]), 14);
    }

    #[test]
    fn part2_array_of_presents() {
        assert_eq!(part2(SAMPLES), 48);
    }
}
