use rand::seq::SliceRandom;

pub struct Grid([Vec<usize>; 4], usize);

fn random_from_slice<T>(array: &[T]) -> &T {
    match array.choose(&mut rand::thread_rng()) {
        Some(x) => x,
        None => panic!("chosen value is None")
    }
}

impl Grid {
    pub fn new() -> Grid {
        Grid(
            [
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ],
            0,
        )
    }
    pub fn scr_refresh(&self) {
        println!("Score: {}", self.1);
        println!("{:?}", self.0[0]);
        println!("{:?}", self.0[1]);
        println!("{:?}", self.0[2]);
        println!("{:?}", self.0[3]);
    }
    pub fn check_status(&self) -> (bool, bool) {
        let mut won = false;
        let mut lost = true;
        for row in &self.0 {
            for &item in row {
                if item == 2048 {
                    won = true;
                } else if item == 0 {
                    lost = false;
                }
            }
        }
        (won, lost)
    }
    pub fn spawn_block(&mut self) {
        let mut open_indices: Vec<[usize; 2]> = Vec::new();
        let choosable_numbers = [2, 4, 8];
        for (row_index, row) in self.0.iter().enumerate() {
            for (index, &item) in row.iter().enumerate() {
                if item == 0 {
                    open_indices.push([row_index, index]);
                }
            }
        }
        let chosen_block = random_from_slice(&open_indices);
        self.0[chosen_block[0]][chosen_block[1]] = random_from_slice(&choosable_numbers).clone();
    }

    // Movement

    pub fn move_up(&mut self) {
        for _ in 0..4 {
            for (row_index, row) in self.0.clone().iter().enumerate() {
                for (index, &item) in row.iter().clone().enumerate() {
                    if row_index == 0 { 
                        break; 
                    } else if item > 0 && self.0[row_index - 1][index] == 0 {
                        self.0[row_index - 1][index] = item.clone();
                        self.0[row_index ][index] = 0;
                    }
                }
            } 
        }
        for (row_index, row) in self.0.clone().iter().enumerate() {
            for (index, &item) in row.iter().clone().enumerate() {
                if row_index == 0 {
                    break;
                } else if item > 0 && self.0[row_index - 1][index] == item {
                    self.0[row_index - 1][index] = item * 2;
                    self.0[row_index][index] = 0;
                    self.1 += item * 2;
                }
            }
        }
    }
    pub fn move_down(&mut self) {
        for _ in 0..4 {
            for (row_index, row) in self.0.clone().iter().enumerate() {
                for (index, &item) in row.iter().clone().enumerate() {
                    if row_index == 3 { 
                        break; 
                    } else if item > 0 && self.0[row_index + 1][index] == 0 {
                        self.0[row_index + 1][index] = item.clone();
                        self.0[row_index ][index] = 0;
                    }
                }
            } 
        }
        for (row_index, row) in self.0.clone().iter().enumerate() {
            for (index, &item) in row.iter().clone().enumerate() {
                if row_index == 3 {
                    break;
                } else if item > 0 && self.0[row_index + 1][index] == item {
                    self.0[row_index + 1][index] = item * 2;
                    self.0[row_index][index] = 0;
                    self.1 += item * 2;
                }
            }
        }
    }
    pub fn move_left(&mut self) {
        for _ in 0..4 {
            for (row_index, row) in self.0.clone().iter().enumerate() {
                for (index, &item) in row.iter().clone().enumerate() {
                    if index == 0 { 
                        continue;
                    } else if item > 0 && self.0[row_index][index - 1] == 0 {
                        self.0[row_index][index - 1] = item.clone();
                        self.0[row_index ][index] = 0;
                    }
                }
            } 
        }
        for (row_index, row) in self.0.clone().iter().enumerate() {
            for (index, &item) in row.iter().clone().enumerate() {
                if index == 0 {
                    continue;
                } else if item > 0 && self.0[row_index][index - 1] == item {
                    self.0[row_index][index - 1] = item * 2;
                    self.0[row_index][index] = 0;
                    self.1 += item * 2;
                }
            }
        }
    }
    pub fn move_right(&mut self) {
        for _ in 0..4 {
            for (row_index, row) in self.0.clone().iter().enumerate() {
                for (index, &item) in row.iter().clone().enumerate() {
                    if index == 3 { 
                        continue;
                    } else if item > 0 && self.0[row_index][index + 1] == 0 {
                        self.0[row_index][index + 1] = item.clone();
                        self.0[row_index][index] = 0;
                    }
                }
            } 
        }
        for (row_index, row) in self.0.clone().iter().enumerate() {
            for (index, &item) in row.iter().clone().enumerate() {
                if index == 3 {
                    continue; 
                } else if item > 0 && self.0[row_index][index + 1] == item {
                    self.0[row_index][index + 1] = item * 2;
                    self.0[row_index][index] = 0;
                    self.1 += item * 2;
                }
            }
        }
    }
}
