use crate::problem_datatypes::Solution;
use crate::problem_datatypes::DataPoints;
use crate::problem_datatypes::Constraints;
use crate::fitness_evolution::FitnessEvolution;
use crate::arg_parser::ProgramParameters;
use crate::arg_parser::SearchType;
use crate::utils;

use rand::rngs::StdRng;
use rand::Rng;
use std::time::Instant;

/// Ejecuta y muestra los resultados de la busqueda
/// Esto para no incluir todo este codigo en el cuerpo de la funcion main
pub fn run_and_show_results(data_points: &DataPoints, constraints: &Constraints, program_arguments: ProgramParameters, rng: &mut StdRng){
    // Parametros iniciales del algoritmo
    let max_fitness_evaluations = 100000;
    let mu = 0.3;
    let final_tmp = 0.001;
    let max_neighbours: i32 = (10.0 * data_points.len() as f64) as i32;
    let max_successes: i32 = (0.1 * max_neighbours as f64) as i32;
    let M: f64 = max_fitness_evaluations as f64 / max_neighbours as f64;

    // Solucion inicial aleatoria. La generamos aqui, porque es necesaria para establecer la
    // temperatura inicial
    let init_solution = Solution::generate_random_solution(data_points, constraints, program_arguments.get_number_of_clusters(), rng);

    // Con ello, computamos la temperatura inicial
    let initial_tmp: f64 = (mu * init_solution.fitness()) / (-mu.ln());

    // Comprobacion de seguridad
    assert_eq!(final_tmp < initial_tmp, true, "La temperatura final es mayor que la temperatura inicial");

    let before = Instant::now();
    let (solucion_local, fitness_evolution) = run(
        max_fitness_evaluations,
        &init_solution,
        initial_tmp,
        final_tmp,
        M,
        max_neighbours,
        max_successes,
        rng
    );
    let after = Instant::now();
    let duration = after.duration_since(before);
    let duration_numeric = duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9;

    // Mostramos los resultados
    println!("==> Enfriamiento simulado");
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
fn run<'a, 'b>(
    max_fitness_evaluations: i32,
    init_sol: &Solution<'a, 'b>,
    initial_tmp: f64,
    final_tmp: f64,
    M: f64,
    max_neighbours: i32,
    max_successes: i32,
    rng: &mut StdRng
) -> (Solution<'a, 'b>, FitnessEvolution){

    // Valores iniciales para empezar a iterar
    let mut current_evaluations = 0;
    let mut current_tmp = initial_tmp;
    let mut current_solution = init_sol.clone();
    let mut best_solution = current_solution.clone();
    let (mut best_fitness, ev_cons) = best_solution.fitness_and_consumed();

    // La solucion inicial deberia haber consumido una iteracion del fitness
    current_evaluations += ev_cons as i32;

    // Necesitamos el valor de beta para el enfriamiento
    let beta: f64 = (initial_tmp - final_tmp) / (M * initial_tmp * final_tmp);

    // Para llevar la cuenta de la evolucion del fitness
    let mut fitness_evolution = FitnessEvolution::new();

    while current_evaluations < max_fitness_evaluations && current_tmp >= final_tmp{

        // Bucle interno
        // Solo generamos max_neighbours a lo sumo. Tambien paramos cuando se ha alcanzado un
        // numero maximo de exitos
        let mut current_successes = 0;
        for _ in 0..max_neighbours{
            let current_neighbour = &current_solution.clone().one_random_neighbour(rng);

            // Calculamos el delta del fitness llevando en cuenta las evaluaciones de fitness
            let (current_solution_fitness, first_ev_cons) = current_solution.fitness_and_consumed();
            let (current_neighbour_fitness, second_ev_cons) = current_neighbour.fitness_and_consumed();
            let delta_fitness = current_solution_fitness - current_neighbour_fitness;

            // Añadimos las evaluaciones del fitness
            current_evaluations += first_ev_cons as i32 + second_ev_cons as i32;

            // TODO -- esta expresion, sin k, esta bien???
            if delta_fitness < 0.0 && rng.gen::<f64>() > (-delta_fitness * current_tmp){
                // La solucion es peor y ademas, la probabilidad dependiente de la temperatura de
                // aceptar soluciones peores ha fallado
                continue;
            }

            // Hemos aceptado la solucion, asi que hacemos el cambio de solucion actual y llevamos
            // la cuenta de los exitos
            current_solution = current_neighbour.clone();
            current_successes += 1;

            // Comprobamos si tenemos mejor coste que la mejor solucion encontrada hasta el
            // momento. Tenemos los valores de fitness ya calculados
            if current_solution_fitness < best_fitness{
                best_fitness = current_solution_fitness;
                best_solution = current_solution.clone();
            }

            // Llevamos la cuenta de como evoluciona el fitness
            fitness_evolution.add_iteration(current_solution_fitness);

            // Si hemos alcanzado el numero maximo de exitos, salimos del bucle interno
            if current_successes >= max_successes{
                break;
            }

            // Si hemos consumido las evaluaciones maximas en el bucle interno, debemos salir
            if current_evaluations >= max_fitness_evaluations{
                break;
            }
        }

        // Computamos el siguiente valor de la temperatura
        let old_tmp = current_tmp;
        current_tmp = current_tmp / (1.0 + beta * current_tmp);

        // Si se obtuvieron 0 exitos en el bucle interno, paramos de iterar
        if current_successes == 0{
            break;
        }

        // Comprobacion de que la temperatura va disminuyendo
        debug_assert!(current_tmp < old_tmp, "La temperatura debe descender monotamente");
    }

    return (best_solution.clone(), fitness_evolution);
}
