use std::fs;
const FILE_PATH: &str = "./src/input.txt";

struct Present {
    area: u32,
    smallest_area: u32,
    ribbon_length: u32,
}

impl Present {
    fn new(l: u32, w:u32, h:u32) -> Self {
        // Calculate the area and perimeter of the present.
        let mut areas = [l * w, w * h, h * l];
        let mut perimeters = [(l + w) * 2, (w + h) * 2, (h + l) * 2];

        // Sort them so the smallest are first in the arrays.
        areas.sort();
        perimeters.sort();

        let smallest_area = areas[0];
        let smallest_perimeter = perimeters[0];

        let bow_length = l * w * h;

        Self {
            area: areas.iter().map(|a| a * 2).sum(),
            smallest_area,
            ribbon_length: smallest_perimeter + bow_length
        }
    }
}

fn main() {
    let input: String = fs::read_to_string(FILE_PATH)
      .expect("Failed to read the input file.");

    let mut total_area: u32 = 0;
    let mut total_ribbon_length: u32 = 0;

    // Parse each line of the input.
    for line in input.split("\n") {
        // Parse the dimensions of a single line.
        let dimensions: Vec<u32> = line.split("x").map(|d: &str| d.parse::<u32>().unwrap()).collect();

        // Create a present struct using the dimensions.
        let present: Present = Present::new(dimensions[0], dimensions[1], dimensions[2]);

        total_area = total_area + present.area + present.smallest_area;
        total_ribbon_length = total_ribbon_length + present.ribbon_length;
    }

    println!("Part 1 | Total area to order: {}", total_area);
    println!("Part 2 | Total bow length: {}", total_ribbon_length);
}