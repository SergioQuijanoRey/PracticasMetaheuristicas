use crate::problem_datatypes::Solution;
use crate::problem_datatypes::DataPoints;
use crate::problem_datatypes::Constraints;

/// Ejecuta la metaheuristica de busqueda local y devuelve la solucion encontrada
pub fn run(data_points: DataPoints, constraints: Constraints, number_of_clusters: i32, max_iterations: i32, seed: i32) -> Solution{

    // Partimos de una solucion inicial aleatoria
    // TODO -- no me gusta hacer este clone, aunque solo hago un clone
    let mut current_solution = Solution::generate_random_solution(data_points.clone(), constraints, number_of_clusters, seed);
    println!("Solucion inicial aleatoria: {:?}", current_solution.get_cluster_indexes());

    // Realizamos las iteraciones pertinentes
    for i in 0..max_iterations{

        // Tomamos el vecino
        let new_solution = match current_solution.get_neighbour(){
            Some(sol) => sol,

            // No hemos encontrado ningun vecino mejor, asi que paramos de iterar
            // Ademas mostramos la informacion de las iteraciones que nos hemos ahorrado
            None => {
                println!("Nos ahorramos {} iteraciones", max_iterations - i);
                break;
            },
        };

        // Mostramos algunas estadisticas
        println!("Mejoramos el fitness de {} a {}", current_solution.fitness(), new_solution.fitness());

        // Hacemos el cambio de solucion ahora que ya hemos mostrado la mejora de fitness
        current_solution = new_solution;

    }

    return current_solution;
}
