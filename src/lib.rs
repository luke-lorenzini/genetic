use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::Rng;

pub struct Thing {
    pub genes: usize,
    pub population_size: usize,
    pub generation: usize,
    pub mutation_probability: f32,
    pub crossover_probability: f32,
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

    pub fn calculate_fitness(&mut self, f: fn(&mut Vec<u8>) -> f32, arg: usize) -> f32 {
        f(&mut self.chromosomes[arg])
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

    pub fn xover(&mut self) {
        for i in (0..self.population_size - 1).step_by(2) {
            self.crossover(i);
        }
    }

    pub fn replace(&mut self) {
        self.chromosomes = self.chromosomes_new_generation.clone();

        // for i in 0..self.chromosomes.len() {
        //     println!("{:?}", self.chromosomes[i]);
        // }
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

    fn crossover(&mut self, index: usize) {
        let dist =
            WeightedIndex::new([self.crossover_probability, 1.0 - self.crossover_probability])
                .unwrap();
        let mut rng = rand::thread_rng();
        let x = dist.sample(&mut rng);

        if x == 1 {
            let xover_point = rng.gen_range(0..=self.genes);

            // println!("Before Xover: {index}");
            // println!("xover point {xover_point}");
            // println!("{:?}", self.chromosomes_new_generation[index]); // * self.crossover_probability;
            // println!("{:?}", self.chromosomes_new_generation[index + 1]); // * pc;
            for i in 0..xover_point {
                let a = self.chromosomes_new_generation[index][i];
                self.chromosomes_new_generation[index][i] =
                    self.chromosomes_new_generation[index + 1][i];
                self.chromosomes_new_generation[index + 1][i] = a;
            }
            // println!("After Xover");
            // println!("{xover_point}");
            // println!("{:?}", self.chromosomes_new_generation[index]); // * self.crossover_probability;
            // println!("{:?}", self.chromosomes_new_generation[index + 1]); // * pc;
        }
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
