use std::env;
use std::error::Error;
use std::boxed::Box;
use std::process::exit;
use simple_error::bail; // Devuelve errores simples con un string descriptivo
use csv;

/// Representa los parametros del programa
/// Estos son los que ha introducido el dato por la linea de comandos
#[derive(Debug)]
struct ProgramParameters{
    data_file: String,
    constraints_file: String,
    seed: i32
}

#[derive(Debug)]
struct DataPoints{
    points: Vec<Point>
}

#[derive(Debug)]
struct Point{
    coordinates: Vec<f32>
}

/// Toma los parametros de entrada por linea de comandos y los parsea a la
/// estructura de datos
fn parse_args(args: Vec<String>) -> Result<ProgramParameters, Box<dyn Error>>{
    if args.len() != 4{
        bail!("3 parameters expected, {} given", args.len() - 1)
    }

    let data_file = args[1].parse::<String>()?;
    let constraints_file = args[2].parse::<String>()?;
    let seed = args[3].parse::<i32>()?;

    return Ok(ProgramParameters{
        data_file, constraints_file, seed
    });

}

/// Toma un fichero de datos y los parsea a la estructura de datos
fn parse_data_file_to_struct(data_path: &str) -> Result<DataPoints, Box<dyn Error>>{
    // Tiene que ser mutable para poder iterar
    // El proceso de tomar el siguiente elemento se considera una mutacion de
    // la variable
    let mut reader = csv::Reader::from_path(data_path)?;
    let mut points: Vec<Point> = vec![];

    for current_line in reader.records(){
        // Unwrap el result
        let current_line = current_line?;

        // Mapear StringRecord a un vector de floats
        let data: Vec<&str> = current_line.iter().collect();
        let data: Vec<String> = data.into_iter().map(|x| x.to_string()).collect();

        // Si algun elemento no se puede parsear, se devuelve un error
        // En otro caso, me tomo los datos
        let data: Result<Vec<f32>, _> = data.into_iter().map(|x| x.parse::<f32>()).collect();
        let data = data?;

        // Añadir el punto al vector de puntos
        let point = Point{coordinates: data};
        points.push(point)
    }

    return Ok(DataPoints{points});
}

fn show_help(){
    println!("Modo de uso del programa:");
    println!("\t./PracticasMetaheuristicas <data_file> <constraints_file> <seed>")
}

fn main() {

    // Tomamos los datos de entrada por linea de comandos
    let args: Vec<String> = env::args().collect();
    let program_arguments = match parse_args(args){
        Ok(value) => value,
        Err(err) => {
            eprintln!("No se pudo leer los parametros dados por terminal");
            eprintln!("[Errcode]: {}", err);
            show_help();
            exit(-1);
        }
    };

    // Parseamos los datos del archivo de datos
    let data_points = match parse_data_file_to_struct(&program_arguments.data_file){
        Ok(value) => value,
        Err(err) => {
            eprintln!("No se pudieron leer los datos del fichero {}", program_arguments.data_file);
            eprintln!("[Errcode]: {}", err);
            exit(-1);
        }
    };

    println!("Data points: {:?}", data_points);
}
