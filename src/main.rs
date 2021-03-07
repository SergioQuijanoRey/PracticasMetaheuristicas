use std::error::Error;
use std::boxed::Box;
use std::process::exit;
use csv;

// Ficheros en los que separo mi codigo
mod arg_parser;
mod file_parsers;
mod problem_datatypes;

fn show_help(){
    println!("Modo de uso del programa:");
    println!("\t./PracticasMetaheuristicas <data_file> <constraints_file> <seed>")
}

fn main() {

    let program_arguments = match arg_parser::ProgramParameters::new(){
        Ok(value) => value,
        Err(err) => {
            eprintln!("No se pudo leer los parametros dados por terminal");
            eprintln!("[Errcode]: {}", err);
            show_help();
            exit(-1);
        }
    };

    // Parseamos los datos del archivo de datos
    let data_points = match file_parsers::parse_data_file_to_struct(&program_arguments.get_data_file()){
        Ok(value) => value,
        Err(err) => {
            eprintln!("No se pudieron leer los datos del fichero {}", program_arguments.get_data_file());
            eprintln!("[Errcode]: {}", err);
            exit(-1);
        }
    };

    // Parseamos los datos del archivo de restricciones
    let constraints = match file_parsers::parse_constraints_file_to_struct(&program_arguments.get_constraints_file()){
        Ok(value) => value,
        Err(err) => {
            eprintln!("No se pudieron leer los datos de restricciones del fichero {}", program_arguments.get_constraints_file());
            eprintln!("[Errcode]: {}", err);
            exit(-1);
        }
    };

    println!("Datos del problema cargados con exito, procediendo a calcular las soluciones");
    println!("================================================================================");
    println!("");

    let max_iterations = 100000;
    //let max_iterations = 100; // TODO -- para debuggear

    // TODO -- read this from cli args
    let number_of_clusters = 7;

    let mut current_solution = problem_datatypes::Solution::generate_random_solution(data_points, constraints, number_of_clusters);
    println!("Initial solution is {:?}", current_solution.get_cluster_indexes());

    for i in 0..max_iterations{
        let new_solution = current_solution.get_neighbour();

        if new_solution.fitness() < current_solution.fitness(){
            println!("Fitness got better, from {} to {}", current_solution.fitness(), new_solution.fitness());
            current_solution = new_solution;
        }

        // No podemos mejorar mas el fitness
        if current_solution.fitness() == 0.0{
            println!("Saved {} iterations", max_iterations - i);
            break;
        }
    }

    let first = crate::problem_datatypes::Point::from_vec(vec![1.23, 1.133, 2.22]);
    let second = crate::problem_datatypes::Point::from_vec(vec![5.0, 1.133, 2.22]);

    let distance = crate::problem_datatypes::Point::distance(&first, &second);
    println!("Distance is {:?}", distance);


    println!("La distancia global instracluster de la solucion es: {}", current_solution.global_cluster_mean_distance());

}
