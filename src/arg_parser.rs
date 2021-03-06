use std::env;
use std::error::Error;
use std::boxed::Box;
use simple_error::bail; // Devuelve errores simples con un string descriptivo

/// Tipo de busqueda que el usuario quiere ejecutar
#[derive(Debug, Clone, Copy)]
pub enum SearchType{
    Copkmeans,
    CopkmeansRobust,

    LocalSearch,

    GenerationalGeneticUniform,
    GenerationalGeneticSegment,
    SteadyGeneticUniform,
    SteadyGeneticSegment,

    MemeticAll,
    MemeticRandom,
    MemeticElitist,

    MultiStartLocalSearch,

    IterativeLocalSearchBasic,
    IterativeLocalSearchSimulatedAnnealing,

    SimulatedAnnealing,
}

impl SearchType{
    /// Toma un string con el tipo de busqueda y lo convierte al struct
    /// En el codigo se pueden ver los valores validos para el codigo
    // TODO -- refactor, usar un HashMap para no tener este bloque de ifs
    pub fn from_str(code: &str) -> Result<Self, Box<dyn Error>>{
        if code == "copkmeans"{
            return Ok(SearchType::Copkmeans);
        }

        if code == "copkmeans_robust"{
            return Ok(SearchType::CopkmeansRobust);
        }

        if code == "local_search"{
            return Ok(SearchType::LocalSearch);
        }

        if code == "gguniform"{
            return Ok(SearchType::GenerationalGeneticUniform);
        }

        if code == "ggsegment"{
            return Ok(SearchType::GenerationalGeneticSegment);

        }

        if code == "gsuniform"{
            return Ok(SearchType::SteadyGeneticUniform);
        }

        if code == "gssegment"{
            return Ok(SearchType::SteadyGeneticSegment);
        }

        if code == "memeall"{
            return Ok(SearchType::MemeticAll);
        }

        if code == "memerandom"{
            return Ok(SearchType::MemeticRandom)
        }

        if code == "memeelitist"{
            return Ok(SearchType::MemeticElitist);
        }

        if code == "multistartlocalsearch" {
            return Ok(SearchType::MultiStartLocalSearch);
        }

        if code == "iterative_local_search"{
            return Ok(SearchType::IterativeLocalSearchBasic);
        }

        if code == "iterative_local_search_annealing"{
            return Ok(SearchType::IterativeLocalSearchSimulatedAnnealing);
        }

        if code == "simulated_annealing"{
            return Ok(SearchType::SimulatedAnnealing);
        }

        // Codigo no valido
        bail!("Valor del string para seleccionar la busqueda no valido");
    }
}

/// Representa los parametros del programa
/// Estos son los que ha introducido el dato por la linea de comandos
#[derive(Debug)]
pub struct ProgramParameters{
    data_file: String,
    constraints_file: String,
    seed: u64,
    number_of_clusters: i32,
    search_type: SearchType,
}

impl ProgramParameters{
    /// Toma los parametros de entrada por linea de comandos y los parsea a la
    /// estructura de datos
    pub fn new() -> Result<Self, Box<dyn Error>>{
        // Tomamos los argumentos pasados por la linea de comandos
        let args: Vec<String> = env::args().collect();

        if args.len() != 6{
            bail!("5 parameters expected, {} given", args.len() - 1)
        }

        let data_file = args[1].parse::<String>()?;
        let constraints_file = args[2].parse::<String>()?;
        let seed = args[3].parse::<u64>()?;
        let number_of_clusters = args[4].parse::<i32>()?;
        let search_type = args[5].parse::<String>()?;
        let search_type = SearchType::from_str(&search_type)?;

        return Ok(ProgramParameters{
            data_file, constraints_file, seed, number_of_clusters, search_type
        });
    }

    pub fn get_data_file(&self) -> String{
        return self.data_file.clone();
    }

    pub fn get_constraints_file(&self) -> String{
        return self.constraints_file.clone();
    }

    pub fn get_seed(&self) -> u64{
        return self.seed;
    }

    pub fn get_number_of_clusters(&self) -> i32{
        return self.number_of_clusters;
    }

    pub fn get_search_type(&self) -> SearchType{
        return self.search_type;
    }
}
