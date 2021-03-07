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

use crate::problem_datatypes::ConstraintType;

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

    println!("Datos del problema cargados con exito, procediendo a calcular la solucion con busqueda local");
    println!("================================================================================");
    println!("");

    let max_iterations = 100000;
    //let max_iterations = 100; // TODO -- para debuggear

    // TODO -- read this from cli args
    let number_of_clusters = 7;

    let solucion_local = busqueda_local::run(data_points, constraints, number_of_clusters, max_iterations);
    println!("La distancia global instracluster de la solucion es: {}", solucion_local.global_cluster_mean_distance());
}
