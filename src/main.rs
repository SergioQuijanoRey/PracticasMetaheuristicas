use std::error::Error;
use std::boxed::Box;
use std::process::exit;
use std::collections::HashMap;
use csv;

// Ficheros en los que separo mi codigo
mod arg_parser;
mod file_parsers;
mod problem_datatypes;
mod busqueda_local;

fn show_help(){
    println!("Modo de uso del programa:");
    println!("\t./PracticasMetaheuristicas <data_file> <constraints_file> <seed> <number_of_clusters>")
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

    println!("Datos del problema cargados con exito, procediendo a calcular la solucion con busqueda local");
    println!("================================================================================");
    println!("");

    let max_iterations = 100000;
    let solucion_local = busqueda_local::run(data_points, constraints, program_arguments.get_number_of_clusters(), max_iterations, program_arguments.get_seed());

    println!("La distancia global instracluster de la solucion es: {}", solucion_local.global_cluster_mean_distance());
    println!("El numero de restricciones violadas es: {}", solucion_local.infeasibility());
    println!("El valor de fitness es: {}", solucion_local.fitness());
    eprintln!("El valor de lambda es: {}", solucion_local.get_lambda());
}
