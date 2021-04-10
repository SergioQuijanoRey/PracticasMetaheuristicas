use crate::problem_datatypes::Solution;
use crate::problem_datatypes::DataPoints;
use crate::problem_datatypes::Constraints;
use crate::problem_datatypes::FitnessEvolution;
use rand::rngs::StdRng;

/// Ejecuta la metaheuristica de busqueda local y devuelve la solucion encontrada
pub fn run<'a, 'b>(data_points: &'a DataPoints, constraints: &'b Constraints, number_of_clusters: i32, max_iterations: i32, rng: &mut StdRng) -> (Solution<'a, 'b>, FitnessEvolution){
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
