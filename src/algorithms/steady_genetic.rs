use crate::problem_datatypes::Solution;
use crate::problem_datatypes::DataPoints;
use crate::problem_datatypes::Constraints;
use crate::fitness_evolution::FitnessEvolution;
use crate::arg_parser::ProgramParameters;
use crate::problem_datatypes::population::Population;
use crate::fitness_evaluation_result::FitnessEvaluationResult;
use crate::utils;
use crate::arg_parser::SearchType;

use rand::rngs::StdRng;
use std::time::Instant;

/// Ejecuta y muestra los resultados de la busqueda genetica con modelo estacionario
/// cross_uniform == true ==> usamos cruce uniforme
/// cross_uniform == false ==> usamos cruce de segmento fijo
pub fn run_and_show_results(data_points: &DataPoints, constraints: &Constraints, program_arguments: ProgramParameters, cross_uniform: bool, rng: &mut StdRng){
    // Parametros del algoritmo
    let max_fitness_evaluations = 100000;
    let population_size = 50;

    // El tamaño de un gen sera el tamaño de la poblacion de datos a asignar a clusters
    let gen_size = data_points.len();
    let mutation_probability_per_gen = 0.1 / gen_size as f64;

    let before = Instant::now();
    let (solucion, fitness_evolution) = run(&data_points, &constraints, program_arguments.get_number_of_clusters(), max_fitness_evaluations, rng, population_size, mutation_probability_per_gen, cross_uniform);
    let after = Instant::now();
    let duration = after.duration_since(before);
    let duration_numeric = duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9;

    // Para la salida de resultados formateada
    let search_type;
    if cross_uniform == true{
        search_type = SearchType::GenerationalGeneticUniform;
    }else{
        search_type = SearchType::GenerationalGeneticSegment;
    }

    // Mostramos los resultados
    println!("==> Busqueda genetica, modelo estatico, cross_uniform: {}", cross_uniform);
    println!("\t--> La distancia global instracluster de la solucion es: {}", solucion.global_cluster_mean_distance());
    println!("\t--> El numero de restricciones violadas es: {}", solucion.infeasibility());
    println!("\t--> El valor de fitness es: {}", solucion.fitness());
    println!("\t--> El valor de lambda es: {}", solucion.get_lambda());
    println!("\t--> Tiempo transcurrido (segundos): {}", duration_numeric);
    println!("\t--> Salvado del fitness: {:?}", fitness_evolution.save_as_numpy_file(&utils::generate_file_name(search_type)));
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
    mutation_probability_per_gen: f64,
    cross_uniform: bool // Si es false, significa que usamos cruce de segmento fijo
                        // Si es true, significa que usamos cruce uniforme
    )
    -> (Solution<'a, 'b>, FitnessEvolution){

    // Llevamos la cuenta del fitness del mejor individuo en cada iteracion sobre la poblacion
    let mut fitness_evolution = FitnessEvolution::new();

    // Poblacion inicial aleatoria
    let mut current_population = Population::new_random_population(data_points, constraints, number_of_clusters, population_size, rng);

    // Realizamos las iteraciones pertinentes
    let mut consumed_fitness_evaluations = 0;
    while consumed_fitness_evaluations < max_fitness_evaluations{

        // Las evaluaciones del fitness que se consumen en este ciclo
        let mut iteration_fitness_evaluations = 0;

        // Tomamos dos individuos de la poblacion por torneo binario
        // Si la poblacion anterior no esta evaluada, puede consumir hasta cuatro evaluaciones del
        // fitness (2 x 2 candidatos compitiendo)
        let selection_population_result = current_population.select_population_binary_tournament(2, rng);
        let selection_population = selection_population_result.get_result();
        iteration_fitness_evaluations += selection_population_result.get_iterations_consumed();
        debug_assert!(selection_population.population_size() == 2 as usize, "La poblacion de seleccion tiene {} elementos", selection_population.population_size());
        debug_assert!(
            selection_population_result.get_iterations_consumed() <= 4,
            "En la seleccion de dos individos por torneo binario debemos consumir, como mucho, 4 evaluaciones. Hemos consumido {} evaluaciones",
            selection_population_result.get_iterations_consumed()
        );

        // Cruzamos los dos individuos que hemos tomado de la poblacion, generando otros dos
        // individuos. Esto no deberia provocar evaluaciones del fitness
        let crossover_probability = 1.00; // Cruzamos forzosamente a los individuos
        let crossed_population_result;
        if cross_uniform == true{
            crossed_population_result = selection_population.cross_population_uniform(crossover_probability, rng);
        }else{
            crossed_population_result = selection_population.cross_population_segment(crossover_probability, rng);
        }

        let crossed_population = crossed_population_result.get_result();
        iteration_fitness_evaluations += crossed_population_result.get_iterations_consumed();
        debug_assert!(crossed_population.population_size() == 2 as usize, "La poblacion de seleccion tiene {} elementos", crossed_population.population_size());
        debug_assert!(
            crossed_population_result.get_iterations_consumed() == 0,
            "El cruce no deberia consumir iteraciones, hemos consumidos {} iteraciones",
            crossed_population_result.get_iterations_consumed()
        );
        debug_assert!(crossed_population.all_population_is_not_cached() == true, "Uno de los dos individuos tiene el valor del fitness cacheado");

        // A partir de los dos hijos cruzados, mutamos en caso de que se escoja aleatoriamente hacerlo
        // Esta operacion no consume iteraciones, por lo que no hacemos la suma
        let mutated_population = crossed_population.mutate_population_given_prob(mutation_probability_per_gen, rng);
        debug_assert!(mutated_population.population_size() == 2 as usize, "La poblacion de seleccion tiene {} elementos", mutated_population.population_size());
        debug_assert!(mutated_population.all_population_is_not_cached() == true, "Uno de los dos individuos tiene el valor del fitness cacheado");

        // Los dos hijos, cruzados y en algunos casos mutados, compiten contra los peores elementos
        // de la poblacion original para pasar a ser parte de ella
        // Esta operacion consume evaluaciones del fitness. Salvo en la primera iteracion, deberia
        // consumir como mucho dos evaluaciones (en los dos nuevos individuos que entran a competir)
        let final_population_result = current_population.compete_with_new_individuals(&mutated_population);
        let final_population = final_population_result.get_result();
        iteration_fitness_evaluations += final_population_result.get_iterations_consumed();
        debug_assert!(
            final_population_result.get_iterations_consumed() >= 2,
            "La competicion de los dos nuevos individuos contra toda la poblacion original debe provocar al menos dos evaluaciones del fitness, hemos obtenido {} evaluaciones",
            final_population_result.get_iterations_consumed()
        );

        // Evaluamnos esta poblacion final. No deberia consumir ninguna iteracion, pues en la
        // competicion de los dos nuevos individuos, ya deberiamos tener a toda la poblacion
        // evaluada. Hacemos esto por seguridad, pues al tener todos los fitness cacheados, no
        // deberia consumir demasiado tiempo
        let evaluate_poblation_result = final_population.evaluate_all_individuals();
        iteration_fitness_evaluations += evaluate_poblation_result.get_iterations_consumed();
        debug_assert!(
            evaluate_poblation_result.get_iterations_consumed() == 0,
            "La poblacion deberia estar evaluada tras la competicion, pero estamos consumiendo {} evaluaciones",
            evaluate_poblation_result.get_iterations_consumed()
        );

        // Realizamos el cambio de poblacion
        // Hacemos una comprobacion de seguridad sobne la poblacion, tras las competicion de los
        // dos nuevos individuos
        current_population = final_population.clone();
        debug_assert!(current_population.population_size() == population_size as usize, "La poblacion final tras la iteracion tiene {} elementos", current_population.population_size());

        // Añadimos las evaluaciones de fitness consumidas en esta pasada
        consumed_fitness_evaluations += iteration_fitness_evaluations as i32;

        // Llevamos la cuenta del valor del fitness de la mejor solucion de la poblacion en esta
        // iteracion. No deberia consumir evaluaciones del fitness porque ya en operaciones
        // pasadas estamos evaluando toda la poblacion. Ademas, hacemos una evaluacion sobre toda
        // la poblacion extra por seguridad (de nuevo, esa comprobacion de seguridad no consume
        // evaluaciones adicionales)
        let best_individual = current_population.get_best_individual().get_result().0;
        let best_individual_fitness = best_individual.fitness();
        fitness_evolution.add_iteration(best_individual_fitness);

    }

    return (current_population.get_best_individual().get_result().0.clone(), fitness_evolution);
}
