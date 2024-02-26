use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde::{Deserialize, Serialize};

use genetic::{Fitness, Thing};
use serde_json::Value;

fn main() {
    let path = "C:\\Users\\lukel\\OneDrive\\Desktop\\genetic\\data\\FoodData_Central_foundation_food_json_2023-04-20\\foundationDownload.json";
    let data = fs::read_to_string(path).unwrap();
    // println!("{}", data);
    let json: serde_json::Value = serde_json::from_str(&data).unwrap();

    // Open the file in read-only mode with buffer.
    // let file = File::open(path)?;
    // let reader = BufReader::new(file);
    // // Read the JSON contents of the file as an instance of `User`.
    // let u = serde_json::from_reader(reader);

    let genes = 16;
    let population_size = genes * 2;
    let generation = population_size * 2;
    let mutation_probability = 0.0005;
    let crossover_probability = 0.7;

    let mut xxx = Thing::new(
        genes,
        population_size,
        generation,
        mutation_probability,
        crossover_probability,
        vec![vec![0; genes]; population_size],
        vec![vec![0; genes]; population_size],
        vec![0.0; population_size],
    );

    xxx.evolve(MinMax::fitness);
}

struct Food {
    cals: usize,
    fibre: usize,
}

struct MinMax;

impl Fitness for MinMax {
    fn fitness(chromosome: &mut Vec<u8>) -> f32 {
        let banana = Food { cals: 89, fibre: 2 };
        let strawberry = Food { cals: 6, fibre: 6 };
        let apple = Food {
            cals: 52,
            fibre: 10,
        };
        let pear = Food {
            cals: 30,
            fibre: 10,
        };

        let mut x = 0;
        let mut y = 0;
        let mut z = 0;
        let mut q = 0;

        for val in chromosome.iter().take(4) {
            x = (x << 1) | val;
        }

        for val in chromosome.iter().skip(4).take(4) {
            y = (y << 1) | val;
        }

        for val in chromosome.iter().skip(8).take(4) {
            z = (z << 1) | val;
        }

        for val in chromosome.iter().skip(12).take(4) {
            q = (q << 1) | val;
        }

        println!("bananas:{x}, strawberries:{y}, apples:{z}, pear:{q}");

        // maximize cals, but keep it under 1400
        let res = ((banana.cals * x as usize)
            + (strawberry.cals * y as usize)
            + (apple.cals * z as usize)
            + (pear.cals * q as usize)) as f32;
        if res <= 1400.0 {
            res
        } else {
            -1.0
        }
        // res
    }

    fn summarize(chromosome: &Vec<u8>) {
        println!("{:?}", chromosome);
    }
}

#[test]
fn test_min_max() {
    let genes = 8;
    let population_size = 1;

    let mut xxx = Thing::new(
        genes,
        population_size,
        0,
        0.0,
        0.0,
        vec![vec![0; genes]; population_size],
        vec![vec![0; genes]; population_size],
        vec![0.0; population_size],
    );

    // Test for x=0 and y=0, res=0.0
    for i in 0..genes {
        xxx.chromosomes[0][i] = 0;
    }
    let res = xxx.calculate_fitness(MinMax::fitness, 0);
    assert_eq!(0.0, res);

    // Test for x=0 and y=0, res=0.0
    for i in 0..genes {
        xxx.chromosomes[0][i] = 1;
    }
    let res = xxx.calculate_fitness(MinMax::fitness, 0);
    assert_eq!(180.0, res);
}
