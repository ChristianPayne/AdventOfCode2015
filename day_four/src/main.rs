fn main() {
    const KEY: &str = "yzbqklnj";
    const MAX_TRIES: u32 = 10_000_000;

    let mut increment: u32 = 0;
    { // Part 1
        loop {
            // Create the string to hash
            let increment_string = format!("{}{}",KEY, increment);
            // Hash the key
            let generated_key = md5::compute(&increment_string);
            // Format the result
            let formatted_hash = format!("{:#x}", &generated_key);
            // Check the result
            let result = check_value(&formatted_hash, 5);
    
            // Stop loop if we reach max loop count (to stop infinite loops).
            if increment > MAX_TRIES {
                println!("Max tries reached! {}", increment);
                break;
            }
    
            // Print the result of we found it.
            if result {
                println!("Part 1 | Found a result {} with hash {}", increment, formatted_hash);
                break;
            }
    
            // Increment at the last step for the next loop.
            increment = increment + 1;
        }
    }

    { // Part 2
        loop {
            // Create the string to hash
            let increment_string = format!("{}{}",KEY, increment);
            // Hash the key
            let generated_key = md5::compute(&increment_string);
            // Format the result
            let formatted_hash = format!("{:#x}", &generated_key);
            // Check the result
            let result = check_value(&formatted_hash, 6);
    
            // Stop loop if we reach max loop count (to stop infinite loops).
            if increment > MAX_TRIES {
                println!("Max tries reached! {}", increment);
                break;
            }
    
            // Print the result of we found it.
            if result {
                println!("Part 2 | Found a result {} with hash {}", increment, formatted_hash);
                break;
            }
    
            // Increment at the last step for the next loop.
            increment = increment + 1;
        }
    }
}

fn check_value (generated_value: &String, consecutive_zeros: u8) -> bool {
    for (i, val) in generated_value.chars().enumerate() {
        // We have 5 consecutive zeros.
        if i >= consecutive_zeros.into() {
            return true;
        }

        // Break the for loop because we found a char that is not zero.
        if val != '0' {
            return false
        }
    }

    // Return at the end in case we don't return in the for loop.
    return  false;
}