use crate::problem_datatypes::Solution;
use crate::problem_datatypes::DataPoints;
use crate::problem_datatypes::Constraints;
use crate::fitness_evolution::FitnessEvolution;
use crate::arg_parser::ProgramParameters;

use rand::rngs::StdRng;
use std::time::Instant;

/// Ejecuta y muestra los resultados de la busqueda
/// Esto para no incluir todo este codigo en el
pub fn run_and_show_results(data_points: &DataPoints, constraints: &Constraints, program_arguments: ProgramParameters, rng: &mut StdRng){
    // Numero maximo de iteraciones para la busqueda local
    let max_iterations = 100000;

    let before = Instant::now();
    let (solucion_local, fitness_evolution) = run(&data_points, &constraints, program_arguments.get_number_of_clusters(), max_iterations, rng);
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
    println!("Evolucion del fitness: {}", fitness_evolution);
    println!("");

}

/// Ejecuta la metaheuristica de busqueda local y devuelve la solucion encontrada
// TODO -- BUG -- creo que aqui, el numero maximo de iteraciones lo estamos haciendo mal
// TODO -- BUG -- una iteracion es una evaluacion de la funcion fitness
fn run<'a, 'b>(data_points: &'a DataPoints, constraints: &'b Constraints, number_of_clusters: i32, max_iterations: i32, rng: &mut StdRng) -> (Solution<'a, 'b>, FitnessEvolution){
    // Cuenta de como avanza la evolucion del fitness a traves de las iteraciones
    let mut fitness_evolution = FitnessEvolution::new();

    // Partimos de una solucion inicial aleatoria
    let mut current_solution = Solution::generate_random_solution(data_points, constraints, number_of_clusters, rng);
    fitness_evolution.add_iteration(current_solution.fitness());

    // Realizamos las iteraciones pertinentes
    for i in 0..max_iterations{

        // Tomamos el vecino
        let new_solution = match current_solution.get_neighbour(rng){
            Some(sol) => sol,

            // No hemos encontrado ningun vecino mejor, asi que paramos de iterar
            // Ademas mostramos la informacion de las iteraciones que nos hemos ahorrado
            None => {
                break;
            },
        };

        // Hacemos el cambio de solucion y guardamos la mejora del fitness
        current_solution = new_solution;
        fitness_evolution.add_iteration(current_solution.fitness());

    }

    return (current_solution, fitness_evolution);
}
