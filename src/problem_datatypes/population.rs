use crate::problem_datatypes::Solution;
use crate::problem_datatypes::DataPoints;
use crate::problem_datatypes::Constraints;

use rand::Rng;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;

/// Representa una poblacion para los algoritmos geneticos
pub struct Population<'a, 'b>{
    /// Individuos de la poblacion
    individuals: Vec<Solution<'a, 'b> >,
}

/// Genera una poblacion aleatoria inicial
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

    /// Copia los datos de esta poblacion
    /// TODO -- es esto muy lento?
    pub fn copy(&self) -> Self{
        let mut new_individuals = vec![];

        for individual in &self.individuals{
            new_individuals.push(individual.copy());
        }

        return Self{
            individuals: new_individuals
        };
    }

    /// Devuelve el numero de individuos de nuestra poblacion
    pub fn population_size(&self) -> usize{
        return self.individuals.len();
    }

    pub fn get_individual(&self, index: usize) -> &Solution<'a, 'b>{
        return &self.individuals[index];
    }

    /// Devuelve la mejor solucion de la poblacion
    /// Debe haber al menos un individuo en la poblacion
    pub fn get_best_individual(&self) -> &Solution<'a, 'b>{

        // Comprobacion inicial de seguridad
        // TODO -- esto deberia ser debug_assert?
        assert!(self.population_size() > 0, "La poblacion no puede ser nula en get_best_individual");

        let mut best_fitness = self.individuals[0].fitness();
        let mut best_index = 0;

        for (index, individual) in self.individuals.iter().enumerate(){
            if individual.fitness() < best_fitness{
                best_index = index;
                best_fitness = individual.fitness();
            }
        }

        return self.get_individual(best_index);
    }

    /// Calcula el indice del individuo de la poblacion con peor fitness
    /// Debe haber al menos un individuo en la poblacion
    pub fn get_index_worst_individual(&self) -> usize{
        // Comprobacion inicial de seguridad
        // TODO -- esto deberia ser debug_assert?
        assert!(self.population_size() > 0, "La poblacion no puede ser nula en get_index_worst_individual");

        let mut worst_fitness = self.individuals[0].fitness();
        let mut worst_index = 0;

        for (index, individual) in self.individuals.iter().enumerate(){
            if individual.fitness() > worst_fitness{
                worst_index = index;
                worst_fitness = individual.fitness();
            }
        }

        return worst_index;
    }

    /// Modifica el individuo en una posicion dada
    /// 0 <= index < population_size para que no de errores
    pub fn set_individual(&mut self, index: usize, individual: Solution<'a, 'b>){
        self.individuals[index] = individual;
    }

    /// Genera, a partir de una poblacion, una nueva poblacion de seleccion de un tamaño dado a
    /// partir de repetir new_population_size veces un torneo binario
    pub fn select_population_binary_tournament(&self, new_population_size: i32, rng: &mut StdRng) -> Self{
        let mut new_pop = Self::new_empty_population();

        // Añadimos individuos usando el torneo binario
        for _ in 0..new_population_size{

            // Los dos individuos que van a competir en el torneo
            let first_candidate = self.individuals.choose(rng).expect("La poblacion no puede estar vacia para hacer el tornero binario");
            let second_candidate = self.individuals.choose(rng).expect("La poblacion no puede estar vacia para hacer el tornero binario");

            // Seleccionamos el ganador
            let winner = Solution::binary_tournament(first_candidate, second_candidate);
            new_pop.individuals.push(winner.copy());
        }

        return new_pop;
    }

    /// Genera una poblacion de cruce a partir de una poblacion (que deberia ser de seleccion, pues
    /// confiamos en que provenga de seleccion para que esto haya introducido ya la aleatoriedad)
    /// La nueva poblacion tiene el mismo tamaño que la poblacion original
    /// Se cruzan los primeros n elementos, este orden se considera aleatorio por venir de un
    /// proceso de seleccion, que introduce aleatoriedad, como ya hemos comentado
    pub fn cross_population_uniform(&self, crossover_probability: f64, rng: &mut StdRng) -> Self{
        // Partimos de una poblacion identica a la dada
        let mut new_population = self.copy();

        // Mutamos el numero de individuos que coincide con la esperanza matematica, para
        // ahorrarnos evaluaciones de los numeros aleatorios
        let inidividuals_to_cross = (crossover_probability * self.population_size() as f64 * 0.5) as usize;

        // Cruzamos los inidividuals_to_cross primeros individos
        let mut index = 0;
        while index < inidividuals_to_cross - 1{

            // Tomamos los dos padres
            let first_parent = new_population.individuals[index].copy();
            let second_parent = new_population.individuals[index + 1].copy();

            // Generamos los dos hijos usando los dos padres
            let first_child = Solution::uniform_cross(&first_parent, &second_parent, rng);
            let second_child = Solution::uniform_cross(&second_parent, &first_parent, rng);

            // Sustituimos los dos individuos
            new_population.individuals[index] = first_child;
            new_population.individuals[index + 1] = second_child;

            // Avanzamos la iteracion
            index = index + 2;
        }


        return new_population;
    }

    /// Mutamos una poblacion a partir de la poblacion que ya ha sido seleccionada y cruzada
    // TODO -- BUG -- deberiamos elegir los elementos de la poblacion aleatoriamente
    pub fn mutate_population(&self, individuals_to_mutate: i32, rng: &mut StdRng) -> Self{
        let mut new_pop = self.copy();

        // Mutamos los primeros individuals_to_mutate elementos:
        for index in 0..individuals_to_mutate as usize{
            new_pop.individuals[index] = new_pop.individuals[index].mutated(rng);
        }

        return new_pop;
    }

    // Itera sobre todos los individuos. Los individuos que son solucion no valida, son reparados
    // TODO -- BUG -- borrar esto porque no deberia hacernos falta
    pub fn repair_bad_individuals(&mut self, rng: &mut StdRng){
        for individual in &mut self.individuals{
            if individual.is_valid() == false{
                individual.repair_solution(rng);
            }

        }

    }
}
