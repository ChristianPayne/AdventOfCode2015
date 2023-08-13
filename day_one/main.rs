use std::fs;

struct Santa {
  floor: i32
}

impl Santa {
  fn new() -> Self {
    Self { floor: 0 }
  }
  fn change_floor(&mut self, instruction: char) {
    match instruction {
      '(' => {
        self.floor = self.floor + 1
      },
      ')' => {
        self.floor = self.floor - 1
      },
      _ => {}
    }
  }
}

fn main() {
  const FILE_PATH: &str = "./input.txt";

  // Read in input
  println!("In file {}", FILE_PATH);

  let input = fs::read_to_string(FILE_PATH)
      .expect("Failed to read the input file.");
  
  // Part 1
  {
    // Create state data
    let mut santa: Santa = Santa::new();

    // Run floor calc loop
    for instruction in input.chars() {
      santa.change_floor(instruction);
    }
  
    // Print result
    println!("Part 1 | Last floor Santa was on: {}", santa.floor);
  }

  // Part 2
  {
    // Create state data
    let mut santa: Santa = Santa::new();

    // Run floor calc loop
    for (index, instruction) in input.chars().enumerate() {
      santa.change_floor(instruction);

      if santa.floor == -1 {
        // Print out the position in the list when Santa crosses into the basement.
        println!("Part 2 | Santa entered the basement at position: {}", index + 1);
        break;
      }
    }
  }
}