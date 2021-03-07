use crate::problem_datatypes::Solution;
use crate::problem_datatypes::DataPoints;
use crate::problem_datatypes::Constraint;

/// Ejecuta la metaheuristica de busqueda local y devuelve la solucion encontrada
pub fn run(data_points: DataPoints, constraints: Vec<Constraint>, number_of_clusters: i32, max_iterations: i32) -> Solution{

    // Partimos de una solucion inicial aleatoria
    let mut current_solution = Solution::generate_random_solution(data_points, constraints, number_of_clusters);
    println!("Solucion inicial aleatoria: {:?}", current_solution.get_cluster_indexes());

    let mut iterations_since_improvement = 0;

    // Realizamos las iteraciones pertinentes
    for i in 0..max_iterations{

        if i % 100 == 0{
            println!("..");
        }

        // Tomamos el vecino
        let new_solution = current_solution.get_neighbour();

        // Si el vecino mejora el fitness, lo tomamos como nueva solucion actual
        if new_solution.fitness() < current_solution.fitness(){
            println!("Mejoramos el fitness de {} a {}", current_solution.fitness(), new_solution.fitness());
            current_solution = new_solution;

            // TODO -- BUG -- esto hay que gestionarlo en el struct solucion
            iterations_since_improvement = 0;
        }else{
            iterations_since_improvement += 1;
        }

        // No podemos mejorar mas el fitness
        if current_solution.fitness() == 0.0{
            println!("Solucion optima encontrada, nos ahorramos {} iteraciones", max_iterations - i);
            break;
        }

        if iterations_since_improvement == 1000{
            println!("TODO -- 1000 iteraciones sin mejorar la solucion");
            println!("Paramos porque nos hemos atascado supuestamente en un optimo local");
            break;
        }
    }

    return current_solution;
}
