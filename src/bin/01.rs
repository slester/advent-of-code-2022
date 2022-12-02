pub fn part_one(input: &str) -> Option<u32> {
    let elf_values : Vec<u32> = vec![0];
    let elves = input.lines().fold(elf_values, |mut acc, element| {
        if element == "" {
            acc.push(0)
        } else {
            let last = acc.last_mut().unwrap();
            *last += element.parse::<u32>().unwrap();
        }
        acc
    });

    elves.iter().max().cloned()
}

pub fn part_two(input: &str) -> Option<u32> {
    let elf_values : Vec<u32> = vec![0];
    let mut elves = input.lines().fold(elf_values, |mut acc, element| {
        if element == "" {
            acc.push(0)
        } else {
            let last = acc.last_mut().unwrap();
            *last += element.parse::<u32>().unwrap();
        }
        acc
    });

    elves.sort();
    elves.reverse();
    Some(elves.iter().take(3).sum::<u32>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
