use crate::problem_datatypes::Solution;
use crate::problem_datatypes::DataPoints;
use crate::problem_datatypes::Constraints;
use crate::fitness_evolution::FitnessEvolution;
use crate::arg_parser::ProgramParameters;
use crate::utils;
use crate::arg_parser::SearchType;
use crate::algorithms::local_search;
use crate::algorithms::simulated_annealing;

use rand::rngs::StdRng;
use std::time::Instant;

/// Ejecuta y muestra los resultados de la busqueda
/// Esto para no incluir todo este codigo en el cuerpo de la funcion main
/// basic indica si usamos busqueda local (true) o enfriamiento simulado (false) entre repeticiones
pub fn run_and_show_results(data_points: &DataPoints, constraints: &Constraints, program_arguments: ProgramParameters, basic: bool, rng: &mut StdRng){
    // Numero maximo de iteraciones para la busqueda local y numero de repeticiones
    // Es menos que en otros algoritmos, porque estamos usando repeticiones
    let max_fitness_evaluations = 10000;
    let number_of_repetitions = 10;

    // Comprobacion de seguridad
    debug_assert!(
        max_fitness_evaluations * number_of_repetitions == 100000,
        "No estamos usando el numero adecuado de evaluaciones del fitness o lanzamientos de local search"
    );

    let before = Instant::now();
    let (solucion_local, fitness_evolution) = run(&data_points, &constraints, program_arguments.get_number_of_clusters(), max_fitness_evaluations, number_of_repetitions, basic, rng);
    let after = Instant::now();
    let duration = after.duration_since(before);
    let duration_numeric = duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9;

    // Mostramos los resultados
    println!("==> Busqueda local iterativa, basic: {}", basic);
    println!("La distancia global instracluster de la solucion es: {}", solucion_local.global_cluster_mean_distance());
    println!("El numero de restricciones violadas es: {}", solucion_local.infeasibility());
    println!("El valor de fitness es: {}", solucion_local.fitness());
    println!("El valor de lambda es: {}", solucion_local.get_lambda());
    println!("Tiempo transcurrido (segundos): {}", duration_numeric);
    println!("Salvado del fitness: {:?}", fitness_evolution.save_as_numpy_file(&utils::generate_file_name(SearchType::LocalSearch)));
    println!("");

}

/// Lanzamos la busqueda iterativa
fn run<'a, 'b>(data_points: &'a DataPoints, constraints: &'b Constraints, number_of_clusters: i32, max_fitness_evaluations: i32, number_of_repetitions: i32, basic: bool, rng: &mut StdRng) -> (Solution<'a, 'b>, FitnessEvolution){
    // Llevamos la cuenta de la evolucion del fintess
    let mut fitness_evolution = FitnessEvolution::new();

    // Generamos una solucion inicial aleatoria
    // Current solution sera la mejor solucion hasta el momento
    let mut current_solution = Solution::generate_random_solution(data_points, constraints, number_of_clusters, rng);
    fitness_evolution.add_iteration(current_solution.fitness()); // Por ser solo una evaluacion no tenemos en
                                                                 // cuenta esto en el maximo de evaluaciones

    // Realizamos las repeticiones dadas
    for _ in 0..number_of_repetitions{

        // Mutamos fuertemente la mejor solucion encontrada hasta el momento
        // Notar que esta mejor solucion no se modifica en el .hard_mutated
        let mut new_solution = current_solution.hard_mutated(rng);

        // Aplicamos busqueda local o enfriamiento simulado a esta solucion mutada fuertemente
        if basic == true{
            let (local_solution, _) = local_search::run_from_init_sol(max_fitness_evaluations, &new_solution, rng);
            new_solution = local_solution;
        }else{
            // Establecemos los parametros para aplicar enfriamiento simulado
            let mu = 0.3;
            let final_tmp = 0.001;
            let max_neighbours: i32 = (10.0 * data_points.len() as f64) as i32;
            let max_successes: i32 = (0.1 * max_neighbours as f64) as i32;
            let M: f64 = max_fitness_evaluations as f64 / max_neighbours as f64;
            let initial_tmp: f64 = (mu * current_solution.fitness()) / (-mu.ln());

            // Aplicamos enfriamiento simulado
            let (annealing_solution, _) = simulated_annealing::run(
                max_fitness_evaluations,
                &current_solution,
                initial_tmp,
                final_tmp,
                M,
                max_neighbours,
                max_successes,
                rng
            );
            new_solution = annealing_solution;
        }

        // Comprobamos si esta solucion es mejor que la que ya teniamos
        if new_solution.fitness() < current_solution.fitness(){
            current_solution = new_solution;
        }
    }

    return (current_solution, fitness_evolution);
}
