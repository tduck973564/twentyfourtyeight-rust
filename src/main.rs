mod functions;
use std::io::stdin;

fn main() {
    println!("twentyfourtyeight-rust");
    println!("By Thomas");
    println!("Github repository at https://github.com/tduck973564/twentyfourtyeight-rust");

    let mut grid = functions::Grid::new();
    grid.scr_refresh();
    loop {
        grid.spawn_block();
        if grid.check_status().0 == true {
            println!("You won!");
            break;
        } else if grid.check_status().1 == true {
            println!("You lost!");
            break;
        }
        loop {
            let mut buffer = String::new();
            stdin().read_line(&mut buffer).expect("could not read input"); buffer = buffer.trim().to_string();
            if buffer == "w".to_string() {
                grid.move_vertical(functions::VerticalDirections::Up);
                break;
            } else if buffer == "s".to_string() {
                grid.move_vertical(functions::VerticalDirections::Down);
                break;
            } else if buffer == "a".to_string() {
                grid.move_horizontal(functions::HorizontalDirections::Left);
                break;
            } else if buffer == "d".to_string() {
                grid.move_horizontal(functions::HorizontalDirections::Right);
                break;
            } else {
                println!("Invalid control.");
            }
        }
        grid.scr_refresh();
    }
}
