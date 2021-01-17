mod functions;
use std::io::stdin;

fn main() {
    println!("twentyfourtyeight-rust");
    println!("By Thomas");
    println!("Github repository at https://github.com/tduck973564/twentyfourtyeight-rust");

    let mut grid = functions::Grid::new();
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
                grid.move_up();
                break;
            } else if buffer == "s".to_string() {
                grid.move_down();
                break;
            } else if buffer == "a".to_string() {
                grid.move_left();
                break;
            } else if buffer == "d".to_string() {
                grid.move_right();
                break;
            } else {
                println!("Invalid control.");
            }
        }
        grid.scr_refresh();
    }
}
