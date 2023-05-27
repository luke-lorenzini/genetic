use genetic::Thing;

// 1 for max ones
const OPT_FUNC: usize = 1;

fn main() {
    let genes = 16;
    let population_size = genes * 2;
    let generation = population_size * 2;
    let mutation_probability = 0.0005;
    let crossover_probability = 0.7;

    let mut max = f32::MIN;
    let mut min = f32::MAX;
    let mut sum = 0.0;

    let mut xxx = Thing {
        genes,
        population_size,
        generation,
        mutation_probability,
        crossover_probability,
        chromosomes: vec![vec![0; genes]; population_size],
        fitnesses: vec![0.0; population_size],
        chromosomes_new_generation: vec![vec![0; genes]; population_size],
    };

    // for i in 0..xxx.population_size {
    //     xxx.randomize_chromosome(i);
    // }
    xxx.rand();

    for gen in 0..xxx.generation {
        // println!("Gneration: {gen}");
        for i in 0..xxx.population_size {
            xxx.fitnesses[i] = xxx.calculate_fitness(i, OPT_FUNC);
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
            "gen:{gen} min:{min} max:{} avg:{}",
            max,
            sum / xxx.population_size as f32
        );

        // xxx.roulette();
        xxx.tournament();

        xxx.xover();
        // xxx.crossover(&mut xxx.chromosomes_new_generation, crossover_probability, i);
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
        "gen:{generation} min:{} max:{} avg:{}",
        min,
        max,
        sum / xxx.population_size as f32
    );
}
