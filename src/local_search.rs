use crate::problem_datatypes::Solution;
use crate::problem_datatypes::DataPoints;
use crate::problem_datatypes::Constraints;
use rand::rngs::StdRng;

/// Ejecuta la metaheuristica de busqueda local y devuelve la solucion encontrada
pub fn run<'a, 'b>(data_points: &'a DataPoints, constraints: &'b Constraints, number_of_clusters: i32, max_iterations: i32, rng: &mut StdRng) -> Solution<'a, 'b>{

    // Partimos de una solucion inicial aleatoria
    let mut current_solution = Solution::generate_random_solution(data_points, constraints, number_of_clusters, rng);
    println!("Solucion inicial aleatoria: {:?}", current_solution.get_cluster_indexes());

    // Realizamos las iteraciones pertinentes
    for i in 0..max_iterations{

        // Tomamos el vecino
        let new_solution = match current_solution.get_neighbour(rng){
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
