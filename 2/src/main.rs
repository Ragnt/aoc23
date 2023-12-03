use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn process_draw(draw: &str) -> bool {
    let min_cubes = [("red", 0), ("green", 0), ("blue", 0)];

    for color_qty in draw.split(",") {
        let parts: Vec<&str> = color_qty.trim().split_whitespace().collect();
        if parts.len() == 2 {
            if let Ok(qty) = parts[0].parse::<u32>() {
                if let Some(&(_, mut min_qty)) = min_cubes.iter().find(|&&(color, _)| color == parts[1]) {
                    if qty > min_qty {
                        min_qty = qty;
                    }
                }
            }
        }
    }

    true
}

fn extract_game_number(input: &str) -> Option<u32> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() > 1 && parts[0] == "Game" {
        return parts[1].replace(":", "").parse::<u32>().ok();
    }
    None
}

fn main() {
    println!("Advent of Code 1");

    let path = Path::new("input.txt");
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open input: {}", why),
        Ok(file) => file,
    };

    let mut fin: u32 = 0;
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read input: {}", why),
        Ok(_) => {
            for line in s.split("\n") {
                if !line.is_empty() {
                    let mut possible = true;
                    let game_num = extract_game_number(line).unwrap();
                    let mut game_min_cubes: [(&str, u32); 3] = [("red", 0), ("green", 0), ("blue", 0)];
                    let draws: Vec<&str> = line.split(": ").collect();
                    for draw in draws[1].split(";") {
                        for color_qty in draw.split(",") {
                            let parts: Vec<&str> = color_qty.trim().split_whitespace().collect();
                            if parts.len() == 2 {
                                if let Ok(qty) = parts[0].parse::<u32>() {
                                    if let Some((_, ref mut min_qty)) = game_min_cubes.iter_mut().find(|&&mut (color, _)| color == parts[1]) {
                                        if qty > *min_qty {
                                            *min_qty = qty;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    let product: u32 = game_min_cubes.iter().map(|&(_, value)| value).product();
                    println!("Game: {} | Min: {:?} | Product: {}",game_num, game_min_cubes, product);
                    fin += product;
                }
                
            }
            println!("Final Number: {}", fin);
        }
    }
}