use crate::problem_datatypes::Solution;
use crate::problem_datatypes::DataPoints;
use crate::problem_datatypes::Constraints;
use crate::fitness_evolution::FitnessEvolution;
use crate::arg_parser::ProgramParameters;
use crate::problem_datatypes::population::Population;
use crate::fitness_evaluation_result::FitnessEvaluationResult;

use rand::rngs::StdRng;
use std::time::Instant;

/// Ejecuta y muestra los resultados de la busqueda genetica con modelo generacional
/// cross_uniform == true ==> usamos cruce uniforme
/// cross_uniform == false ==> usamos cruce de segmento fijo
pub fn run_and_show_results(data_points: &DataPoints, constraints: &Constraints, program_arguments: ProgramParameters, cross_uniform: bool, rng: &mut StdRng){
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
    let (solucion, fitness_evolution) = run(&data_points, &constraints, program_arguments.get_number_of_clusters(), max_fitness_evaluations, rng, population_size, crossover_probability, individuals_to_mutate, cross_uniform);
    let after = Instant::now();
    let duration = after.duration_since(before);
    let duration_numeric = duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9;

    // Mostramos los resultados
    println!("==> Busqueda genetica, modelo generacional, cross_uniform: {}", cross_uniform);
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
    individuals_to_mutate: i32,
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

        let mut iteration_fitness_evaluations = 0;

        // Generamos una nueva poblacion a partir de torneos binarios
        // Como tamaño, tomamos toda la poblacion, porque esto es lo correspondiente al modelo
        // estacionario
        let selection_population_result = current_population.select_population_binary_tournament(population_size, rng);
        let selection_population = selection_population_result.get_result();
        iteration_fitness_evaluations += selection_population_result.get_iterations_consumed();
        debug_assert!(selection_population.population_size() == population_size as usize, "La poblacion de seleccion tiene {} elementos", selection_population.population_size());

        // A partir de la poblacion seleccionada, generamos una nueva poblacion a partir de los
        // cruces de los elementos de esa poblacion. El cruce depende del tipo de cruce que estemos
        // estableciendo
        let crossed_population_result;
        if cross_uniform == true{
            crossed_population_result = selection_population.cross_population_uniform(crossover_probability, rng);
        }else{
            crossed_population_result = selection_population.cross_population_segment(crossover_probability, rng);
        }

        let crossed_population = crossed_population_result.get_result();
        iteration_fitness_evaluations += crossed_population_result.get_iterations_consumed();
        debug_assert!(crossed_population.population_size() == population_size as usize, "La poblacion de seleccion tiene {} elementos", crossed_population.population_size());

        // A partir de la poblacion cruzada, mutamos para generar una ultima poblacion
        // Esta operacion no consume iteraciones, por lo que no hacemos la suma
        let mutated_population = crossed_population.mutate_population(individuals_to_mutate, rng);
        debug_assert!(mutated_population.population_size() == population_size as usize, "La poblacion de seleccion tiene {} elementos", mutated_population.population_size());

        // En la poblacion nueva podemos estar perdiendo el mejor individuo de la poblacion
        // original. Tenemos que comprobar que dicho individuo sobreviva, y en caso de que no lo
        // haga, introducirlo en la nueva poblacion, en su poblacion original.
        // Esta operacion solo hace comprobaciones sobre el vector de posiciones (para comprobar
        // que ya tengamos la solucion en la poblacion) y por ello no consume iteraciones. De todas
        // formas, dejamos la comprobacion por seguridad (tenemos que elegir al mejor individuo de
        // la poblacion original. Esta poblacion deberia estar evaluada, pero por si acaso)
        let final_population_result = mutated_population.preserve_best_past_parent(&current_population);
        let final_population = final_population_result.get_result();
        iteration_fitness_evaluations += final_population_result.get_iterations_consumed();

        // Evaluamnos esta poblacion final. Esta operacion consume bastantes evaluaciones, porque
        // llegamos aqui con una poblacion altamente modificada, cuyos fitness no se han evaluado.
        // Otra gran parte de la poblacion, la que llega sin modificarse, no contribuye a estas
        // evaluaciones
        let evaluate_poblation_result = final_population.evaluate_all_individuals();
        iteration_fitness_evaluations += evaluate_poblation_result.get_iterations_consumed();

        // Realizamos el cambio de poblacion
        current_population = final_population.copy();
        debug_assert!(current_population.population_size() == population_size as usize, "La poblacion final tras la iteracion tiene {} elementos", current_population.population_size());

        // Añadimos las evaluaciones de fitness consumidas en esta pasada
        consumed_fitness_evaluations += iteration_fitness_evaluations as i32;

        // Llevamos la cuenta del valor del fitness de la mejor solucion de la poblacion en esta
        // iteracion
        let best_individual = current_population.get_best_individual().get_result().0;
        let best_individual_fitness = best_individual.fitness();
        fitness_evolution.add_iteration(best_individual_fitness);
    }

    return (current_population.get_best_individual().get_result().0.copy(), fitness_evolution);
}
