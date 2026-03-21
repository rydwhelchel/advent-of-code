pub fn read_input() -> Vec<String> {
    let contents = std::fs::read_to_string("src/day2/input").unwrap();
    contents.lines().map(|l| String::from(l)).collect()
}

struct PasswordEntry {
    rule_char: char,
    rule_count_lowerbound: usize,
    rule_count_upperbound: usize,
    password_entry: String,
}
impl PasswordEntry {
    pub fn new(input: &String) -> Self {
        let entries: Vec<&str> = input.split(": ").collect();
        if entries.len() != 2 {
            panic!("expected only 2 entries after splitting on ': '");
        }

        let raw_rules: Vec<&str> = entries[0].split(" ").collect();
        let raw_rule_count: Vec<&str> = raw_rules[0].split("-").collect();
        let rule_char: char = raw_rules[1].trim().parse::<char>().unwrap();
        let (rule_count_lowerbound, rule_count_upperbound) = (
            raw_rule_count[0].parse::<usize>().unwrap(),
            raw_rule_count[1].parse::<usize>().unwrap(),
        );
        let password_entry = String::from(entries[1]);

        Self {
            rule_char,
            rule_count_lowerbound,
            rule_count_upperbound,
            password_entry,
        }
    }
}

pub fn part1(input: &Vec<String>) -> String {
    let mut valid_pes: Vec<PasswordEntry> = Vec::new();

    for line in input {
        let pe = PasswordEntry::new(&line);

        let mut char_count = 0;
        for c in pe.password_entry.chars() {
            if c == pe.rule_char {
                char_count += 1;
            }
        }
        if pe.rule_count_lowerbound <= char_count && char_count <= pe.rule_count_upperbound {
            valid_pes.push(pe);
        }
    }

    return valid_pes.len().to_string();
}

pub fn part2(input: &Vec<String>) -> String {
    let mut valid_pes: Vec<PasswordEntry> = Vec::new();

    for line in input {
        let pe = PasswordEntry::new(&line);

        if pe.password_entry.len() < pe.rule_count_lowerbound
            || pe.password_entry.len() < pe.rule_count_upperbound
        {
            // invalid because the password entry isn't long enough to be indexed by the rule
            continue;
        }

        let mut match_count = 0;
        if pe.password_entry.chars().collect::<Vec<char>>()[pe.rule_count_lowerbound - 1]
            == pe.rule_char
        {
            match_count += 1;
        }
        if pe.password_entry.chars().collect::<Vec<char>>()[pe.rule_count_upperbound - 1]
            == pe.rule_char
        {
            match_count += 1;
        }
        if match_count == 1 {
            valid_pes.push(pe);
        }
    }

    return valid_pes.len().to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> Vec<String> {
        let contents = std::fs::read_to_string("src/day2/testinput").unwrap();
        contents.lines().map(|l| String::from(l)).collect()
    }

    #[test]
    fn test_part1() {
        let input = test_input();
        assert_eq!(part1(&input), "2");
    }

    #[test]
    fn test_part2() {
        let input = test_input();
        assert_eq!(part2(&input), "1");
    }
}
