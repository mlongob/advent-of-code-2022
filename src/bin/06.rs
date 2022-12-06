use itertools::Itertools;

fn find_signal(marker_count: usize, input: &str) -> Option<u32> {
    let (enum_count, _) = input
        .as_bytes()
        .windows(marker_count)
        .enumerate()
        .find(|(_, s)| s.iter().all_unique())?;
    Some((enum_count + marker_count) as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    find_signal(4, input)
}

pub fn part_two(input: &str) -> Option<u32> {
    find_signal(14, input)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(10));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(29));
    }
}
