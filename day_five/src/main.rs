use std::fs;
const FILE_PATH: &str = "./src/input.txt";

struct Entry {
    valid: bool
}

impl Entry {
    fn new(input: String, validators: &Vec<&dyn Fn(&String) -> bool>) -> Self {
        let mut validator_results: Vec<bool> = Vec::new();

        for validator in validators.iter() {
            validator_results.push(validator(&input));
        }

        Self {
            valid: validator_results.iter().all(|r| *r == true)
        }
    }
}

fn main() {
    let input: String = fs::read_to_string(FILE_PATH)
    .expect("Failed to read the input file.");

    { // Part 1
        let mut validators: Vec<&dyn Fn(&String) -> bool> = Vec::from([]);

        // Add validators
        validators.push(&contains_three_vowels);
        validators.push(&contains_double_letter);
        validators.push(&does_not_contain_forbidden_strings);
        
        let mut valid_entries = 0;
    
        for line in input.split("\n") {
            let entry: Entry = Entry::new(String::from(line), &validators);
            if entry.valid {
                valid_entries = valid_entries + 1;
            }
        }
    
        println!("Part 1 | Total valid entries: {}", valid_entries);
    }

    { // Part 2
        let mut validators: Vec<&dyn Fn(&String) -> bool> = Vec::from([]);
        
        // Add validators
        validators.push(&contains_repeated_pair);
        validators.push(&contains_letter_with_one_between);

        let mut valid_entries = 0;
    
        for line in input.split("\n") {
            let entry: Entry = Entry::new(String::from(line), &validators);
            if entry.valid {
                valid_entries = valid_entries + 1;
            }
        }
    
        println!("Part 2 | Total valid entries: {}", valid_entries);
    }

}

fn contains_three_vowels(input: &String) -> bool {
    let vowels = "aeiou";
    let vowel_count = input.chars().filter(|c| vowels.contains(*c)).count();
    vowel_count >= 3
}

fn contains_double_letter(input: &String) -> bool {
    let mut prev_char = ' ';
    for current_char in input.chars() {
        if current_char == prev_char {
            return true;
        }
        prev_char = current_char;
    }
    false
}


fn does_not_contain_forbidden_strings(input: &String) -> bool {
    let forbidden_strings = ["ab", "cd", "pq", "xy"];
    !forbidden_strings.iter().any(|s| input.contains(s))
}

fn contains_repeated_pair(input: &String) -> bool {
    for i in 0..input.len() - 1 {
        let pair = &input[i..i + 2];
        if input[i + 2..].contains(pair) {
            return true;
        }
    }
    false
}

fn contains_letter_with_one_between(input: &String) -> bool {
    let chars: Vec<char> = input.chars().collect();
    for i in 0..input.len() - 2 {
        if chars[i] == chars[i + 2] {
            return true;
        }
    }
    false
}
