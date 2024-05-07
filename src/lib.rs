pub mod environment;
pub mod ant;
pub mod config; // Ensure this module defines all necessary configuration constants.

use environment::Environment;
use ant::Ant;

pub struct Simulation {
    environment: Environment,
    ants: Vec<Ant>,
}

impl Simulation {
    pub const WIDTH: usize = config::WIDTH;
    pub const HEIGHT: usize = config::HEIGHT;
    pub const SCALE_X: usize = config::WIDTH / config::VIRTUAL_WIDTH;
    pub const SCALE_Y: usize = config::HEIGHT / config::VIRTUAL_HEIGHT;

    pub fn new() -> Self {
        let environment = Environment::new();
        let ants = (0..config::ANT_COUNT).map(|_| {
            // Ensure NEST_POSITION refers to the virtual dimensions, not scaled
            Ant::new(config::NEST_POSITION.0, config::NEST_POSITION.1)
        }).collect();
        Self { environment, ants }
    }

    pub fn update(&mut self) {
        for ant in &mut self.ants {
            ant.update(&mut self.environment);
        }

        // Update pheromones in the environment
        self.environment.update_pheromones();
    }

    pub fn render(&self, frame: &mut [u8]) {
        frame.fill(0); // Clear frame
        self.environment.render(frame, Self::SCALE_X, Self::SCALE_Y); // Pass scale factors to the render method
        for ant in &self.ants {
            ant.render(frame, Self::SCALE_X, Self::SCALE_Y); // Each ant is rendered considering scaling
        }
    }
}

