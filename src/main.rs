use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::Rng;

struct Thing {
    // genes: usize,
    population_size: usize,
    generation: usize,
    mutation_probability: f32,
    // crossover_probability:f32,
    chromosomes: Vec<Vec<u8>>,
    chromosomes_new_generation: Vec<Vec<u8>>,
    fitnesses: Vec<f32>,
}

impl Thing {
    fn muta(&mut self) {
        for i in 0..self.population_size {
            // mutate(&mut xxx.chromosomes_new_generation[i], xxx.mutation_probability);
            self.mutate(i);
        }
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

    fn rand(&mut self) {
        for i in 0..self.population_size {
            self.randomize_chromosome(i);
        }
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

    fn calculate_fitness(&mut self, index: usize, i: usize) -> f32 {
        match i {
            0 => self.function(index),
            _ => self.max_ones(index),
        }
    }

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

    fn roulette(&mut self) {
        let dist = WeightedIndex::new(&self.fitnesses).unwrap();
        let mut rng = rand::thread_rng();
        for i in 0..self.population_size {
            self.chromosomes_new_generation[i] = self.chromosomes[dist.sample(&mut rng)].clone();
        }
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

    fn replace(&mut self) {
        self.chromosomes = self.chromosomes_new_generation.clone();
    }

    fn elite() {
        // store the best
    }
}

fn main() {
    let genes = 16;
    let population_size = genes * 2;
    let generation = population_size * 2;
    let mutation_probability = 0.0005;
    // let crossover_probability = 0.0;

    let mut max = f32::MIN;
    let mut min = f32::MAX;
    let mut sum = 0.0;

    let mut xxx = Thing {
        // genes,
        population_size,
        generation,
        mutation_probability,
        // crossover_probability,
        chromosomes: vec![vec![0; genes]; population_size],
        fitnesses: vec![0.0; population_size],
        chromosomes_new_generation: vec![vec![0; genes]; population_size],
    };

    // for i in 0..xxx.population_size {
    //     xxx.randomize_chromosome(i);
    // }
    xxx.rand();

    for _gen in 0..xxx.generation {
        // println!("Gneration: {gen}");
        for i in 0..xxx.population_size {
            xxx.fitnesses[i] = xxx.calculate_fitness(i, 1);
        }

        sum = 0.0;
        max = f32::MIN;
        min = f32::MAX;
        for i in 0..xxx.population_size {
            sum += xxx.fitnesses[i];
            if xxx.fitnesses[i] > max {
                max = xxx.fitnesses[i];
            }
            if xxx.fitnesses[i] < min {
                min = xxx.fitnesses[i];
            }
        }
        println!(
            "min:{} max:{} avg:{}",
            min,
            max,
            sum / xxx.population_size as f32
        );

        // xxx.roulette();
        xxx.tournament();

        // //     // crossover(&mut xxx.chromosomes_new_generation, crossover_probability, i);
        // // }

        // for i in 0..xxx.population_size {
        //     // mutate(&mut xxx.chromosomes_new_generation[i], xxx.mutation_probability);
        //     xxx.mutate(i);
        // }
        xxx.muta();

        // xxx.chromosomes = xxx.chromosomes_new_generation.clone();
        xxx.replace();
    }

    println!(
        "min:{} max:{} avg:{}",
        min,
        max,
        sum / xxx.population_size as f32
    );
}
