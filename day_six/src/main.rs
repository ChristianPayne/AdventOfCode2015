use std::collections::HashMap;
use std::fs;
use regex::Regex;
const FILE_PATH: &str = "./src/input.txt";
struct Light {
    on: bool,
    brightness: u32
}

impl Light {
    fn new() -> Self {
        Self {
            on: false,
            brightness: 0
        }
    }
    fn switch(mut self, value: bool) -> Light {
        self.on = value;
        if value == true {
            self.brightness += 1;
        } else if value == false && self.brightness > 0 {
            self.brightness -= 1;
        }
        self
    }
    fn toggle(mut self) -> Light {
        self.on = !self.on;
        self.brightness += 2;
        self
    }
}

fn main() {
    let mut light_map: HashMap<(i32, i32), Light> = HashMap::new();

    let input: String = fs::read_to_string(FILE_PATH).expect("Failed to read the input file.");
    
    for line in input.split("\n") {
        println!("{}", line);
        let re = Regex::new(r"(?P<action>turn on|turn off|toggle) (?P<x1>\d+),(?P<y1>\d+) through (?P<x2>\d+),(?P<y2>\d+)").unwrap();

        for cap in re.captures_iter(&line) {
            let action = cap.name("action").unwrap().as_str();

            let x1: i32 = cap.name("x1").unwrap().as_str().parse().unwrap();
            let y1: i32 = cap.name("y1").unwrap().as_str().parse().unwrap();
            let x2: i32 = cap.name("x2").unwrap().as_str().parse().unwrap();
            let y2: i32 = cap.name("y2").unwrap().as_str().parse().unwrap();

            for x in x1..x2 + 1 {
                for y in y1..y2 + 1 {
                    let coords = (x, y);
                    let current_value: Light = match light_map.get(&coords) {
                        None => Light::new(),
                        Some(_) => light_map.remove(&coords).unwrap()
                      };
                    
                    let modified_light: Light = match action {
                        "turn on" => current_value.switch(true),
                        "turn off" => current_value.switch(false),
                        _ => current_value.toggle()
                    };
                    
                    light_map.insert(coords, modified_light);
                }
            }
        }
    }

    println!("Part 1 | Lights that are on: {}", light_map.values().filter(|v| v.on == true).count());
    println!("Part 2 | Total brightness of lights: {}", light_map.values().map(|l| l.brightness).reduce(|reducer, val| reducer + val).unwrap());
}

// Too low: 568658