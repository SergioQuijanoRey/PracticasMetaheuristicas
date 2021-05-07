use crate::problem_datatypes::Solution;
use crate::problem_datatypes::DataPoints;
use crate::problem_datatypes::Constraints;
use crate::fitness_evolution::FitnessEvolution;
use crate::arg_parser::ProgramParameters;
use crate::problem_datatypes::population::Population;

use rand::rngs::StdRng;
use std::time::Instant;

/// Ejecuta y muestra los resultados de la busqueda genetica con modelo generacional
pub fn run_and_show_results(data_points: &DataPoints, constraints: &Constraints, program_arguments: ProgramParameters, rng: &mut StdRng){
    // Parametros del algoritmo
    let max_fitness_evaluations = 100000;
    let population_size = 50;
    let crossover_probability = 0.7;

    // El tamaño de un gen sera el tamaño de la poblacion de datos a asignar a clusters
    let gen_size = data_points.len();
    let mutation_probability_per_gen = 0.1 / gen_size as f64;
    let individuals_to_mutate = (mutation_probability_per_gen * gen_size as f64 * population_size as f64) as i32;

    // Comprobacion de seguridad. No se tiene en cuenta cuando usamos --release
    debug_assert!(individuals_to_mutate == 5, "El numero de individuos deberia ser 5, pero tenemos {} individuos a mutar", individuals_to_mutate);

    let before = Instant::now();
    let (solucion, fitness_evolution) = run(&data_points, &constraints, program_arguments.get_number_of_clusters(), max_fitness_evaluations, rng, population_size, crossover_probability, individuals_to_mutate);
    let after = Instant::now();
    let duration = after.duration_since(before);
    let duration_numeric = duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9;

    // Mostramos los resultados
    println!("==> Busqueda genetica, modelo generacional");
    println!("\t--> La distancia global instracluster de la solucion es: {}", solucion.global_cluster_mean_distance());
    println!("\t--> El numero de restricciones violadas es: {}", solucion.infeasibility());
    println!("\t--> El valor de fitness es: {}", solucion.fitness());
    println!("\t--> El valor de lambda es: {}", solucion.get_lambda());
    println!("\t--> Tiempo transcurrido (segundos): {}", duration_numeric);
    println!("\t--> Evolucion del fitness: {}", fitness_evolution);
    println!("");
}

/// Ejecuta la busqueda genetica y devuelve la solucion encontrada
fn run<'a, 'b>(
    data_points: &'a DataPoints,
    constraints: &'b Constraints,
    number_of_clusters: i32,
    max_fitness_evaluations: i32,
    rng: &mut StdRng,
    population_size: i32,
    crossover_probability: f64,
    individuals_to_mutate: i32)
    -> (Solution<'a, 'b>, FitnessEvolution){

    // Llevamos la cuenta del fitness del mejor individuo en cada iteracion sobre la poblacion
    // TODO -- puede relantecer demasiado el algoritmo?
    let mut fitness_evolution = FitnessEvolution::new();

    // Poblacion inicial aleatoria
    let mut current_population = Population::new_random_population(data_points, constraints, number_of_clusters, population_size, rng);

    // Realizamos las iteraciones pertinentes
    let mut consumed_fitness_evaluations = 0;
    while consumed_fitness_evaluations < max_fitness_evaluations{

        // Generamos una nueva poblacion a partir de torneos binarios
        // Como tamaño, tomamos toda la poblacion, porque esto es lo correspondiente al modelo
        // estacionario
        let selection_population = current_population.select_population_binary_tournament(population_size, rng);
        debug_assert!(selection_population.population_size() == population_size as usize, "La poblacion de seleccion tiene {} elementos", selection_population.population_size());

        // A partir de la poblacion seleccionada, generamos una nueva poblacion a partir de los
        // cruces de los elementos de esa poblacion
        let mut crossed_population = selection_population.cross_population_uniform(crossover_probability, rng);
        debug_assert!(crossed_population.population_size() == population_size as usize, "La poblacion de seleccion tiene {} elementos", crossed_population.population_size());

        // A partir de la poblacion cruzada, mutamos para generar una ultima poblacion
        // TODO -- esta generando soluciones en la poblacion no validas
        let mut mutated_population = crossed_population.mutate_population(individuals_to_mutate, rng);
        debug_assert!(mutated_population.population_size() == population_size as usize, "La poblacion de seleccion tiene {} elementos", mutated_population.population_size());

        // TODO -- borrar esto, es un parche para evitar el error anterior
        mutated_population.repair_bad_individuals(rng);

        // A partir de la poblacion mutada, sustituimos la poblacion original
        // Reemplazamiento con elitismo, se mantiene el miembro de la poblacion original con mejor
        // fitness
        // TODO -- esto deberia hacerlo si el mejor individuo de la poblacion no sobrevive


        // Tomamos el mejor elemento de la poblacion original y lo sustituimos por el peor de la
        // nueva poblacion
        let best_individual_at_original_pop = current_population.get_best_individual();

        // TODO -- BUG -- esta operacion genera los fallos
        // TODO -- BUG -- provoca: thread 'main' panicked at '[Err: Solution::intra_cluster_distance] Cluster without points', src/problem_datatypes/solution.rs:211:13
        let index_worst_individual_at_mut_pop = mutated_population.get_index_worst_individual();
        mutated_population.set_individual(index_worst_individual_at_mut_pop, best_individual_at_original_pop.copy());


        // Realizamos el cambio de poblacion
        current_population = mutated_population;
        debug_assert!(crossed_population.population_size() == population_size as usize, "La poblacion de seleccion tiene {} elementos", crossed_population.population_size());

        // TODO -- BUG -- borrar esto
        consumed_fitness_evaluations += 100;

        // Llevamos la cuenta del valor del fitness de la mejor solucion de la poblacion en esta
        // iteracion
        fitness_evolution.add_iteration(current_population.get_best_individual().fitness());
    }

    return (current_population.get_best_individual().copy(), fitness_evolution);
}
