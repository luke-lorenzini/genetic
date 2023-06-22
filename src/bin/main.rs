use genetic::{Genetic, Thing};

fn main() {
    let genes = 512;
    let population_size = genes * 2;
    let generation = population_size * 2;
    let mutation_probability = 0.0005;
    let crossover_probability = 0.7;

    // let mut max = f32::MIN;
    // let mut min = f32::MAX;
    // let mut sum = 0.0;

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

    xxx.evolve(max_ones);

    // xxx.rand();

    // for gen in 0..xxx.generation {
    //     // println!("Gneration: {gen}");
    //     for i in 0..xxx.population_size {
    //         xxx.fitnesses[i] = xxx.calculate_fitness(max_ones, i);
    //     }

    //     sum = 0.0;
    //     max = f32::MIN;
    //     min = f32::MAX;
    //     for i in 0..xxx.population_size {
    //         sum += xxx.fitnesses[i];
    //         if xxx.fitnesses[i] > max {
    //             max = xxx.fitnesses[i];
    //         }
    //         if xxx.fitnesses[i] < min {
    //             min = xxx.fitnesses[i];
    //         }
    //     }
    //     println!(
    //         "gen:{gen} min:{min} max:{} avg:{}",
    //         max,
    //         sum / xxx.population_size as f32,
    //     );

    //     // xxx.roulette();
    //     xxx.tournament();

    //     xxx.xover();

    //     xxx.muta();

    //     xxx.replace();
    // }

    // println!(
    //     "gen:{generation} min:{} max:{} avg:{}",
    //     min,
    //     max,
    //     sum / xxx.population_size as f32
    // );
}

fn max_ones(chromosome: &mut Vec<u8>) -> f32 {
    let mut fitness = 0.0;

    for i in 0..chromosome.len() {
        fitness += f32::from(chromosome[i]);
    }

    // println!("{fitness}");
    fitness
}

fn opt_function(chromosome: &mut Vec<u8>) -> f32 {
    // yx^2-x^4
    // 3 bits
    // x = 2, y = 7
    let mut x = 0;
    let mut y = 0;

    for val in chromosome.iter().take(3) {
        x = (x << 1) | val;
    }

    for val in chromosome.iter().take(6).skip(3) {
        y = (y << 1) | val;
    }

    // (x * y).into()
    (y as i32 * i32::pow(x as i32, 2) - i32::pow(x as i32, 4)) as f32
    // println!("x:{x} y:{y} xxx:{xxx}");
}

fn knapsack(chromosome: &mut Vec<u8>) -> f32 {
    let mut fitness = 0.0;
    let max_weight = 15.0;

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;
    let mut e = 0;

    for i in 0..3 {
        a = (a << 1) | chromosome[i];
    }

    for i in 3..6 {
        b = (b << 1) | chromosome[i];
    }

    for i in 6..9 {
        c = (c << 1) | chromosome[i];
    }

    for i in 9..12 {
        d = (d << 1) | chromosome[i];
    }

    for i in 12..15 {
        e = (e << 1) | chromosome[i];
    }

    // weight
    let w = 12.0 * a as f32 + 2.0 * b as f32 + 1.0 * c as f32 + 1.0 * d as f32 + 4.0 * e as f32;

    if w <= max_weight {
        // value
        fitness =
            4.0 * a as f32 + 2.0 * b as f32 + 2.0 * c as f32 + 1.0 * d as f32 + 10.0 * e as f32
    }

    fitness
}

#[test]
fn test_max_ones() {
    let genes = 6;
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
    let res = xxx.calculate_fitness(max_ones, 0);
    assert_eq!(0.0, res);

    // Test for x=7 and y=7, res=6
    for i in 0..genes {
        xxx.chromosomes[0][i] = 1;
    }
    let res = xxx.calculate_fitness(max_ones, 0);
    assert_eq!(6.0, res);

    // Test for x=2 and y=7, res=9.0
    for i in 0..genes {
        xxx.chromosomes[0][i] = 0;
    }
    xxx.chromosomes[0][1] = 1;
    xxx.chromosomes[0][3] = 1;
    xxx.chromosomes[0][4] = 1;
    xxx.chromosomes[0][5] = 1;
    let res = xxx.calculate_fitness(max_ones, 0);
    assert_eq!(4.0, res);
}

#[test]
fn test_opt_function() {
    let genes = 6;
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
    let res = xxx.calculate_fitness(opt_function, 0);
    assert_eq!(0.0, res);

    // Test for x=7 and y=7, res=-2058.0
    for i in 0..genes {
        xxx.chromosomes[0][i] = 1;
    }
    let res = xxx.calculate_fitness(opt_function, 0);
    assert_eq!(-2058.0, res);

    // Test for x=2 and y=7, res=12.0
    for i in 0..genes {
        xxx.chromosomes[0][i] = 0;
    }
    xxx.chromosomes[0][1] = 1;
    xxx.chromosomes[0][3] = 1;
    xxx.chromosomes[0][4] = 1;
    xxx.chromosomes[0][5] = 1;
    let res = xxx.calculate_fitness(opt_function, 0);
    assert_eq!(12.0, res);
}

#[test]
fn test_knapsack() {}
