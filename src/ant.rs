use super::environment::Environment;
use super::config;
use rand::{Rng, thread_rng};

#[derive(Clone)]
pub struct Ant {
    x: usize,
    y: usize,
    last_positions: Vec<(usize, usize)>,  // Stores the last few positions to avoid backtracking
    carrying_food: bool,
}

impl Ant {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            last_positions: vec![(x, y); 3],  // Initialize with the starting position repeated
            carrying_food: false,
        }
    }

    fn update_position_memory(&mut self, new_x: usize, new_y: usize) {
        if self.last_positions.len() >= 3 {
            self.last_positions.remove(0);  // Keep the vector size constant by removing the oldest entry
        }
        self.last_positions.push((new_x, new_y));  // Add the new position
    }

    fn is_position_in_memory(&self, x: usize, y: usize) -> bool {
        self.last_positions.contains(&(x, y))
    }


    pub fn update(&mut self, environment: &mut Environment) {
        if self.carrying_food {
            if self.x == environment.nest_position.0 && self.y == environment.nest_position.1 {
                self.carrying_food = false;
                environment.add_food_to_nest();
            } else {
                self.return_to_nest(environment);
            }
        } else {
            if let Some(index) = environment.find_food_near(self.x, self.y) {
                self.carrying_food = true;
                environment.food_positions.remove(index);
            } else if environment.has_significant_pheromones(self.x, self.y) {
                self.move_toward_food(environment);
            } else if rand::random::<f32>() < 0.1 {
                self.move_randomly(environment);
            }
        }
    }

    fn lay_pheromone(&mut self, environment: &mut Environment, direction: (f32, f32), intensity: f32) {
        environment.add_pheromone(self.x, self.y, direction, intensity);
    }


    // Also in Ant.rs



    fn move_toward_food(&mut self, environment: &Environment) {
        let mut directions = Vec::new();
        let ant_direction_from_nest = self.calculate_direction_from_nest(environment);

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 { continue; } // Skip current position
                let nx = (self.x as isize + dx).clamp(0, config::VIRTUAL_WIDTH as isize - 1) as usize;
                let ny = (self.y as isize + dy).clamp(0, config::VIRTUAL_HEIGHT as isize - 1) as usize;

                if let Some((intensity, direction)) = environment.pheromones[ny][nx] {
                    if intensity > 0.1 && !self.is_position_in_memory(nx, ny) {
                        if self.is_direction_valid(ant_direction_from_nest, direction) {
                            directions.push((intensity, nx, ny));
                        }
                    }
                }
            }
        }

        if let Some(&(_, nx, ny)) = directions.iter().max_by(|a, b| a.0.partial_cmp(&b.0).unwrap()) {
            self.update_position_memory(nx, ny);
            self.x = nx;
            self.y = ny;
        } else {
            self.move_randomly(environment);
        }
    }






    fn is_direction_valid(&self, ant_dir: (f32, f32), pheromone_dir: (f32, f32)) -> bool {
        let ant_dir_magnitude = (ant_dir.0.powi(2) + ant_dir.1.powi(2)).sqrt();
        let pheromone_dir_magnitude = (pheromone_dir.0.powi(2) + pheromone_dir.1.powi(2)).sqrt();

        let ant_dir_norm = (ant_dir.0 / ant_dir_magnitude, ant_dir.1 / ant_dir_magnitude);
        let pheromone_dir_norm = (pheromone_dir.0 / pheromone_dir_magnitude, pheromone_dir.1 / pheromone_dir_magnitude);

        let dot_product = ant_dir_norm.0 * pheromone_dir_norm.0 + ant_dir_norm.1 * pheromone_dir_norm.1;
        dot_product > 0.75  // Increase the threshold to ensure more alignment
    }



    fn calculate_direction_from_nest(&self, environment: &Environment) -> (f32, f32) {
        let dx = (self.x as isize - environment.nest_position.0 as isize) as f32;
        let dy = (self.y as isize - environment.nest_position.1 as isize) as f32;
        (dx, dy)
    }

    pub fn return_to_nest(&mut self, environment: &mut Environment) {
        // Assuming ants move towards the nest in straight lines for simplicity
        let direction = self.calculate_direction_from_nest(environment);
        while self.x != environment.nest_position.0 || self.y != environment.nest_position.1 {
            self.move_toward(environment.nest_position.0, environment.nest_position.1, 1);
            self.lay_pheromone(environment, direction, 1.0); // Lay pheromone with high intensity
        }
    }

    fn move_toward(&mut self, target_x: usize, target_y: usize, steps: usize) {
        for _ in 0..steps {
            if self.x < target_x {
                self.x += 1;
            } else if self.x > target_x {
                self.x -= 1;
            }

            if self.y < target_y {
                self.y += 1;
            } else if self.y > target_y {
                self.y -= 1;
            }
        }
    }

    fn move_randomly(&mut self, environment: &Environment) {
        let mut rng = thread_rng();
        for _ in 0..10 {
            let dx = rng.gen_range(-1..=1);
            let dy = rng.gen_range(-1..=1);
            let nx = (self.x as isize + dx).clamp(0, config::VIRTUAL_WIDTH as isize - 1) as usize;
            let ny = (self.y as isize + dy).clamp(0, config::VIRTUAL_HEIGHT as isize - 1) as usize;

            if !self.is_position_in_memory(nx, ny) {
                self.update_position_memory(nx, ny);
                self.x = nx;
                self.y = ny;
                break;
            }
        }
    }

    pub fn render(&self, frame: &mut [u8], scale_x: usize, scale_y: usize) {
        let scaled_x = self.x * scale_x;
        let scaled_y = self.y * scale_y;
        for dx in 0..scale_x {
            for dy in 0..scale_y {
                let idx = ((scaled_y + dy) * config::WIDTH + (scaled_x + dx)) * 4;
                if let Some(pixel) = frame.get_mut(idx..idx+4) {
                    if self.carrying_food {
                        pixel[0] = 0xff;  // Red (ants carrying food)
                        pixel[1] = 0x00;
                        pixel[2] = 0x00;
                        pixel[3] = 0xff;  // Alpha
                    } else {
                        pixel[0] = 0x00;
                        pixel[1] = 0xff;  // Green (ants not carrying food)
                        pixel[2] = 0x00;
                        pixel[3] = 0xff;  // Alpha
                    }
                }
            }
        }
    }
}

