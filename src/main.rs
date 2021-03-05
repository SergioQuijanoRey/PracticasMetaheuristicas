use std::env;
use std::error::Error;
use std::boxed::Box;
use std::process::exit;
use simple_error::bail;

/// Representa los parametros del programa
/// Estos son los que ha introducido el dato por la linea de comandos
#[derive(Debug)]
struct ProgramParameters{
    data_file: String,
    constraints_file: String,
    seed: i32
}

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

fn main() {
    let args: Vec<String> = env::args().collect();
    let program_arguments = match parse_args(args){
        Ok(value) => value,
        Err(err) => {
            eprintln!("No se pudo leer los parametros dados por terminal");
            eprintln!("[Errcode]: {}", err);
            exit(-1);
        }
    };

    println!("{:?}", program_arguments);
}
