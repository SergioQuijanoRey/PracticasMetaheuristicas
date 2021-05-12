use crate::problem_datatypes::Solution;
use crate::problem_datatypes::DataPoints;
use crate::problem_datatypes::Constraints;
use crate::fitness_evaluation_result::FitnessEvaluationResult;
use crate::arg_parser::SearchType;

use rand::Rng;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use std::io::{stdin, stdout, Read, Write};

// Para usar una cola con prioridad
use priority_queue::PriorityQueue;
use ordered_float::OrderedFloat;

use std::collections::HashSet;


/// Representa una poblacion para los algoritmos geneticos
#[derive(Debug, Clone)]
pub struct Population<'a, 'b>{
    /// Individuos de la poblacion
    individuals: Vec<Solution<'a, 'b> >,
}

/// Implementacion para la parte de los algoritmos geneticos
impl<'a, 'b> Population<'a, 'b>{

    /// Genera una poblacion vacia, sin individuos
    pub fn new_empty_population() -> Self{
        return Self{
            individuals: vec![]
        };
    }

    /// Genera una nueva poblacion aleatoria
    pub fn new_random_population(data_points: &'a DataPoints, constraints: &'b Constraints, number_of_clusters: i32, population_size: i32, rng: &mut StdRng) -> Self{
        let mut rand_population = Self{individuals: vec![]};

        // Añadimos las soluciones aleatorias a la poblacion
        for _ in 0..population_size{
            let new_individual = Solution::generate_random_solution(data_points, constraints, number_of_clusters, rng);
            rand_population.individuals.push(new_individual);
        }

        return rand_population;
    }

    /// Devuelve el numero de individuos de nuestra poblacion
    pub fn population_size(&self) -> usize{
        return self.individuals.len();
    }

    pub fn get_individual(&self, index: usize) -> &Solution<'a, 'b>{
        return &self.individuals[index];
    }

    /// Devuelve la mejor solucion de la poblacion y el indice en el que se encuentra
    /// Debe haber al menos un individuo en la poblacion
    pub fn get_best_individual(&self) -> FitnessEvaluationResult<(&Solution<'a, 'b>, u32)>{

        let mut fit_eval_consumed = 0;

        // Comprobacion inicial de seguridad
        assert!(self.population_size() > 0, "La poblacion no puede ser nula en get_best_individual");

        let (mut best_fitness, fit_cons) = self.individuals[0].fitness_and_consumed();
        fit_eval_consumed += fit_cons;
        let mut best_index = 0;

        for (index, individual) in self.individuals.iter().enumerate(){
            let (individual_fitness, fit_cons) = individual.fitness_and_consumed();
            fit_eval_consumed += fit_cons;
            if individual_fitness < best_fitness{
                best_index = index;
                best_fitness = individual.fitness();
            }
        }

        return FitnessEvaluationResult::new((self.get_individual(best_index), best_index as u32), fit_eval_consumed);
    }

    /// Calcula el indice del individuo de la poblacion con peor fitness
    /// Debe haber al menos un individuo en la poblacion
    pub fn get_index_worst_individual(&self) -> FitnessEvaluationResult<usize>{
        // Comprobacion inicial de seguridad
        // TODO -- esto deberia ser debug_assert?
        debug_assert!(self.population_size() > 0, "La poblacion no puede ser nula en get_index_worst_individual");

        let mut fit_eval_consumed = 0;

        let (mut worst_fitness, fit_cons) = self.individuals[0].fitness_and_consumed();
        fit_eval_consumed += fit_cons;
        let mut worst_index = 0;

        for (index, individual) in self.individuals.iter().enumerate(){
            let (individual_fitness, fit_cons) = individual.fitness_and_consumed();
            fit_eval_consumed += fit_cons;

            if individual_fitness > worst_fitness{
                worst_index = index;
                worst_fitness = individual_fitness;
            }
        }

        return FitnessEvaluationResult::new(worst_index, fit_eval_consumed);
    }

    /// Modifica el individuo en una posicion dada
    /// 0 <= index < population_size para que no de errores
    pub fn set_individual(&mut self, index: usize, individual: Solution<'a, 'b>){
        self.individuals[index] = individual;
    }

    /// Genera, a partir de una poblacion, una nueva poblacion de seleccion de un tamaño dado a
    /// partir de repetir new_population_size veces un torneo binario
    /// Los valores comunes para new_population_size son o bien el tamaño de la poblacion pasada o
    /// bien 2, para el modelo estacionario
    pub fn select_population_binary_tournament(&self, new_population_size: i32, rng: &mut StdRng) -> FitnessEvaluationResult<Self>{
        let mut new_pop = Self::new_empty_population();
        let mut fit_ev_consumed = 0;

        // Añadimos individuos usando el torneo binario
        for _ in 0..new_population_size{

            // Los dos individuos que van a competir en el torneo
            let first_candidate = self.individuals.choose(rng).expect("La poblacion no puede estar vacia para hacer el tornero binario");
            let second_candidate = self.individuals.choose(rng).expect("La poblacion no puede estar vacia para hacer el tornero binario");

            // Seleccionamos el ganador
            let (winner, fit_consumed) = Solution::binary_tournament(first_candidate, second_candidate);
            new_pop.individuals.push(winner.clone());
            fit_ev_consumed += fit_consumed;
        }

        return FitnessEvaluationResult::new(new_pop, fit_ev_consumed);
    }

    /// Genera una poblacion de cruce a partir de una poblacion (que deberia ser de seleccion, pues
    /// confiamos en que provenga de seleccion para que esto haya introducido ya la aleatoriedad)
    /// La nueva poblacion tiene el mismo tamaño que la poblacion original
    /// Se cruzan los primeros n elementos, este orden se considera aleatorio por venir de un
    /// proceso de seleccion, que introduce aleatoriedad, como ya hemos comentado
    pub fn cross_population_uniform(&self, crossover_probability: f64, rng: &mut StdRng) -> FitnessEvaluationResult<Self>{
        // Partimos de una poblacion identica a la dada
        let mut new_population = self.clone();

        // Mutamos el numero de individuos que coincide con la esperanza matematica, para
        // ahorrarnos evaluaciones de los numeros aleatorios
        let inidividuals_to_cross = (crossover_probability * self.population_size() as f64) as usize;

        // Cruzamos los inidividuals_to_cross primeros individos
        let mut index = 0;
        while index < inidividuals_to_cross - 1{

            // Tomamos los dos padres
            let first_parent = new_population.individuals[index].clone();
            let second_parent = new_population.individuals[index + 1].clone();

            // Generamos los dos hijos usando los dos padres
            let first_child = Solution::uniform_cross(&first_parent, &second_parent, rng);
            let second_child = Solution::uniform_cross(&second_parent, &first_parent, rng);

            // Sustituimos los dos individuos
            new_population.individuals[index] = first_child;
            new_population.individuals[index + 1] = second_child;

            // Avanzamos la iteracion
            index = index + 2;
        }

        // En esta parte, directamente no estamos haciendo evaluaciones del fitness
        let fit_evals_consumed = 0;
        return FitnessEvaluationResult::new(new_population, fit_evals_consumed);
    }

    /// Genera una poblacion de cruce a partir de una poblacion (que deberia ser de seleccion, pues
    /// confiamos en que provenga de seleccion para que esto haya introducido ya la aleatoriedad)
    /// La nueva poblacion tiene el mismo tamaño que la poblacion original
    /// Se cruzan los primeros n elementos, este orden se considera aleatorio por venir de un
    /// proceso de seleccion, que introduce aleatoriedad, como ya hemos comentado
    pub fn cross_population_segment(&self, crossover_probability: f64, rng: &mut StdRng) -> FitnessEvaluationResult<Self>{
        // Partimos de una poblacion identica a la dada
        let mut new_population = self.clone();

        // Mutamos el numero de individuos que coincide con la esperanza matematica, para
        // ahorrarnos evaluaciones de los numeros aleatorios
        let inidividuals_to_cross = (crossover_probability * self.population_size() as f64) as usize;

        // Cruzamos los inidividuals_to_cross primeros individos
        let mut index = 0;
        while index < inidividuals_to_cross - 1{

            // Tomamos los dos padres
            let first_parent = new_population.individuals[index].clone();
            let second_parent = new_population.individuals[index + 1].clone();

            // Generamos los dos hijos usando los dos padres
            let first_child = Solution::cross_segment(&first_parent, &second_parent, rng);
            let second_child = Solution::cross_segment(&second_parent, &first_parent, rng);

            // Sustituimos los dos individuos
            new_population.individuals[index] = first_child;
            new_population.individuals[index + 1] = second_child;

            // Avanzamos la iteracion
            index = index + 2;
        }

        // En esta parte, directamente no estamos haciendo evaluaciones del fitness
        let fit_evals_consumed = 0;
        return FitnessEvaluationResult::new(new_population, fit_evals_consumed);

    }

    /// Mutamos una poblacion a partir de la poblacion que ya ha sido seleccionada y cruzada
    /// Esta operacion no consume evaluaciones del fitness
    /// Usamos la esperanza matematicas para no gastar tantas tiradas aleatorias, por lo que en vez
    /// de pasar la probabilida de mutacion, pasamos el numero de individuos a mutar
    /// Notar que un individuo puede mutar mas de una vez
    pub fn mutate_population(&self, individuals_to_mutate: i32, rng: &mut StdRng) -> Self{
        let mut new_pop = self.clone();

        // Posiciones sobre las que podemos elegir aleatoriamente
        let positions: Vec<usize> = (0..self.population_size() as usize).collect();

        for _ in 0..individuals_to_mutate as usize{
            let random_index = *positions.choose(rng).expect("No se ha podido escoger un valor aleatorio");
            new_pop.individuals[random_index] = new_pop.individuals[random_index].mutated(rng);
        }

        return new_pop;
    }

    /// Mutamos una poblacion a partir de la poblacion que ya ha sido seleccionada y cruzada
    /// Esta operacion no consume iteraciones sobre la poblacion
    /// A diferencia de mutate_population, no usamos el numero esperado de mutaciones, sino tiradas
    /// aleatorias. Por ello, la poblacion con la que trabajamos no debiera ser demasiado grande
    pub fn mutate_population_given_prob(&self, mutation_probability_per_gen: f64, rng: &mut StdRng) -> Self{
        let mut new_pop = self.clone();

        // Iteramos sobre los individuos y decidimos si mutamos o no aleatoriamente
        for (index, _individual) in self.individuals.iter().enumerate(){

            // Iteramos sobre los genes. Realmente lanzamos numero_genes de veces los aleatorios,
            // pues estamos trabajando con probabilidad por gen
            for _ in 0..self.individuals[0].get_cluster_indexes().len(){
                let do_mutation = rng.gen::<f64>() <= mutation_probability_per_gen;
                if do_mutation == true{
                    new_pop.individuals[index] = new_pop.individuals[index].mutated(rng);
                }
            }
        }


        return new_pop;
    }

    /// Dada una poblacion original, comprueba si el mejor individuo de la poblacion original esta
    /// en esta poblacion. En caso de que no este, se introduce en la nueva poblacion, en la
    /// posicion en la que estaba en la poblacion original
    pub fn preserve_best_past_parent(&self, original_population: &Population<'a, 'b>) -> FitnessEvaluationResult<Self>{
        let mut new_pop = self.clone();
        let mut fit_eval_cons = 0;

        // Tomamos el mejor individuo de la poblacion original
        // Añadimos las iteraciones que consume esto, deberian ser cero pues esa poblacion ya
        // deberia estar evaluada
        let best_individual_at_original_pop_result= original_population.get_best_individual();
        let (best_individual_at_original_pop, best_individual_index_original_pop) = best_individual_at_original_pop_result.get_result();
        fit_eval_cons += best_individual_at_original_pop_result.get_iterations_consumed();

        // Comprobamos si esta dentro de la poblacion
        // Esta operacion no consume iteraciones, porque solo estamos comprobando la igualdad entre
        // vectores de posiciones
        let search_result = self.search_individual_with_same_cluster_indixes(best_individual_at_original_pop.get_cluster_indexes());
        match search_result{
            // El mejor individuo pasado ha sobrevivido, devolvemos la poblacion sin modificar
            // junto a las evaluaciones consumidas
            Some(_) => return FitnessEvaluationResult::new(self.clone(), fit_eval_cons),

            // No hemos encontrado el individuo, no hacemos nada, por lo que seguimos con el
            // proceso de incluir el mejor individuo pasado en la poblacion
            None => (),
        };

        // El mejor individuo pasado no esta en la nueva poblacion, lo introducimos en su posicion
        // de la poblacion original en la nueva poblacion
        new_pop.individuals[*best_individual_index_original_pop as usize] = (*best_individual_at_original_pop).clone();
        return FitnessEvaluationResult::new(new_pop, fit_eval_cons);
    }

    /// Busca el individuo en la poblacion con la misma asignacion de cluster
    fn search_individual_with_same_cluster_indixes(&self, cluster_indixes: Vec<u32>) -> Option<u32>{
        // Realizamos la busqueda
        for (index, individual) in self.individuals.iter().enumerate(){
            if individual.get_cluster_indexes() == cluster_indixes{
                return Some(index as u32)
            }
        }

        // No se ha encontrado el elemento buscado
        return None;

    }

    // Dada una poblacion original y una nueva poblacion candidata, los individuos de la poblacion
    // candidata luchan contra los peores individuos de la poblacion original (&self) para quedarse
    // en dicha poblacion
    // La poblacion original no se modifica, se devuelve una copia con la poblacion resultante
    pub fn compete_with_new_individuals(&self, candidate_population: &Population<'a, 'b>) -> FitnessEvaluationResult<Self>{
        let mut new_pop = self.clone();
        let mut fit_eval_cons = 0;

        for candidate in candidate_population.individuals.iter(){

            // Tomamos el peor individuo de la poblacion
            let worst_individual_result = new_pop.get_index_worst_individual();
            let worst_individual_index = worst_individual_result.get_result();
            fit_eval_cons += worst_individual_result.get_iterations_consumed();

            // Evaluamos el fitness del peor individuo y del candidato. En ambos casos tenemos en
            // cuenta las evaluaciones que esto puede suponer. El peor individuo deberia estar
            // evaluado, mientras que el candidato no. Hacemos las dos cuentas por seguridad
            let (worst_fitness, worst_it_cons) = new_pop.individuals[*worst_individual_index].fitness_and_consumed();
            let (candidate_fitness, candidate_it_cons) = candidate.fitness_and_consumed();
            fit_eval_cons += worst_it_cons + candidate_it_cons;

            // Comprobacion de seguridad
            debug_assert!(candidate_it_cons == 1, "El candidato debe tener el fitness sin evaluar, el valor de consumiciones es {}", candidate_it_cons);

            // Decidimos si el candidato entra o no en la poblacion
            if candidate_fitness < worst_fitness {
                new_pop.individuals[*worst_individual_index] = candidate.clone();
            }
        }

        return FitnessEvaluationResult::new(new_pop, fit_eval_cons);
    }

    /// Evaluamos a todos los individuos de la poblacion
    /// Devolvemos las evaluaciones de fitness consumidas. Potencialmente sera un valor alto, pues
    /// llegamos con una poblacion nueva, que ha sido en parte cruzada y mutada. A este valor solo
    /// contribuyen los individuos nuevos. Los de la poblacion pasada, que no han cambiado, no
    /// contribuyen
    ///
    /// Notar que los elementos mutan, pero al estar usando un patron de mutabilidad interior, no
    /// tenemos un patron de mutabilidad interior, no hace falta pasar una referencia mutable
    pub fn evaluate_all_individuals(&self) -> FitnessEvaluationResult<()>{
        let mut fit_evals_consumed = 0;

        for individual in &self.individuals{
            let (_, ev_consumed) = individual.fitness_and_consumed();
            fit_evals_consumed += ev_consumed;

        }

        return FitnessEvaluationResult::new((), fit_evals_consumed);
    }

    /// Comprueba si todos los individuos de una poblacion tienen todos los valores del fitness sin
    /// calcular. Lo usamos para debuggear la poblacion de candidatos en genetico estacionario
    pub fn all_population_is_not_cached(&self) -> bool{
        for individual in self.individuals.iter(){

            // Un individuo tiene el valor del fitness cacheado
            if individual.is_fitness_cached() == true{
                return false;
            }
        }

        // Todos los individuos tienen el fitness sin precalcular
        return true;
    }

    /// Comprobamos que todos los individuos de la poblacion tengan el valor del fitness cacheado
    /// Notar que no es lo mismo que comprobar self.all_population_is_not_cached == false
    pub fn all_population_is_cached(&self) -> bool{
        for individual in self.individuals.iter(){

            // Un individuo tiene el valor del fitness cacheado
            if individual.is_fitness_cached() == false{
                return false;
            }
        }

        // Todos los individuos tienen el fitness cacheado
        return true;
    }

    /// Muestra las asignaciones de clusters de los individuos de la poblacion
    /// Lo usamos para debuggear el codigo, porque nuestra poblacion converge demasiado rapido a
    /// repetir el mismo individuo
    pub fn show_population(&self){
        let max_values_in_row = 30;

        for (index, individual) in self.individuals.iter().enumerate(){
            print!("{}:\t", index);
            for col in 0..max_values_in_row{
                print!("{} ", individual.get_cluster_indexes()[col]);
            }
            println!("");
        }

        // Esperamos a que el usuario pulse una tecla
        Population::wait_for_user_input();
    }

    fn wait_for_user_input() {
        let mut stdout = stdout();
        stdout.write(b"Press Enter to continue...").unwrap();
        stdout.flush().unwrap();
        stdin().read(&mut [0]).unwrap();
    }

    /// Intentamos medir la variedad que tenemos en nuestra poblacion. La variedad se medira como
    /// el numero de elementos con distinto valor de fitness
    /// WARNING -- Lo usamos para debuggear el codigo. No usar en codigo final porque puede evaluar
    /// una poblacion sin considerar las evaluaciones del fitness
    // TODO -- poner un panic para ver que no se use en codigo final
    pub fn measure_variety(&self) -> u32{
        // Para comprobar que no podemos llamar a esta funcion
        // Quitar esto cuando hagamos debug del codigo. Tenemos un test para comprobar que esta
        // funcion no puede ser llamada
        panic!("function disabled");

        let mut fitness_values = HashSet::new();

        for individual in self.individuals.iter(){
            fitness_values.insert(OrderedFloat::<f64>::from(individual.fitness()));
        }

        return fitness_values.len() as u32;
    }

}

/// Implementacion para la parte de algoritmos memeticos
impl<'a, 'b> Population<'a, 'b>{
    /// Aplica la busqueda local suave, segun el criterio indicado por memetic_type, a la
    /// poblacion, generando una nueva poblacion
    pub fn soft_local_search(&self, memetic_type: SearchType, max_fails: i32, rng: &mut StdRng) -> FitnessEvaluationResult<Self>{
        // Lanzamos la busqueda local suave correspondiente
        match memetic_type{
            SearchType::MemeticAll => {
                return self.soft_local_search_all(max_fails, rng);
            }

            SearchType::MemeticRandom => {
                let search_percentage = 0.1;
                return self.soft_local_search_random(max_fails, search_percentage, rng);
            }

            SearchType::MemeticElitist => {
                let search_percentage = 0.1;
                return self.soft_local_search_elitist(max_fails, search_percentage, rng);
            }

            _ => {
                panic!("Valor erroneo para memetic_type")
            }
        }
    }

    // Aplica la busqueda local suave, sobre todos los individuos de la poblacion
    fn soft_local_search_all(&self, max_fails: i32, rng: &mut StdRng) -> FitnessEvaluationResult<Self>{
        let mut new_pop = self.clone();
        let mut fit_eval_cons = 0;

        // Aplicamos la busqueda local suave a todos los individuos de la poblacion
        // Itero sobre indices, asi que puedo iterar sobre self para no tener problemas de
        // mutabilidad con new_pop
        for (index, _individual) in self.individuals.iter().enumerate(){
            let new_individual_result = new_pop.individuals[index].soft_local_search(max_fails, rng);
            let new_individual = new_individual_result.get_result();
            fit_eval_cons += new_individual_result.get_iterations_consumed();
            new_pop.individuals[index] = new_individual.clone();
        }

        return FitnessEvaluationResult::new(new_pop, fit_eval_cons);
    }

    // Aplica la busqueda local suave, sobre un porcentaje de individuos aleatorios de la poblacion
    fn soft_local_search_random(&self, max_fails: i32, search_percentage: f64, rng: &mut StdRng) -> FitnessEvaluationResult<Self>{
        let mut new_pop = self.clone();
        let mut fit_eval_cons = 0;

        // Numero de individuos sobre los que vamos a realizar la busqueda local suave
        let number_of_individuals_to_intensify = (self.individuals.len() as f64 * search_percentage) as i32;

        // Indices de todos los individuos ordenados aleatoriamente
        let mut individuals_indixes: Vec<u32> = (0..self.individuals.len() as u32).collect();
        individuals_indixes.shuffle(rng);

        // Aplicamos la busqueda local solo a un numero dado de los individuos. Usando los indices
        // en orden aleatorio, escogemos aleatoriamente a dichos individuos
        for i in 0..number_of_individuals_to_intensify{
            // Escogemos aleatoriamente al individuo
            let index = individuals_indixes[i as usize] as usize;

            // Aplicamos la busqueda local a ese individuo
            let new_individual_result = new_pop.individuals[index].soft_local_search(max_fails, rng);
            let new_individual = new_individual_result.get_result();
            fit_eval_cons += new_individual_result.get_iterations_consumed();
            new_pop.individuals[index] = new_individual.clone();
        }

        return FitnessEvaluationResult::new(new_pop, fit_eval_cons);
    }

    // Aplica la busqueda local suave, sobre el mejor porcentaje de individuos de la poblacion
    fn soft_local_search_elitist(&self, max_fails: i32, search_percentage: f64, rng: &mut StdRng) -> FitnessEvaluationResult<Self>{
        let mut new_pop = self.clone();
        let mut fit_eval_cons = 0;

        // Numero de individuos sobre los que vamos a realizar la busqueda local suave
        let number_of_individuals_to_intensify = (self.individuals.len() as f64 * search_percentage) as i32;

        // Seleccionamos los indices del mejor porcentaje de la poblacion
        let best_indixes_result = self.select_best_indixes(number_of_individuals_to_intensify);
        let best_indixes = best_indixes_result.get_result();
        fit_eval_cons += best_indixes_result.get_iterations_consumed();

        // Aplicamos la busqueda local a este porcentaje mejor de individuos
        for index in best_indixes{

            // Aplicamos la busqueda local a ese individuo
            let new_individual_result = new_pop.individuals[*index as usize].soft_local_search(max_fails, rng);
            let new_individual = new_individual_result.get_result();
            fit_eval_cons += new_individual_result.get_iterations_consumed();
            new_pop.individuals[*index as usize] = new_individual.clone();
        }

        return FitnessEvaluationResult::new(new_pop, fit_eval_cons);
    }

    /// Dado un numero de individuos, selecciona los indices de los mejores individuos de la
    /// poblacion. Es decir, aquellos indices de individuos con mejor valor de fitness
    // TODO -- es bastante facil de testear
    fn select_best_indixes(&self, number_of_individuals: i32) -> FitnessEvaluationResult<Vec<u32>>{
        let mut fit_evals_cons = 0;
        let mut best_indixes = vec![];

        // Necesitamos que toda la poblacion este evaluada para poder ordenar a sus individuos
        let eval_result = self.evaluate_all_individuals();
        fit_evals_cons += eval_result.get_iterations_consumed();
        debug_assert!(self.all_population_is_cached());

        // Guardamos a los individuos de la poblacion en una priority queue.
        // En verdad, solo necesitamos guardar los indices de los individuos ordenados por su valor
        // de fitness.
        // Podemos guardar todos los elementos en esta, sabiendo que el fitness ya esta evaluado
        // porque hemos evaluado toda la poblacion anteriormente.
        let mut priority_queue = PriorityQueue::new();
        for (index, individual) in self.individuals.iter().enumerate(){
            // -1.0 * porque asi devolvemos los mejores individuos, y no los peores
            priority_queue.push(index, OrderedFloat::<f64>::from(-1.0 * individual.fitness()));
        }

        // Sacamos a los number_of_individuals primeros individuos de la prioqueu
        for (it, index) in priority_queue.into_sorted_iter().enumerate(){
            // Tomamos el valor del indice. index tiene el valor del indice, y su valor de fitness,
            // que no nos interesa
            best_indixes.push(index.0 as u32);

            // Solo guardamos el numero dado de individuos
            if it as i32 >= number_of_individuals - 1{
                break;
            }
        }

        // Comprobacion de seguridad
        debug_assert!(best_indixes.len() == number_of_individuals as usize);

        return FitnessEvaluationResult::new(best_indixes, fit_evals_cons);
    }
}

mod test{
    use crate::problem_datatypes::population::Population;

    #[test]
    #[should_panic(expected="function disabled")]
    /// Comprobamos que tenemos desactivada esta funcion, porque si se puede llamar puede afectar
    /// al comportamiento de los algoritmos (pues evalua el fitness de todos los elementos de la
    /// poblacion sin notifcar cuantas evaluaciones efectivas del fitness estamos realizando)
    fn test_measure_variety_disabled(){
        let pop = Population::new_empty_population();
        pop.measure_variety();
    }
}
