use std::env;
use std::error::Error;
use std::boxed::Box;
use simple_error::bail; // Devuelve errores simples con un string descriptivo

/// Representa los parametros del programa
/// Estos son los que ha introducido el dato por la linea de comandos
#[derive(Debug)]
pub struct ProgramParameters{
    data_file: String,
    constraints_file: String,
    seed: i32
}

impl ProgramParameters{
    /// Toma los parametros de entrada por linea de comandos y los parsea a la
    /// estructura de datos
    pub fn new() -> Result<Self, Box<dyn Error>>{
        let args: Vec<String> = env::args().collect();

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

    pub fn get_data_file(&self) -> String{
        return self.data_file.clone();
    }

    pub fn get_constraints_file(&self) -> String{
        return self.constraints_file.clone();
    }

    pub fn get_seed(&self) -> i32{
        return self.seed;
    }
}
