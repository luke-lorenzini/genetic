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

    pub fn calculate_fitness(&mut self, index: usize, i: usize) -> f32 {
        match i {
            0 => self.opt_function(index),
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

    fn max_ones(&mut self, index: usize) -> f32 {
        let mut fitness: f32 = 0.0;

        for i in 0..self.chromosomes[index].len() {
            fitness += f32::from(self.chromosomes[index][i]);
        }

        // println!("{fitness}");
        fitness
    }

    fn opt_function(&mut self, index: usize) -> f32 {
        // yx^2-x^4
        // 3 bits
        // x = 2, y = 7
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
        let xxx = y as i32 * i32::pow(x as i32, 2) - i32::pow(x as i32, 4);
        // println!("x:{x} y:{y} xxx:{xxx}");
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

#[test]
fn test_function() {
    // optimization function is 0
    const OPT_FUNC: usize = 0;

    let genes = 6;
    let population_size = 1;

    let mut xxx = Thing {
        genes,
        population_size,
        generation: 0,
        mutation_probability: 0.0,
        crossover_probability: 0.0,
        chromosomes: vec![vec![0; genes]; population_size],
        fitnesses: vec![0.0; population_size],
        chromosomes_new_generation: vec![vec![0; genes]; population_size],
    };

    // Test for x=0 and y=0, res=0.0
    for i in 0..genes {
        xxx.chromosomes[0][i] = 0;
    }
    let res = xxx.calculate_fitness(0, OPT_FUNC);
    assert_eq!(0.0, res);

    // Test for x=7 and y=7, res=-2058.0
    for i in 0..genes {
        xxx.chromosomes[0][i] = 1;
    }
    let res = xxx.calculate_fitness(0, OPT_FUNC);
    assert_eq!(-2058.0, res);

    // Test for x=2 and y=7, res=12.0
    for i in 0..genes {
        xxx.chromosomes[0][i] = 0;
    }
    xxx.chromosomes[0][1] = 1;
    xxx.chromosomes[0][3] = 1;
    xxx.chromosomes[0][4] = 1;
    xxx.chromosomes[0][5] = 1;
    let res = xxx.calculate_fitness(0, OPT_FUNC);
    assert_eq!(12.0, res);
}

#[test]
fn test_max_ones() {
    // Max ones is 1
    const OPT_FUNC: usize = 1;

    let genes = 6;
    let population_size = 1;

    let mut xxx = Thing {
        genes,
        population_size,
        generation: 0,
        mutation_probability: 0.0,
        crossover_probability: 0.0,
        chromosomes: vec![vec![0; genes]; population_size],
        fitnesses: vec![0.0; population_size],
        chromosomes_new_generation: vec![vec![0; genes]; population_size],
    };

    // Test for x=0 and y=0, res=0.0
    for i in 0..genes {
        xxx.chromosomes[0][i] = 0;
    }
    let res = xxx.calculate_fitness(0, OPT_FUNC);
    assert_eq!(0.0, res);

    // Test for x=7 and y=7, res=6
    for i in 0..genes {
        xxx.chromosomes[0][i] = 1;
    }
    let res = xxx.calculate_fitness(0, OPT_FUNC);
    assert_eq!(6.0, res);

    // Test for x=2 and y=7, res=9.0
    for i in 0..genes {
        xxx.chromosomes[0][i] = 0;
    }
    xxx.chromosomes[0][1] = 1;
    xxx.chromosomes[0][3] = 1;
    xxx.chromosomes[0][4] = 1;
    xxx.chromosomes[0][5] = 1;
    let res = xxx.calculate_fitness(0, OPT_FUNC);
    assert_eq!(4.0, res);
}
