use crate::problem_datatypes::Solution;
use crate::problem_datatypes::DataPoints;
use crate::problem_datatypes::Constraints;
use crate::fitness_evolution::FitnessEvolution;
use crate::arg_parser::ProgramParameters;
use crate::utils;
use crate::arg_parser::SearchType;
use crate::algorithms::local_search;

use rand::rngs::StdRng;
use std::time::Instant;

/// Ejecuta y muestra los resultados de la busqueda
/// Esto para no incluir todo este codigo en el cuerpo de la funcion main
pub fn run_and_show_results(data_points: &DataPoints, constraints: &Constraints, program_arguments: ProgramParameters, rng: &mut StdRng){
    // Numero maximo de iteraciones para la busqueda local <- Menos que en la busqueda local
    // clasica, porque estamos repitiendo varias veces la busqueda
    let max_fitness_evaluations = 10000;

    // Numero de veces que lanzamos la busqueda local
    let number_of_local_searchs = 10;

    // Comprobacion de seguridad -> Debemos estar usando en total el mismo numero de evaluaciones
    // del fitness que el resto de algoritmos
    debug_assert!(
        max_fitness_evaluations * number_of_local_searchs == 100000,
        "No estamos usando el numero adecuado de evaluaciones del fitness o lanzamientos de local search"
    );

    // Lanzamos 10 veces la busqueda local, guardando las soluciones
    // Tambien guardaremos las evoluciones del fitness, para poder guardarlas en un archivo al
    // finalizar
    let mut solutions = vec![];
    let mut fitness_evolutions = vec![];

    // Llevamos la cuenta del tiempo empleado en todo el proceso
    let before = Instant::now();
    for i in 0..number_of_local_searchs{
        let (solucion_local, fitness_evolution) = local_search::run(&data_points, &constraints, program_arguments.get_number_of_clusters(), max_fitness_evaluations, rng);
        solutions.insert(i as usize, solucion_local);
        fitness_evolutions.insert(i as usize, fitness_evolution);
    }

    // Las 10 repeticiones de local search han parado, cronometramos este instante
    let after = Instant::now();
    let duration = after.duration_since(before);
    let duration_numeric = duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9;

    // Escogemos el indice de la solucion con mejor fitness
    // Tomando el indice podemos tomar el fitness_evolution de la mejor solucion facilmente
    let best_index = select_best_solution(&solutions);

    // Recuperamos la mejor solucion y la mejor evolucion del fitness
    let best_solution = &solutions[best_index];
    let best_fit_ev = &fitness_evolutions[best_index];

    // Mostramos los resultados
    println!("==> Busqueda local multiarranque basica");
    println!("La distancia global instracluster de la solucion es: {}", best_solution.global_cluster_mean_distance());
    println!("El numero de restricciones violadas es: {}", best_solution.infeasibility());
    println!("El valor de fitness es: {}", best_solution.fitness());
    println!("El valor de lambda es: {}", best_solution.get_lambda());
    println!("Tiempo transcurrido (segundos): {}", duration_numeric);
    println!("Salvado del fitness: {:?}", best_fit_ev.save_as_numpy_file(&utils::generate_file_name(SearchType::LocalSearch)));
    println!("");
}

/// Dado un vector de soluciones, devuelve el indice de la solucion con menor fitness (la mejor
/// solucion para nuestro problema de minimizar el fitness)
/// El vector de soluciones debe tener al menos una solucion
fn select_best_solution(solutions: &Vec<Solution>) -> usize{
    // Comprobacion adicional de seguridad
    debug_assert!(solutions.len() > 0, "El vector de soluciones debe tener al menos un elemento");

    let mut best_index = 0;
    let mut best_fitness = solutions[best_index].fitness();

    for index in 0..solutions.len(){
        if solutions[index].fitness() < best_fitness{
            best_fitness = solutions[index].fitness();
            best_index = index;
        }
    }

    return best_index;
}
