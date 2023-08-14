use std::fs;
use std::collections::HashMap;

const FILE_PATH: &str = "./src/input.txt";

struct Santa {
  coordinates: (i32, i32),
}

impl Santa {
    fn new() -> Self {
      Self { coordinates: (0,0) }
    }
    fn move_in_direction (&mut self, house_map: &mut HashMap<(i32, i32), u32>, direction: char) {
      let current_house_value = match house_map.get(&self.coordinates) {
        None => 0,
        Some(x) => *x
      };

      house_map.insert(self.coordinates, current_house_value + 1);
  
      match direction {
          '^' => { self.coordinates = (self.coordinates.0, self.coordinates.1 + 1) },
          'v' => { self.coordinates = (self.coordinates.0, self.coordinates.1 - 1) },
          '>' => { self.coordinates = (self.coordinates.0 + 1, self.coordinates.1) },
          '<' => { self.coordinates = (self.coordinates.0 - 1, self.coordinates.1) },
          _   => { println!("Found a char that is not represented. {}", direction) }
      }
    }
}

fn main() {
    let input: String = fs::read_to_string(FILE_PATH)
      .expect("Failed to read the input file.");

    
    { // Part 1
      let mut house_map: HashMap<(i32, i32), u32> = HashMap::new();
      let mut santa = Santa::new();
  
      for movement in input.chars() {
        santa.move_in_direction(&mut house_map, movement);
      }
  
      println!("Day 1 | Total houses with presents: {}", house_map.keys().len());
    }

    { // Part 2
      let mut house_map: HashMap<(i32, i32), u32> = HashMap::new();
      let mut santa = Santa::new();
      let mut robo_santa = Santa::new();

      for (index, movement) in input.chars().enumerate() {
        if index % 2 == 0 {
          santa.move_in_direction(&mut house_map, movement);
        } else {
          robo_santa.move_in_direction(&mut house_map, movement);
        }
      }

      println!("Day 2 | Total houses with presents: {}", house_map.keys().len());
    }
}