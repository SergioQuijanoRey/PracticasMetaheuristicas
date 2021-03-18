use std::process::exit;
use std::time::{Instant, Duration};

// Ficheros en los que separo mi codigo
mod arg_parser;
mod file_parsers;
mod problem_datatypes;
mod local_search;
mod copkmeans;

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

    // Realizamos la busqueda local
    println!("Datos del problema cargados con exito, procediendo a realizar las busquedas");
    println!("================================================================================");
    println!("");

    // Realizamos la busqueda greedy
    // Si devuelve None, es porque la generacion aleatoria de centroides ha dejado
    // clusters sin elementos, y hay que repetir el algoritmo
    // De momento, no estoy contabilizando el tiempo perdido por esa situacion
    //
    // TODO -- preguntar si hay que contabilizar el tiempo que perdemos cuando la primera solucion
    // aleatoria nos genera clusters vacios o simplemente considerar el tiempo de la ejecucion
    // buena del algoritmo
    println!("Corriendo busqueda greedy");
    let mut greedy_solution: Option<problem_datatypes::Solution>;
    let mut duration_numeric;
    loop {
        let before = Instant::now();
        greedy_solution = copkmeans::run(&data_points, &constraints, program_arguments.get_number_of_clusters(), program_arguments.get_seed());
        let after = Instant::now();
        let duration = after.duration_since(before);
        duration_numeric = duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9;

        match greedy_solution {
            // Hemos contrado solucion, paramos de iterar
            Some(_) => break,

            // No hemos encontrado solucion, por lo que no hacemos nada, lo que provoca que sigamos
            // iterando
            None => (),
        }
    }

    // Tomamos la solucion del Option
    let greedy_solution = greedy_solution.expect("En el bucle anterior nos aseguramos de que no seas None");

    // Para que no sea mutable
    let duration_numeric = duration_numeric;

    // Mostramos los resultados
    println!("==> Busqueda greedy");
    println!("La distancia global instracluster de la solucion es: {}", greedy_solution.global_cluster_mean_distance());
    println!("El numero de restricciones violadas es: {}", greedy_solution.infeasibility());
    println!("El valor de fitness es: {}", greedy_solution.fitness());
    println!("El valor de lambda es: {}", greedy_solution.get_lambda());
    println!("Tiempo transcurrido (segundos): {}", duration_numeric);
    println!("");

    // Realizamos la busqueda local
    let max_iterations = 100000;

    println!("Corriendo busqueda local");
    let before = Instant::now();
    let solucion_local = local_search::run(&data_points, &constraints, program_arguments.get_number_of_clusters(), max_iterations, program_arguments.get_seed());
    let after = Instant::now();
    let duration = after.duration_since(before);
    let duration_numeric = duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9;

    println!("==> Busqueda local");
    println!("La distancia global instracluster de la solucion es: {}", solucion_local.global_cluster_mean_distance());
    println!("El numero de restricciones violadas es: {}", solucion_local.infeasibility());
    println!("El valor de fitness es: {}", solucion_local.fitness());
    println!("El valor de lambda es: {}", solucion_local.get_lambda());
    println!("Tiempo transcurrido (segundos): {}", duration_numeric);
    println!("");

}
