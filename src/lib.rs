use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::Rng;

pub struct Thing {
    // genes: usize,
    pub population_size: usize,
    pub generation: usize,
    pub mutation_probability: f32,
    // crossover_probability:f32,
    pub chromosomes: Vec<Vec<u8>>,
    pub chromosomes_new_generation: Vec<Vec<u8>>,
    pub fitnesses: Vec<f32>,
}

impl Thing {
    pub fn muta(&mut self) {
        for i in 0..self.population_size {
            // mutate(&mut xxx.chromosomes_new_generation[i], xxx.mutation_probability);
            self.mutate(i);
        }
    }

    pub fn rand(&mut self) {
        for i in 0..self.population_size {
            self.randomize_chromosome(i);
        }
    }

    pub fn calculate_fitness(&mut self, index: usize, i: usize) -> f32 {
        match i {
            0 => self.function(index),
            _ => self.max_ones(index),
        }
    }

    pub fn tournament(&mut self) {
        let mut rng = rand::thread_rng();
        let max_val = self.population_size - 1;
        let mut idx1;
        let mut idx2;
        for i in 0..self.population_size {
            idx1 = rng.gen_range(0..=max_val);
            idx2 = rng.gen_range(0..=max_val);
            if self.fitnesses[idx1] > self.fitnesses[idx2] {
                self.chromosomes_new_generation[i] = self.chromosomes[idx1].clone();
            } else {
                self.chromosomes_new_generation[i] = self.chromosomes[idx2].clone();
            }
        }
    }

    pub fn replace(&mut self) {
        self.chromosomes = self.chromosomes_new_generation.clone();
    }

    fn mutate(&mut self, index: usize) {
        let dist = WeightedIndex::new([self.mutation_probability, 1.0 - self.mutation_probability])
            .unwrap();
        let mut rng = rand::thread_rng();

        // println!("{:?}", chromosome);

        for j in 0..self.chromosomes_new_generation[index].len() {
            let x = dist.sample(&mut rng);
            // println!("{x}");
            if x == 0 {
                if self.chromosomes_new_generation[index][j] == 0 {
                    self.chromosomes_new_generation[index][j] = 1;
                } else {
                    self.chromosomes_new_generation[index][j] = 0;
                }
            }
        }

        // println!("{:?}", chromosome);

        // println!();
    }

    fn randomize_chromosome(&mut self, index: usize) {
        let mut rng = rand::thread_rng();
        let max_val = 1;

        // println!()
        for i in 0..self.chromosomes[index].len() {
            self.chromosomes[index][i] = rng.gen_range(0..=max_val);
            // print!("{i}");
        }
        // println!("");
    }

    // fn crossover(self, pc:f32, i: usize) {
    //     println!("{:?}", self.chromosomes_new_generation[i]);// * pc;
    //     println!("{:?}", self.chromosomes_new_generation[i+1]);// * pc;
    // }

    fn max_ones(&mut self, index: usize) -> f32 {
        let mut fitness: f32 = 0.0;

        for i in 0..self.chromosomes[index].len() {
            fitness += f32::from(self.chromosomes[index][i]);
        }

        // println!("{fitness}");
        fitness
    }

    fn function(&mut self, index: usize) -> f32 {
        // yx^2-x^4
        // 3 bits
        //x = 1, y = 5
        // let fitness = 0.0;
        let mut x = 0;
        let mut y = 0;

        for i in 0..3 {
            x = (x << 1) | self.chromosomes[index][i];
        }

        for i in 3..6 {
            y = (y << 1) | self.chromosomes[index][i];
        }

        // (x * y).into()
        let xxx: i32 = y as i32 * (x as i32 ^ 2) - (x as i32 ^ 4);
        xxx as f32
    }

    fn _roulette(&mut self) {
        let dist = WeightedIndex::new(&self.fitnesses).unwrap();
        let mut rng = rand::thread_rng();
        for i in 0..self.population_size {
            self.chromosomes_new_generation[i] = self.chromosomes[dist.sample(&mut rng)].clone();
        }
    }

    fn _elite() {
        // store the best
    }
}
