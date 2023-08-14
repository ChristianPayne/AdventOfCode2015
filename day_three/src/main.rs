use std::fs;
use std::collections::HashMap;

const FILE_PATH: &str = "./src/input.txt";

fn main() {
    let input: String = fs::read_to_string(FILE_PATH)
      .expect("Failed to read the input file.");

    let mut house_map: HashMap<(i32, i32), u32> = HashMap::new();

    // (x,y)
    let mut current_house_coords: (i32, i32) = (0,0);
    let mut presents_delivered: u32 = 0;

    for movement in input.chars() {
      let current_house_value = match house_map.get(&current_house_coords) {
        None => 0,
        Some(x) => *x
      };

      house_map.insert(current_house_coords, current_house_value + 1);

      match movement {
          '^' => { current_house_coords = (current_house_coords.0, current_house_coords.1 + 1) },
          'v' => { current_house_coords = (current_house_coords.0, current_house_coords.1 - 1) },
          '>' => { current_house_coords = (current_house_coords.0 + 1, current_house_coords.1) },
          '<' => { current_house_coords = (current_house_coords.0 - 1, current_house_coords.1) },
          _   => { println!("Found a char that is not represented. {}", movement) }
      }

      presents_delivered = presents_delivered + 1;
    }

    println!("Day 1 | Total houses with presents: {}", house_map.keys().len());
    println!("Day 1 | Presents delivered: {}", presents_delivered);
}