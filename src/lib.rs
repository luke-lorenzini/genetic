use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::Rng;

pub struct Thing {
    genes: usize,
    pub population_size: usize,
    pub generation: usize,
    mutation_probability: f32,
    crossover_probability: f32,
    pub chromosomes: Vec<Vec<u8>>,
    chromosomes_new_generation: Vec<Vec<u8>>,
    pub fitnesses: Vec<f32>,
}

pub trait Genetic {
    fn new(
        genes: usize,
        population_size: usize,
        generation: usize,
        mutation_probability: f32,
        crossover_probability: f32,
        chromosomes: Vec<Vec<u8>>,
        chromosomes_new_generation: Vec<Vec<u8>>,
        fitnesses: Vec<f32>,
    ) -> Self;
    fn evolve(&mut self, f: fn(&mut Vec<u8>) -> f32);
}

impl Default for Thing {
    fn default() -> Self {
        Thing {
            genes: 1,
            population_size: 2,
            generation: 4,
            mutation_probability: 0.0005,
            crossover_probability: 0.7,
            chromosomes: vec![vec![0; 1]; 2],
            chromosomes_new_generation: vec![vec![0; 1]; 2],
            fitnesses: vec![0.0; 2],
        }
    }
}

impl Genetic for Thing {
    fn new(
        genes: usize,
        population_size: usize,
        generation: usize,
        mutation_probability: f32,
        crossover_probability: f32,
        chromosomes: Vec<Vec<u8>>,
        chromosomes_new_generation: Vec<Vec<u8>>,
        fitnesses: Vec<f32>,
    ) -> Thing {
        Thing {
            genes,
            population_size,
            generation,
            mutation_probability,
            crossover_probability,
            chromosomes,
            chromosomes_new_generation,
            fitnesses,
        }
    }

    fn evolve(&mut self, f: fn(&mut Vec<u8>) -> f32) {
        let mut max = f32::MIN;
        let mut min = f32::MAX;
        let mut sum = 0.0;

        self.rand();

        for gen in 0..self.generation {
            // println!("Gneration: {gen}");
            for i in 0..self.population_size {
                self.fitnesses[i] = self.calculate_fitness(f, i);
            }

            sum = 0.0;
            max = f32::MIN;
            min = f32::MAX;
            for i in 0..self.population_size {
                sum += self.fitnesses[i];
                if self.fitnesses[i] > max {
                    max = self.fitnesses[i];
                }
                if self.fitnesses[i] < min {
                    min = self.fitnesses[i];
                }
            }
            println!(
                "gen:{gen} min:{min} max:{} avg:{}",
                max,
                sum / self.population_size as f32,
            );

            if false {
                self.roulette();
            } else {
                self.tournament();
            }

            self.xover();

            self.muta();

            self.replace();
        }

        println!(
            "gen:{} min:{} max:{} avg:{}",
            self.generation,
            min,
            max,
            sum / self.population_size as f32
        );
    }
}

impl Thing {
    fn muta(&mut self) {
        // let mut handles = vec![];
        // let mp = self.mutation_probability;

        for i in 0..self.population_size {
            // let mut xxx = self.chromosomes_new_generation[i].clone();
            // let handle = thread::spawn(move || {
            //     let dist = WeightedIndex::new([mp, 1.0 - mp]).unwrap();
            //     let mut rng = rand::thread_rng();

            //     for j in 0..xxx.len() {
            //         let x = dist.sample(&mut rng);
            //         if x == 0 {
            //             if xxx[j] == 0 {
            //                 xxx[j] = 1;
            //             } else {
            //                 xxx[j] = 0;
            //             }
            //         }
            //     }
            // });
            self.mutate(i);

            // handles.push(handle);
        }

        // for handle in handles {
        //     handle.join().unwrap();
        // }
    }

    fn rand(&mut self) {
        for i in 0..self.population_size {
            self.randomize_chromosome(i);
        }
    }

    pub fn calculate_fitness(&mut self, f: fn(&mut Vec<u8>) -> f32, arg: usize) -> f32 {
        f(&mut self.chromosomes[arg])
    }

    fn tournament(&mut self) {
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

    fn roulette(&mut self) {
        let dist = WeightedIndex::new(&self.fitnesses).unwrap();
        let mut rng = rand::thread_rng();
        for i in 0..self.population_size {
            self.chromosomes_new_generation[i] = self.chromosomes[dist.sample(&mut rng)].clone();
        }
    }

    fn xover(&mut self) {
        for i in (0..self.population_size - 1).step_by(2) {
            self.crossover(i);
        }
    }

    fn replace(&mut self) {
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

    fn _elite() {
        // store the best
    }
}
