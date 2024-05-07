use rand::{Rng, thread_rng};
use super::config;

pub struct Environment {
    pub food_positions: Vec<(usize, usize)>,
    pub nest_position: (usize, usize),
    pub pheromones: Vec<Vec<Option<(f32, (f32, f32))>>>,  // Pheromone levels for each cell
}

impl Environment {
    pub const WIDTH: usize = config::VIRTUAL_WIDTH;
    pub const HEIGHT: usize = config::VIRTUAL_HEIGHT;


    pub fn new() -> Self {
        let mut food_positions = Vec::new();
        let pheromones = vec![vec![None; config::VIRTUAL_WIDTH]; config::VIRTUAL_HEIGHT];
        let mut rng = thread_rng();

        for _ in 0..config::CLUMP_COUNT {
            let cx = rng.gen_range(config::PADDING..config::VIRTUAL_WIDTH - config::PADDING) as isize;
            let cy = rng.gen_range(config::PADDING..config::VIRTUAL_HEIGHT - config::PADDING) as isize;

            for _ in 0..(config::FOOD_COUNT / config::CLUMP_COUNT) {
                let dx = rng.gen_range((-(config::CLUMP_RADIUS as isize))..=(config::CLUMP_RADIUS as isize));
                let dy = rng.gen_range((-(config::CLUMP_RADIUS as isize))..=(config::CLUMP_RADIUS as isize));
                let x = (cx + dx).clamp(config::PADDING as isize, (config::VIRTUAL_WIDTH - config::PADDING) as isize) as usize;
                let y = (cy + dy).clamp(config::PADDING as isize, (config::VIRTUAL_HEIGHT - config::PADDING) as isize) as usize;
                food_positions.push((x, y));
            }
        }

        Self { food_positions, nest_position: config::NEST_POSITION, pheromones }
    }

    pub fn update_pheromones(&mut self) {
        let decay_factor = 0.995;
        let threshold = 0.01;
        for y in 0..Self::HEIGHT {
            for x in 0..Self::WIDTH {
                if let Some((intensity, direction)) = self.pheromones[y][x] {
                    let new_intensity = intensity * decay_factor;
                    if new_intensity < threshold {
                        self.pheromones[y][x] = None;
                    } else {
                        self.pheromones[y][x] = Some((new_intensity, direction));
                    }
                }
            }
        }
    }


    pub fn get_pheromone_direction(&self, x: usize, y: usize) -> Option<(f32, f32)> {
        self.pheromones[y][x].map(|(_, direction)| direction)
    }

    pub fn find_food_near(&mut self, x: usize, y: usize) -> Option<usize> {
        self.food_positions.iter().position(|&(fx, fy)| fx == x && fy == y)
    }

    pub fn add_food_to_nest(&mut self) {
        // Logic for adding food to the nest (optional implementation for now)
    }

    pub fn has_significant_pheromones(&self, x: usize, y: usize) -> bool {
        let threshold = 0.01; // Define a threshold for significant pheromone levels
        self.pheromones.get(y)
            .and_then(|row| row.get(x))
            .map_or(false, |&option| option.map_or(false, |(intensity, _)| intensity > threshold))
    }


    // Add pheromone at specific location
    pub fn add_pheromone(&mut self, x: usize, y: usize, direction: (f32, f32), intensity: f32) {
        if x < Self::WIDTH && y < Self::HEIGHT {
            self.pheromones[y][x] = Some((intensity, direction));
        }
    }



    pub fn render(&self, frame: &mut [u8], scale_x: usize, scale_y: usize) {
        // Render food as small blue dots scaled to window size
        for &(x, y) in &self.food_positions {
            let scaled_x = x * scale_x;
            let scaled_y = y * scale_y;
            // Assuming each food piece fills a square area according to scale
            for dx in 0..scale_x {
                for dy in 0..scale_y {
                    let idx = ((scaled_y + dy) * config::WIDTH + (scaled_x + dx)) * 4;
                    if let Some(pixel) = frame.get_mut(idx..idx+4) {
                        pixel[0] = 0x00;  // Red component
                        pixel[1] = 0x00;  // Green component
                        pixel[2] = 0xff;  // Blue component (food is blue)
                        pixel[3] = 0xff;  // Alpha component
                    }
                }
            }
        }
        // Rendering the nest with scaling
        let nest_x = self.nest_position.0 * scale_x;
        let nest_y = self.nest_position.1 * scale_y;
        for dx in 0..scale_x {
            for dy in 0..scale_y {
                let idx = ((nest_y + dy) * config::WIDTH + (nest_x + dx)) * 4;
                if let Some(pixel) = frame.get_mut(idx..idx+4) {
                    pixel[0] = 0xff;  // Red component (nest is red)
                    pixel[1] = 0x65;  // Green component (darker shade for visibility)
                    pixel[2] = 0x00;  // Blue component
                    pixel[3] = 0xff;  // Alpha component
                }
            }
        }

        for y in 0..Self::HEIGHT {
            for x in 0..Self::WIDTH {
                if let Some((intensity, _)) = self.pheromones[y][x] {
                    let scaled_x = x * scale_x;
                    let scaled_y = y * scale_y;
                    let alpha = (intensity * 255.0).min(255.0) as u8;
                    for dx in 0..scale_x {
                        for dy in 0..scale_y {
                            let idx = ((scaled_y + dy) * config::WIDTH + (scaled_x + dx)) * 4;
                            if let Some(pixel) = frame.get_mut(idx..idx+4) {
                                pixel[0] = 0x80;  // Purple component
                                pixel[1] = 0x00;
                                pixel[2] = 0x80;  // Purple component
                                pixel[3] = alpha;  // Alpha component based on intensity
                            }
                        }
                    }
                }
            }
        }
    }
}



