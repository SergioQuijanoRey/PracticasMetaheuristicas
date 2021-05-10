use crate::problem_datatypes::Solution;
use crate::problem_datatypes::DataPoints;
use crate::problem_datatypes::Constraints;
use crate::fitness_evolution::FitnessEvolution;
use crate::arg_parser::ProgramParameters;
use crate::problem_datatypes::population::Population;
use crate::fitness_evaluation_result::FitnessEvaluationResult;
use crate::arg_parser::SearchType;
use crate::utils;

use rand::rngs::StdRng;
use std::time::Instant;
use std::process::exit;

/// Ejecuta y muestra los resultados de la busqueda genetica con modelo estacionario
/// cross_uniform == true ==> usamos cruce uniforme
/// cross_uniform == false ==> usamos cruce de segmento fijo
pub fn run_and_show_results(data_points: &DataPoints, constraints: &Constraints, program_arguments: ProgramParameters, memetic_type: SearchType, rng: &mut StdRng){
    // Parametros del algoritmo
    // Son los mismos parametros para los tres tipos de algoritmo memetico
    let max_fitness_evaluations = 100000;
    let population_size = 50;
    let crossover_probability = 0.7;
    let max_fails = (0.1 * data_points.len() as f64) as i32;

    // El tamaño de un gen sera el tamaño de la poblacion de datos a asignar a clusters
    let gen_size = data_points.len();
    let mutation_probability_per_gen = 0.1 / gen_size as f64;

    let before = Instant::now();
    let (solucion, fitness_evolution) = run(
        &data_points,
        &constraints,
        program_arguments.get_number_of_clusters(),
        max_fitness_evaluations,
        rng,
        population_size,
        mutation_probability_per_gen,
        crossover_probability,
        max_fails,
        memetic_type
    );
    let after = Instant::now();
    let duration = after.duration_since(before);
    let duration_numeric = duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9;

    // Mostramos los resultados
    println!("==> Busqueda memetica, tipo memetico: {:?}", memetic_type);
    println!("\t--> La distancia global instracluster de la solucion es: {}", solucion.global_cluster_mean_distance());
    println!("\t--> El numero de restricciones violadas es: {}", solucion.infeasibility());
    println!("\t--> El valor de fitness es: {}", solucion.fitness());
    println!("\t--> El valor de lambda es: {}", solucion.get_lambda());
    println!("\t--> Tiempo transcurrido (segundos): {}", duration_numeric);
    println!("\t--> Salvado del fitness: {:?}", fitness_evolution.save_as_numpy_file(&utils::generate_file_name(memetic_type)));
    println!("");
}

/// Ejecuta la busqueda genetica y devuelve la solucion encontrada
// TODO -- hay que elegir el algoritmo genetico que mejores resultados nos haya aportado
fn run<'a, 'b>(
    data_points: &'a DataPoints,
    constraints: &'b Constraints,
    number_of_clusters: i32,
    max_fitness_evaluations: i32,
    rng: &mut StdRng,
    population_size: i32,
    mutation_probability_per_gen: f64,
    crossover_probability: f64,
    max_fails: i32,
    memetic_type: SearchType
    )
    -> (Solution<'a, 'b>, FitnessEvolution){

    // Comprobamos que el tipo de busqueda, dado por memetic_type, sea correcto. Esto porque el
    // enumerado puede llevar valores que no correspondan a memetico
    match memetic_type{
        // Opciones validas
        SearchType::MemeticAll => (),
        SearchType::MemeticRandom => (),
        SearchType::MemeticElitist => (),

        // Hemos alcanzado una opcion que no es valida
        _ => {
            println!("[Err] Tipo de busqueda memetica no correcta");
            println!("El tipo de busqueda memetica obtenida fue: {:?}", memetic_type);
            exit(-1);
        }
    }

    // Llevamos la cuenta del fitness del mejor individuo en cada iteracion sobre la poblacion
    // TODO -- puede relantecer demasiado el algoritmo?
    let mut fitness_evolution = FitnessEvolution::new();

    // Poblacion inicial aleatoria
    let mut current_population = Population::new_random_population(data_points, constraints, number_of_clusters, population_size, rng);

    // Realizamos las iteraciones pertinentes
    let mut consumed_fitness_evaluations = 0;
    let mut current_generation = 0;

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
        let crossover_probability = 1.0; // Cruzamos forzosamente a los individuos

        // TODO -- hay que elegir el operador de cruce
        let crossed_population_result = selection_population.cross_population_uniform(crossover_probability, rng);
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

        // Cada diez iteraciones, aplicamos la busqueda local suave segun el criterio que indica
        // memetic_type. Llevamos las cuentas de las evaluaciones adicionales que consume esta
        // busqueda local suave
        let soft_local_search_pop;
        if current_generation % 10 == 0{
            let soft_local_search_pop_result = final_population.soft_local_search(memetic_type, max_fails, rng);
            soft_local_search_pop = soft_local_search_pop_result.get_result().copy();
            iteration_fitness_evaluations += soft_local_search_pop_result.get_iterations_consumed();
        }else{
            soft_local_search_pop = final_population.copy();
        }

        // Realizamos el cambio de poblacion
        // Hacemos una comprobacion de seguridad sobne la poblacion, tras las competicion de los
        // dos nuevos individuos
        current_population = soft_local_search_pop.copy();
        debug_assert!(current_population.population_size() == population_size as usize, "La poblacion final tras la iteracion tiene {} elementos", current_population.population_size());

        // Añadimos las evaluaciones de fitness consumidas en esta pasada
        consumed_fitness_evaluations += iteration_fitness_evaluations as i32;

        // Hemos creado una nueva generacion
        current_generation += 1;

        // Llevamos la cuenta del valor del fitness de la mejor solucion de la poblacion en esta
        // iteracion. No deberia consumir evaluaciones del fitness porque ya en operaciones
        // pasadas estamos evaluando toda la poblacion. Ademas, hacemos una evaluacion sobre toda
        // la poblacion extra por seguridad (de nuevo, esa comprobacion de seguridad no consume
        // evaluaciones adicionales)
        let best_individual = current_population.get_best_individual().get_result().0;
        let best_individual_fitness = best_individual.fitness();
        fitness_evolution.add_iteration(best_individual_fitness);
    }

    return (current_population.get_best_individual().get_result().0.copy(), fitness_evolution);
}
