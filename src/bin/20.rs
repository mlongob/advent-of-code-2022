use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct EncryptedFile {
    codes: VecDeque<(usize, i32)>,
}

impl EncryptedFile {
    pub fn new() -> EncryptedFile {
        EncryptedFile {
            codes: VecDeque::new(),
        }
    }

    pub fn mix(&mut self) {
        // Iterate through all message ids
        (0..self.codes.len()).for_each(|id| {
            // Find the message id O(N), extract current position in the deque
            let pos = self
                .codes
                .iter()
                .enumerate()
                .find_map(|(pos, (n_id, _))| (*n_id == id).then_some(pos))
                .expect("All message ids must be present");

            // Rotate the deque such that the message is at the front
            self.codes.rotate_left(pos);

            // Pop out the message
            let message = self.codes.pop_front().expect("Collection cannot be empty");

            // Compute new position accounting for circular buffer
            let rotation = message.1.rem_euclid(self.codes.len() as i32) as usize;

            // Rotate to new position
            self.codes.rotate_left(rotation);

            // Re-insert the message
            self.codes.push_front(message);
        });
    }

    pub fn coordinate(&self) -> Option<i32> {
        // Find the zero
        let zero_position = self
            .codes
            .iter()
            .enumerate()
            .find_map(|(pos, (_, n))| (*n == 0).then_some(pos))?;

        // Sum the "interesting" positions in relations to the zero accounting for the circular buffer
        Some(
            [1000, 2000, 3000]
                .iter()
                .map(|th| self.codes[(th + zero_position) % self.codes.len()].1)
                .sum(),
        )
    }
}

impl FromStr for EncryptedFile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let codes = s
            .lines()
            .filter_map(|l| l.parse::<i32>().ok())
            .enumerate()
            .collect::<VecDeque<_>>();
        Ok(EncryptedFile { codes })
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut file = input.parse::<EncryptedFile>().ok()?;
    file.mix();
    file.coordinate()
}

pub fn part_two(input: &str) -> Option<i32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), None);
    }
}
