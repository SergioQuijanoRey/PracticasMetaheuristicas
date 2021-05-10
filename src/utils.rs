/// Funciones auxiliares

use crate::arg_parser::SearchType;
use chrono::{DateTime, Utc};


/// Genera un nombre de fichero para los FitnessEvolution que guardamos. El nombre del fichero sera
/// la carpeta, mas el tipo de busqueda, mas la fecha en la que se genero el archivo
pub fn generate_file_name(search_type: SearchType) -> String{
    let dir_path = "./fitness_evolution_data/".to_string();
    let search_type = format!("{:?}", search_type);
    let timestamp = format!("{}", Utc::now().format("%Y-%M-%d--%H:%M:%S"));

    return format!("{}/{}--{}", dir_path, search_type, timestamp);
}
