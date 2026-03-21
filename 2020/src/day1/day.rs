pub fn read_input() -> Vec<u32> {
    let contents = std::fs::read_to_string("src/day1/input").unwrap();
    contents
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect()
}

pub fn part1(input: &Vec<u32>) -> String {
    for (i, num1) in input.iter().enumerate() {
        for num2 in input[i + 1..].iter() {
            if num1 + num2 == 2020 {
                return (num1 * num2).to_string();
            }
        }
    }
    return String::from("-1");
}

pub fn part2(input: &Vec<u32>) -> String {
    for (i, num1) in input.iter().enumerate() {
        for (j, num2) in input[i + 1..].iter().enumerate() {
            for num3 in input[i + 1..].iter() {
                if num1 + num2 + num3 == 2020 {
                    return (num1 * num2 * num3).to_string();
                }
            }
        }
    }
    return String::from("-1");
}
// tests
#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> Vec<u32> {
        let contents = std::fs::read_to_string("src/day1/testinput").unwrap();
        contents
            .lines()
            .map(|l| l.parse::<u32>().unwrap())
            .collect()
    }

    #[test]
    fn test_part1() {
        let input = test_input();
        assert_eq!(part1(&input), "514579");
    }

    #[test]
    fn test_part2() {
        let input = test_input();
        assert_eq!(part2(&input), "241861950");
    }
}
