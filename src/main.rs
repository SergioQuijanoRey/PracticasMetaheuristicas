use std::process::exit;
use rand::SeedableRng;
use rand::rngs::StdRng;

// Ficheros en los que separo mi codigo
mod arg_parser;
mod file_parsers;
mod problem_datatypes;
mod algorithms;
mod fitness_evolution;
mod fitness_evaluation_result;

use algorithms::local_search;
use algorithms::copkmeans;
use algorithms::generational_genetic;
use algorithms::steady_genetic;
use algorithms::memetic;

fn show_help(){
    println!("Modo de uso del programa:");
    println!("\t./PracticasMetaheuristicas <data_file> <constraints_file> <seed> <number_of_clusters> <search_type>");
    println!("\t<search_type>: copkmeans | copkmeans_robust | local_search | gguniform | ggsegment | gsuniform | gssegment | memeall");
}

fn main() {

    // Argumentos del programa que recibimos de la terminal
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

    println!("Datos del problema cargados con exito, procediendo a realizar las busquedas");
    println!("\tData file: {}", program_arguments.get_data_file());
    println!("\tConstraints file: {}", program_arguments.get_constraints_file());
    println!("\tNumber of clusters: {}", program_arguments.get_number_of_clusters());
    println!("\tSeed: {}", program_arguments.get_seed());
    println!("\tSearch type: {:?}", program_arguments.get_search_type());
    println!("================================================================================");
    println!("");

    // Tomamos un generador de numeros aleatorios, que debe ser una referencia
    // mutable para poder generar numeros aleatorios
    let mut rng = StdRng::seed_from_u64(program_arguments.get_seed());

    // Miramos que busqueda quiere realizar el usuario
    match program_arguments.get_search_type(){
        arg_parser::SearchType::Copkmeans => {
            let robust = false;
            copkmeans::run_and_show_results(&data_points, &constraints, program_arguments, &mut rng, robust);
        }

        arg_parser::SearchType::CopkmeansRobust => {
            let robust = true;
            copkmeans::run_and_show_results(&data_points, &constraints, program_arguments, &mut rng, robust);
        }

        arg_parser::SearchType::LocalSearch => {
            local_search::run_and_show_results(&data_points, &constraints, program_arguments, &mut rng);
        }

        arg_parser::SearchType::GenerationalGeneticUniform => {
            let cross_uniform = true;
            generational_genetic::run_and_show_results(&data_points, &constraints, program_arguments, cross_uniform, &mut rng);
        }

        arg_parser::SearchType::GenerationalGeneticSegment => {
            let cross_uniform = false; // No usamos cruce uniforme, sino segmento fijo
            generational_genetic::run_and_show_results(&data_points, &constraints, program_arguments, cross_uniform, &mut rng);
        }

        arg_parser::SearchType::SteadyGeneticUniform => {
            let cross_uniform = true;
            steady_genetic::run_and_show_results(&data_points, &constraints, program_arguments, cross_uniform, &mut rng);

        }

        arg_parser::SearchType::SteadyGeneticSegment => {
            let cross_uniform = false;
            steady_genetic::run_and_show_results(&data_points, &constraints, program_arguments, cross_uniform, &mut rng);
        }

        arg_parser::SearchType::MemeticAll => {
            memetic::run_and_show_results(&data_points, &constraints, program_arguments, arg_parser::SearchType::MemeticAll, &mut rng);
        }
    }
}
