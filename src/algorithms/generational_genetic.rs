use crate::problem_datatypes::Solution;
use crate::problem_datatypes::DataPoints;
use crate::problem_datatypes::Constraints;
use crate::fitness_evolution::FitnessEvolution;
use crate::arg_parser::ProgramParameters;
use crate::problem_datatypes::genetic;

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

    // TODO -- llevar la cuenta de como avanza el fitness en las iteraciones
    let fitness_evolution = FitnessEvolution::new();

    // Poblacion inicial aleatoria
    let current_population = genetic::Population::new_random_population(data_points, constraints, number_of_clusters, population_size, rng);

    // Realizamos las iteraciones pertinentes
    let mut consumed_fitness_evaluations = 0;
    while consumed_fitness_evaluations < max_fitness_evaluations{

        // Generamos una nueva poblacion a partir de torneos binarios
        // Como tamaño, tomamos toda la poblacion, porque esto es lo correspondiente al modelo
        // estacionario
        let selection_population = current_population.select_population_binary_tournament(population_size, rng);

        // A partir de la poblacion seleccionada, generamos una nueva poblacion a partir de los
        // cruces de los elementos de esa poblacion
        let crossed_population = selection_population.cross_population_uniform(crossover_probability, rng);

        // A partir de la poblacion cruzada, mutamos para generar una ultima poblacion
        let mutated_population = crossed_population.mutate_population(individuals_to_mutate, rng);


        // TODO -- BUG -- borrar esto
        consumed_fitness_evaluations += 100;
    }

    // TODO -- BUG -- borrar esto porque estamos haciendo mal la devolucion de la solucion
    let current_solution = Solution::generate_random_solution(data_points, constraints, number_of_clusters, rng);
    return (current_solution, fitness_evolution);
}
