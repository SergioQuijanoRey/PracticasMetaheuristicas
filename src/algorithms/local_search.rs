use crate::problem_datatypes::Solution;
use crate::problem_datatypes::DataPoints;
use crate::problem_datatypes::Constraints;
use crate::fitness_evolution::FitnessEvolution;
use crate::arg_parser::ProgramParameters;
use crate::utils;
use crate::arg_parser::SearchType;

use rand::rngs::StdRng;
use std::time::Instant;

/// Ejecuta y muestra los resultados de la busqueda
/// Esto para no incluir todo este codigo en el cuerpo de la funcion main
pub fn run_and_show_results(data_points: &DataPoints, constraints: &Constraints, program_arguments: ProgramParameters, rng: &mut StdRng){
    // Numero maximo de iteraciones para la busqueda local
    let max_fitness_evaluations = 100000;

    let before = Instant::now();
    let (solucion_local, fitness_evolution) = run(&data_points, &constraints, program_arguments.get_number_of_clusters(), max_fitness_evaluations, rng);
    let after = Instant::now();
    let duration = after.duration_since(before);
    let duration_numeric = duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9;

    // Mostramos los resultados
    println!("==> Busqueda local");
    println!("La distancia global instracluster de la solucion es: {}", solucion_local.global_cluster_mean_distance());
    println!("El numero de restricciones violadas es: {}", solucion_local.infeasibility());
    println!("El valor de fitness es: {}", solucion_local.fitness());
    println!("El valor de lambda es: {}", solucion_local.get_lambda());
    println!("Tiempo transcurrido (segundos): {}", duration_numeric);
    println!("Salvado del fitness: {:?}", fitness_evolution.save_as_numpy_file(&utils::generate_file_name(SearchType::LocalSearch)));
    println!("");

}

/// Ejecuta la metaheuristica de busqueda local y devuelve la solucion encontrada
/// Parte de una solucion inicial aleatoria
pub fn run<'a, 'b>(data_points: &'a DataPoints, constraints: &'b Constraints, number_of_clusters: i32, max_fitness_evaluations: i32, rng: &mut StdRng) -> (Solution<'a, 'b>, FitnessEvolution){
    // Partimos de una solucion inicial aleatoria
    let init_sol = Solution::generate_random_solution(data_points, constraints, number_of_clusters, rng);

    // Usamos la funcion que corre la busqueda local cuando se le da la solucion inicial
    return run_from_init_sol(max_fitness_evaluations, &init_sol, rng);
}

/// Lanza la busqueda local.
/// Se necesita como argumento una solucion inicial.
/// Usar run si se quiere solucion inicial aleatoria
pub fn run_from_init_sol<'a, 'b>(max_fitness_evaluations: i32, init_sol: &Solution<'a, 'b>, rng: &mut StdRng) -> (Solution<'a, 'b>, FitnessEvolution){
    // Cuenta de como avanza la evolucion del fitness a traves de las iteraciones
    let mut fitness_evolution = FitnessEvolution::new();

    // Partimos de una solucion inicial dada por parametro
    let mut current_solution = init_sol.clone();
    fitness_evolution.add_iteration(current_solution.fitness());

    // Realizamos las iteraciones pertinentes mientras no hayamos consumido todas las evaluaciones
    // sobre el fitness
    let mut fitness_evaluations_consumed = 0;
    while fitness_evaluations_consumed < max_fitness_evaluations{

        // Las evaluaciones de fitness que se consumen en esta iteracion
        let mut current_fitness_consumed = 0;

        // Tomamos el vecino y tenemos en cuenta las evaluaciones del fitness consumidas
        let evaluations_left = max_fitness_evaluations - fitness_evaluations_consumed;
        let find_new_solution_result = current_solution.get_neighbour(evaluations_left, rng);
        let new_solution = find_new_solution_result.get_result();
        current_fitness_consumed += find_new_solution_result.get_iterations_consumed();

        let new_solution = match new_solution{
            Some(sol) => sol,

            // No hemos encontrado ningun vecino mejor, asi que paramos de iterar
            // Ademas mostramos la informacion de las iteraciones que nos hemos ahorrado
            None => {
                break;
            },
        };

        // Hacemos el cambio de solucion y guardamos la mejora del fitness
        // Este valor del fitness ya ha sido calculado en la busqueda del vecinadario, y por tanto,
        // no consume evaluaciones del fitness
        current_solution = new_solution.clone();
        fitness_evolution.add_iteration(current_solution.fitness());
        debug_assert!(current_solution.is_fitness_cached() == true, "El vecino generado debe tener el valor del fitness cacheado");

        // A??adimos todas las evaluaciones que se hayan consumido en la busqueda
        fitness_evaluations_consumed += current_fitness_consumed as i32;
    }

    return (current_solution, fitness_evolution);
}

