// Para fijar la semilla de numeros aleatorios
use rand::Rng;
use rand::rngs::StdRng;

// Para hacer shuffle de un vector
use rand::seq::SliceRandom;

// Para tener mutabilidad interior
use std::cell::RefCell;

use crate::problem_datatypes::{DataPoints, Constraints, Point, ConstraintType, NeighbourGenerator};
use crate::fitness_evaluation_result::FitnessEvaluationResult;

/// Estructura que representa una solucion del problema
///
/// La solucion viene representada como un vector de indices
/// En dicho vector, la posicion i-esima indica el cluster al que pertenece el i-esimo
/// punto del conjunto de datos
#[derive(Debug, Clone)]
pub struct Solution<'a, 'b> {
    cluster_indexes: Vec<u32>,
    data_points: &'a DataPoints,
    constraints: &'b Constraints,
    number_of_clusters: i32,

    /// Representa el peso de infeasibility en el calculo de fitness
    /// Solo se calcula una vez al invocar a Solution::new
    lambda: f64,

    // Para cachear el valor de fitness pues es un calculo costoso de realizar
    // Como los datos del struct no cambian, podemos hacer el cacheo sin miedo
    // Usamos RefCell para tener un patron de mutabilidad interior
    fitness: RefCell<Option<f64>>,

}

impl<'a, 'b> Solution<'a, 'b> {
    /// Util cuando no conocemos el valor de lambda, pues se calcula en esta llamada
    /// En otro caso, se puede construir el struct de forma directa
    pub fn new(
        cluster_indexes: Vec<u32>,
        data_points: &'a DataPoints,
        constraints: &'b Constraints,
        number_of_clusters: i32,
    ) -> Self {

        // Calculamos el valor de lambda
        let lambda = Point::max_distance_among_two(&data_points.get_points()) / constraints.get_data().len() as f64;

        return Self {
            cluster_indexes,
            data_points,
            constraints,
            number_of_clusters,
            lambda,
            fitness: RefCell::new(None),
        };
    }

    pub fn get_cluster_indexes(&self) -> Vec<u32>{
        return self.cluster_indexes.clone();
    }

    pub fn get_lambda(&self) -> f64{
        return self.lambda;
    }

    pub fn get_data_points(&self) -> &DataPoints{
        return self.data_points;
    }

    /// Comprueba si la solucion es valida o no
    /// Una solucion no es valida cuando existen clusters que no tienen ningun punto asignado
    /// Tambien es invalido cuando la dimensionalidad del vector de asignacion a cluster no
    /// coincide con la cantidad de puntos que tenemos que asignar
    pub fn is_valid(&self) -> bool {

        // Condicion de seguridad que nunca deberia ocurrir
        // Por eso pongo el panic!, porque es un problema de probramacion
        if self.cluster_indexes.len() != self.data_points.get_points().len(){
            eprintln!("No puede ocurrir que la longitud de los indices sea distinta al numero de puntos");
            return false;
        }

        // Comprobamos que no haya clusters vacios
        for cluster in 0..self.number_of_clusters{
            match self.cluster_indexes.iter().find(|&&x| x == cluster as u32){
                // Se ha encontrado, no hacemos nada
                Some(_) => (),

                // No hemos encontrado ningun valor de indice que apunte a este cluster
                None => {
                    return false
                },
            }
        }

        // No hemos encontrado cluster vacio
        return true;
    }

    /// Devuelve el valor de fitness. Si ya ha sido calculado antes, devuelve
    /// el valor cacheado sin repetir los calculos
    pub fn fitness(&self) -> f64 {
        let fit_opt = *self.fitness.borrow();

        match fit_opt{
            // Tenemos el valor cacheado del fitness, no repetimos calculos
            Some(fitness) => return fitness,

            // No hemos calculado todavia el valor de fitness
            // Lo calculamos, lo guardamos y lo devolvemos
            None => {
                let calc_fitness = self.global_cluster_mean_distance() + self.lambda * self.infeasibility() as f64;
                *self.fitness.borrow_mut() = Some(calc_fitness);
                return calc_fitness;
            }
        }
    }

    /// Calcula el valor del fitness, y las iteraciones consumidas en el proceso
    /// Las iteraciones consumidas pueden ser o bien 0 o bien 1
    pub fn fitness_and_consumed(&self) -> (f64, u32){
        let consumed = match *self.fitness.borrow(){
            None => 1,
            Some(_) => 0,
        };

        return (self.fitness(), consumed);
    }

    /// Resetea el valor de fitness a None, por lo tanto, cuando se intente acceder a este valor,
    /// deberemos volver a calcular su valor
    pub fn invalid_fitness_cache(&mut self){
        *self.fitness.borrow_mut() = None;
    }

    /// Comprueba si el fitness de la solucion esta cacheado (ya calculado) o no
    pub fn is_fitness_cached(&self) -> bool{
        match *self.fitness.borrow(){
            Some(_) => return true,
            None => return false,
        };
    }

    /// Devuelve el primer vecino de la solucion valido que mejora la solucion
    /// actual (el primero mejor)
    /// Necesitamos saber cuantas evaluaciones podemos consumir como criterio de parada, para
    /// evitar generar todo un vecindario cuando esto no sea posible por agotar las evaluaciones
    pub fn get_neighbour(&self, left_iterations: i32, rng: &mut StdRng) -> FitnessEvaluationResult<Option<Self>> {

        // Para llevar las cuentas de las llamadas a funcion fitness consumidas en la operacion
        let mut fitness_consumed = 0;

        // Fitness de la solucion a mejorar. Solo lo calculamos una unica vez
        let (fitness_to_beat, ev_cons) = self.fitness_and_consumed();
        fitness_consumed += ev_cons;

        // Tomo los generadores de vecinos
        let mut neighbours_generator = NeighbourGenerator::generate_all_neighbours(self.data_points.len() as i32, self.number_of_clusters);

        // Mezclo los generadores de vecinos
        neighbours_generator.shuffle(rng);

        for current_generator in neighbours_generator{

            // Generamos la nueva solucion
            let current_solution = self.generate_solution_from(current_generator);

            // Si la solucion no es valida, no perdemos evaluaciones del fitness
            if current_solution.is_valid() == false{
                continue;
            }

            // Tomamos el valor del fitness de la nueva solucion
            let (current_fitness, ev_cons) = current_solution.fitness_and_consumed();
            fitness_consumed += ev_cons;


            // Comprobamos si hemos mejorado a la solucion original
            if current_fitness < fitness_to_beat{
                return FitnessEvaluationResult::new(Some(current_solution), fitness_consumed);
            }

            // Comprobamos si hemos excedido el maximo de evaluaciones de las que disponiamos
            if fitness_consumed >= left_iterations as u32{
                // No hemos encontrado un vecino mejor a tiempo
                return FitnessEvaluationResult::new(None, fitness_consumed);
            }
        }

        // No hemos encontrado un vecino mejor
        return FitnessEvaluationResult::new(None, fitness_consumed);
    }

    /// A partir de un NeighbourGenerator, genera la solucion que representa el
    /// generador aplicado a la solucion &self
    fn generate_solution_from(&self, generator: NeighbourGenerator) -> Self{
        let mut new_solution = Self{
            cluster_indexes: self.cluster_indexes.clone(),
            data_points: &self.data_points,
            constraints: &self.constraints,
            number_of_clusters: self.number_of_clusters,
            lambda: self.lambda,
            fitness: RefCell::new(None), // None porque hemos cambiado la solucion, por tanto,
                                         // tendra otro valor de fitness
        };

        new_solution.cluster_indexes[generator.get_element_index() as usize] = generator.get_new_cluster();
        return new_solution;
    }

    /// Genera una solucion inicial aleatoria, como punto de partida de las busquedas
    // TODO -- no puede dejar clusters vacios
    pub fn generate_random_solution(
        data_points: &'a DataPoints,
        constraints: &'b Constraints,
        number_of_clusters: i32,
        rng: &mut StdRng
    ) -> Self {

        return Self::new(
            (0..data_points.get_points().len()).into_iter().map(|_| rng.gen_range(0..number_of_clusters) as u32).collect(),
            data_points,
            constraints,
            number_of_clusters,
        );

    }

    /// Dado un cluster (representado por el entero que los identifica), calcula
    /// la distancia intracluster en la solucion actual
    pub fn intra_cluster_distance(&self, cluster: u32) -> f64{

        // Calculamos el vector de puntos que estan en el cluster
        let cluster_points = self.get_points_in_cluster(cluster);

        // Comprobacion de seguridad
        if cluster_points.len() == 0{
            panic!("[Err: Solution::intra_cluster_distance] Cluster without points");
        }

        // Calculamos el centroide de dicho conjunto de puntos
        let centroid = Point::calculate_centroid(&cluster_points);

        // Calculamos la distancia intracluster
        let mut cum_sum = 0.0;
        for point in &cluster_points{
            cum_sum += Point::distance(point, &centroid);
        }

        return cum_sum / cluster_points.len() as f64;

    }

    /// Dado un cluster indicado por el indice que lo representa, devuelve los puntos
    /// que componen dicho cluster
    pub fn get_points_in_cluster(&self, cluster: u32) -> Vec<&Point>{
        let mut cluster_points = vec![];

        for (index, curr_cluster) in self.cluster_indexes.iter().enumerate(){
            if *curr_cluster == cluster{
                cluster_points.push(&self.data_points.get_points()[index]);
            }
        }

        return cluster_points;
    }

    /// Dado un cluster indicado por el indice que lo representa, devuelve los indices de los
    /// puntos que componen dicho cluster
    pub fn get_index_points_in_cluster(&self, cluster: u32) -> Vec<usize>{
        let mut index_cluster_points = vec![];

        for (index, curr_cluster) in self.cluster_indexes.iter().enumerate(){
            if *curr_cluster == cluster{
                index_cluster_points.push(index);
            }
        }

        return index_cluster_points;
    }

    /// Calcula la media de distancias intracluster sobre todos los clusters
    /// Esto representa una de las componentes de la funcion fitness
    pub fn global_cluster_mean_distance(&self) -> f64{
        let mut cum_sum = 0.0;
        for i in 0 .. self.number_of_clusters as u32 {
            cum_sum += self.intra_cluster_distance(i);
        }

        return cum_sum / self.number_of_clusters as f64;
    }

    /// Calcula el numero de restricciones que se violan en la solucion actual
    pub fn infeasibility(&self) -> i32{
        let mut infea = 0;
        for ((first_index, second_index), value) in self.constraints.get_data(){

            // Tomamos los dos indices de cluster para compararlos
            let first_cluster = self.cluster_indexes[*first_index as usize];
            let second_cluster  = self.cluster_indexes[*second_index as usize];

            match value{
                ConstraintType::MustLink => {
                    // Sumamos cuando no estan en el mismo cluster
                    if first_cluster != second_cluster{
                        infea += 1;
                    }
                }

                ConstraintType::CannotLink => {
                    // Sumamos cuando estan en el mismo cluster
                    if first_cluster == second_cluster{
                        infea += 1;
                    }
                }
            }
        }

        return infea;
    }

    /// Muestra las distancias intracluster de cada uno de los clusters
    /// Esta funcion ha sido usada para debuggear el codigo
    pub fn show_intra_cluster_distances(&self){
        for cluster in 0 .. self.number_of_clusters{
            println!("\tDistancia intra-cluster del cluster {}: {}", cluster, self.intra_cluster_distance(cluster as u32));
        }

    }

    /// Devuelve el conjunto de clusters que tiene mas de un punto asignado
    // TODO -- TEST -- es muy facil de testear y es algo bastante critico
    pub fn get_clusters_with_more_than_one_point(&self) -> Vec<i32>{
        let mut clusters_with_more_than_one_point = vec![];

        for cluster in 0..self.number_of_clusters{
            let points_in_cluster = self.get_points_in_cluster(cluster as u32);

            if points_in_cluster.len() >= 2{
                clusters_with_more_than_one_point.push(cluster);

            }
        }

        return clusters_with_more_than_one_point;
    }

    /// Devuelve el conjunto de clusters que no tienen puntos asignados
    // TODO -- TEST -- es muy facil de testear y algo critico
    pub fn get_cluster_without_points(&self) -> Vec<i32>{
        let mut clusters_without_points = vec![];

        for cluster in 0..self.number_of_clusters{
            let points_in_cluster = self.get_points_in_cluster(cluster as u32);

            if points_in_cluster.len() == 0{
                clusters_without_points.push(cluster);
            }
        }

        return clusters_without_points;
    }

}

/// Metodos asociados a la parte genetica de las practicas
impl<'a, 'b> Solution<'a, 'b> {

    /// Dadas dos soluciones, devuelve aquella con mejor fitness
    /// Entendemos por mejor fitness, aquel con menor valor numerico, pues estamos buscando
    /// minimizar la funcion de fitness
    pub fn binary_tournament<'c>(first: &'c Solution<'a, 'b>, second: &'c Solution<'a, 'b>) -> (&'c Solution<'a, 'b>, u32){
        let (first_fitness, first_consumed) = first.fitness_and_consumed();
        let (second_fitness, second_consumed) = second.fitness_and_consumed();
        if first_fitness < second_fitness{
            return (first, first_consumed + second_consumed);
        }else{
            return (second, first_consumed + second_consumed);
        }
    }

    /// Operador de cruce uniforme para dos soluciones
    // TODO -- testear porque creo que puede estar mal
    pub fn uniform_cross(first: &Self, second: &Self, rng: &mut StdRng) -> Self{
        let gen_size= first.cluster_indexes.len();
        let half_gen_size = (gen_size as f64 / 2.0) as usize;

        // Generamos aleatoriamente las posiciones de los genes del primer padre con las que nos
        // quedamos. Para ello, tomamos una permutacion aleatoria de {0, ..., gen_size - 1} y nos
        // quedamos con la primera mitad. La segunda mitad nos indicara las posiciones que usamos
        // del segundo padre
        let mut positions_to_mutate: Vec<usize> = (0..gen_size as usize).collect();
        positions_to_mutate.shuffle(rng);

        // Nueva solucion a partir de la informacion de uno de los padres
        let mut crossed_solution = first.clone();

        // Tomamos los elementos aleatorios del primer padre
        for index in 0..half_gen_size{
            // Tenemos que usar el indice que indica de la permutacion aleatoria
            let curr_index = positions_to_mutate[index];
            crossed_solution.cluster_indexes[curr_index] = first.cluster_indexes[curr_index];
        }

        // Tomamos los elementos aleatorios del segundo padre
        for index in half_gen_size..gen_size{
            // Tenemos que usar el indice que indica de la permutacion aleatoria
            let curr_index = positions_to_mutate[index];
            crossed_solution.cluster_indexes[curr_index] = second.cluster_indexes[curr_index];
        }

        // No deberia ocurrir, pero reseteo el valor del fitness para evitar problemas
        // No a??ade sobrecoste, porque al estar cruzando, el fitness de la nueva solucion se tiene
        // que recalcular de todas formas
        crossed_solution.invalid_fitness_cache();

        // Reparamos la solucion en caso de que sea necesario
        if crossed_solution.is_valid() == false {
            crossed_solution.repair_solution(rng);
        }

        return crossed_solution;
    }

    /// Operador de cruce por segmento fijo
    // TODO -- testear porque puede estar bastante mal
    pub fn cross_segment(first: &Self, second: &Self, rng: &mut StdRng) -> Self{
        // Nueva solucion a partir de la informacion de uno de los padres
        let mut crossed_solution = first.clone();
        let gen_size= first.cluster_indexes.len();

        // Seleccionamos el inicio y tama??o del segmento
        let segment_start = rng.gen_range(0..gen_size);
        let segment_size = rng.gen_range(0..gen_size);

        // Copiamos los valores del primer padre
        for i in 0..segment_size{
            // Calculamos la posicion actual en el segmento
            let index = (segment_start + i) % gen_size;

            crossed_solution.cluster_indexes[index] = first.cluster_indexes[index];
        }

        // Copiamos, con cruce uniforme, el resto de valores
        for i in 0..(gen_size - segment_size){
            // Calculamos el indice de la misma forma que antes, partiendo de donde nos quedamos
            let index = (segment_size + segment_start + i) % gen_size;

            // Padre del que queremos tomar la informacion
            // Tenemos que poner el rango hasta 2, porque el extremo superior no se considera
            let choose_parent = rng.gen_range(0..2);
            if choose_parent == 0{
                crossed_solution.cluster_indexes[index] = first.cluster_indexes[index];
            }else{
                crossed_solution.cluster_indexes[index] = second.cluster_indexes[index];
            }
        }

        // No deberia ocurrir, pero reseteo el valor del fitness para evitar problemas
        // No a??ade sobrecoste, porque al estar cruzando, el fitness de la nueva solucion se tiene
        // que recalcular de todas formas
        crossed_solution.invalid_fitness_cache();

        // Reparamos la solucion en caso de que sea necesario
        if crossed_solution.is_valid() == false {
            crossed_solution.repair_solution(rng);
        }

        return crossed_solution;
    }

    /// Devuelve una solucion mutada
    ///
    /// Permitimos que se mute a una solucion no valida, que luego es reparada. Si no hacemos esto,
    /// perdemos mucha variabilidad que queremos introducir con la mutacion. Por ejemplo, si
    /// dejamos un cluster con un solo punto, esta posicion se queda "atascada" al no poder ser
    /// cambiada por mutacion. Esto era lo que provocaba el mal comportamiento
    pub fn mutated(&self, rng: &mut StdRng) -> Self{
        // Copiamos la solucion para realizar una modificacion
        let mut mutated_sol = self.clone();

        // Tomamos una posicion a mutar. Esta posicion puede ser la de un cluster que no tenga mas
        // de dos puntos. En ese caso, deja al cluster vacio. Por ello, debemos reparar la solucion
        // si se da el caso. Otra opcion es comprobar que el cluster escogido tenga al menos dos
        // puntos asignados, pero entonces excluimos clusters con solo un punto, disminuyendo en
        // parte la variabilidad que introduce la mutacion
        let mut_position_candidates: Vec<i32> = (0..mutated_sol.data_points.len() as i32).collect();
        let mut_position = mut_position_candidates.choose(rng).expect("No tenemos puntos en nuestro dataset que asignar a clusters");

        // Podemos elegir como nuevo valor aquellos que esten en el itervalo adecuado y que no sean
        // el cluster original que ya teniamos, pues estariamos perdiendo una mutacion efectiva
        let mut new_cluster_candidates: Vec<i32> = (0..mutated_sol.number_of_clusters).collect();
        new_cluster_candidates.retain(|&x| x != mutated_sol.cluster_indexes[*mut_position as usize] as i32);
        let mut_value = new_cluster_candidates.choose(rng).expect("No hemos podido generar una lista de clusters candidatos");

        // Mutamos el valor
        mutated_sol.cluster_indexes[*mut_position as usize] = *mut_value as u32;

        // Reseteamos el fitness, porque estamos haciendo un cambio a la solucion que devolvemos
        mutated_sol.invalid_fitness_cache();

        // Comprobamos que la solucion sea valida. En caso de que no lo sea, la reparamos
        if mutated_sol.is_valid() == false{
            mutated_sol.repair_solution(rng);
        }

        // Comprobamos que la solucion no sea la misma (puede ocurrir al reparar). En caso de que
        // sea la misma solucion, volvemos a mutar
        if mutated_sol.cluster_indexes == self.cluster_indexes{
            return self.mutated(rng);
        }

        return mutated_sol;
    }

    /// Devuelve una solucion mutada fuertemente. Se usa para iterative_local_search. La mutacion
    /// que usamos en algoritmos geneticos, porque queremos alejarnos mas de la solucion dada
    pub fn hard_mutated(&self, segment_size: usize, rng: &mut StdRng) -> Self{
        // Copia para devolver la solucion mutada sin tener que mutar la solucion original
        let mut mutated = self.clone();

        // Seleccionamos el inicio del segmento
        let gen_size = self.cluster_indexes.len();
        let segment_start = rng.gen_range(0..gen_size);

        // Mutamos los valores el el segmento. El resto de valores son automaticamente copiados del
        // padre porque mutated es clone de self
        for i in 0..segment_size{
            // Indice que debemos mutar segun los valores del segmento
            let index = (segment_start + i) % segment_size;

            // Mutamos dicho valor. No comprobamos que la mutacion sea ahora valida, para dar mas
            // variedad. Mas adelante repararemos la solucion
            let new_cluster = rng.gen_range(0..mutated.number_of_clusters);
            mutated.cluster_indexes[index] = new_cluster as u32;
        }

        // Reparamos la solucion si la solucion mutada acaba por no ser valida
        if mutated.is_valid() == false{
            mutated.repair_solution(rng);
        }

        return mutated;
    }

    /// Repara una solucion. Toma los clusters sin puntos asignados, y asigna aleatoriamente un
    /// punto de un cluster que tenga mas de un punto asignado (pues no podemos dejar otros
    /// clusters vacios en el proceso de reparacion)
    pub fn repair_solution(&mut self, rng: &mut StdRng){
        // Tomamos los clusters sin puntos asignados
        let clusters_without_points = self.get_cluster_without_points();

        if clusters_without_points.len() == 0{
            return;
        }

        // Tomamos el primer cluster sin puntos para repararlo
        let cluster_without_points = clusters_without_points[0];

        // Tomamos los clusters con al menos dos puntos asignados
        let clusters_with_more_than_one_point = self.get_clusters_with_more_than_one_point();

        // Realizamos el cambio

        // Seleccionamos el cluster del que quitamos un punto aleatoriamente
        let selected_cluster = clusters_with_more_than_one_point.choose(rng).expect("No hay clusters con mas de un punto");

        // Seleccionamos un indice en los puntos a los que realizar el cambio
        let point_indixes_selected_cluster = self.get_index_points_in_cluster(*selected_cluster as u32);
        let selected_point_index = point_indixes_selected_cluster.choose(rng).expect("No hay puntos en el cluster seleccionado");

        // Realizamos la asignacion
        self.cluster_indexes[*selected_point_index] = cluster_without_points as u32;

        // Si quedan mas clusters sin puntos asignados, volvemos a llamar a esta funcion para que
        // siga reparando la solucion
        if clusters_without_points.len() >= 2{
            self.repair_solution(rng);
        }

        // Al haber modificado la solucion, debemos invalidar la cache del fitness
        self.invalid_fitness_cache();
    }
}

/// Metodos asociados a la parte memetica de las practicas
impl<'a, 'b> Solution<'a, 'b> {
    pub fn soft_local_search(&self, max_fails: i32, rng: &mut StdRng) -> FitnessEvaluationResult<Self>{
        let mut new_solution = self.clone();
        let mut fit_eval_cons = 0;

        // Recorreremos las posiciones de los puntos en orden aleatorio
        let mut indixes: Vec<i32> = (0..self.data_points.len() as i32).collect();
        indixes.shuffle(rng);

        // Valores iniciales para el algoritmo
        let mut fails = 0;
        let mut i = 0;

        // Iteramos sobre las asignaciones de clusters mientras no sobrepasemos el valor de fallos
        while fails < max_fails && i < self.data_points.len(){
            // Tomamos la posicion que toca cambiar en esta iteracion
            let index = indixes[i];

            // Seleccionamos el mejor cluster para este punto en la posicion index
            let new_cluster_result = new_solution.select_best_cluster(index as u32);
            let new_cluster = new_cluster_result.get_result();
            fit_eval_cons += new_cluster_result.get_iterations_consumed();

            // Realizamos el cambio, guardando el valor original de la asignacion
            let past_cluster = new_solution.cluster_indexes[index as usize];
            new_solution.cluster_indexes[index as usize] = *new_cluster;

            // Comprobamos si hemos realizado un cambio o no, incrementando el contador de fallos
            // en caso de que sea necesario
            if *new_cluster == past_cluster{
                fails += 1;
            }

            // Pasamos a la siguiente posicion
            i += 1;
        }

        return FitnessEvaluationResult::new(new_solution, fit_eval_cons);
    }

    /// Selecciona la mejor asignacion de cluster para un punto dado por su indice.
    /// La mejor asignacion es aquella que es valida y que tiene el minimo valor de fitness
    /// Esta operacion va a consumir muchas evaluaciones del fitness
    /// La solucion &self debe ser una solucion valida para poder hacer esta busqueda sin problemas
    // TODO -- TEST -- bastante facil de testear, ademas es una parte critica de los memeticos
    pub fn select_best_cluster(&self, point_index: u32) -> FitnessEvaluationResult<u32>{
        // Comprobacion de seguridad
        debug_assert!(
            self.is_valid() == true,
            "La solucion original no es valida, no se puede buscar la mejor asignacion de cluster"
        );

        let mut fit_eval_cons = 0;
        let mut best_cluster = -1;
        let mut best_fitness = -1.0;

        // Recorremos todos las posibles asignaciones de cluster
        for cluster in 0..self.number_of_clusters{

            // Generamos la solucion asociada al cambio a este cluster
            let mut new_sol = self.clone();
            new_sol.cluster_indexes[point_index as usize] = cluster as u32;

            // Comrpobamos que la solucion generada sea valida
            if new_sol.is_valid() == false{
                continue;
            }

            // Evaluamos el fitness. Para ello, previamente tenemos que invalidar la cache
            new_sol.invalid_fitness_cache();
            let (new_sol_fit, ev_cons) = new_sol.fitness_and_consumed();
            fit_eval_cons += ev_cons;

            // Comprobamos si esta es la mejor
            if best_fitness == -1.0 || new_sol_fit < best_fitness{
                best_fitness = new_sol_fit;
                best_cluster = cluster;
            }
        }

        // Comprobacion de seguridad
        debug_assert!(
            best_cluster != -1,
            "No hemos encontrado una mejor asignacion. Esto no es correcto, pues la solucion original es valida y podria ser un primer candidato"
        );

        return FitnessEvaluationResult::new(best_cluster as u32, fit_eval_cons);
    }
}

/// Metodos asociados al algoritmo de enfriamiento simulado
impl<'a, 'b> Solution<'a, 'b>{
    /// Genera un vecino aleatorio unico, sin recurrir a usar generador de vecinos
    /// Se parece mucho a mutated. Sin embargo, en mutated generamos la mutacion y permitimos
    /// soluciones no validas, que son reparadas. Esto hace que en una mutacion pueda cambiar mas
    /// de un valor. En este caso, comprobamos que solo se modifique una posicion
    pub fn one_random_neighbour(&self, rng: &mut StdRng) -> Self{
        // Usamos la funcion de mutacion para realizar el cambio
        let mutated = self.mutated(rng);

        // Si hay mas de una diferencia, es porque el operador de reparacion ha reparado provocando
        // mas cambios. En este caso, esto no es lo que queremos
        if mutated.number_of_discrepancies(self) != 1{
            return self.mutated(rng);
        }

        return mutated;
    }

    /// Calcula el numero de puntos que tienen distinta asignacion de cluster entre dos soluciones
    // TODO -- TEST -- muy sencillo de escribir un test
    fn number_of_discrepancies(&self, other: &Solution) -> i32{
        let mut discrepancies = 0;
        for index in 0..self.cluster_indexes.len(){
            if self.cluster_indexes[index] != other.cluster_indexes[index]{
                discrepancies += 1;
            }
        }

        return discrepancies;
    }

}

#[cfg(test)]
mod tests{
    use crate::problem_datatypes::Solution;
    use crate::problem_datatypes::DataPoints;
    use crate::problem_datatypes::Point;
    use crate::problem_datatypes::Constraints;
    use crate::problem_datatypes::ConstraintType;

    use rand::rngs::StdRng;
    use rand::SeedableRng;

    // Para comprobar que dos soluciones son practicamente iguales (ignorando problemas
    // del punto flotante)
    use assert_approx_eq::assert_approx_eq;

    fn epsilon() -> f64{0.01}               // Tolerancia a fallos de punto flotante
    fn max_test_iterations() -> u32{10000}  // Maximo de iteraciones sobre test

    /// Callback porque en otro caso tenemos que hacer clones de los datos
    /// que componen la solucion que devolvemos
    /// FnOnce porque queremos tener ownership de la solucion que generamos
    fn generate_basic_solution(callback: impl FnOnce(&Solution)) {
        let cluster_indexes = vec![0, 1, 2, 3, 0, 1];
        let data_points = DataPoints::new(vec![
            Point::from_vec(vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
            Point::from_vec(vec![0.0, 1.0, 0.0, 0.0, 0.0, 0.0]),
            Point::from_vec(vec![0.0, 0.0, 1.0, 0.0, 0.0, 0.0]),
            Point::from_vec(vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0]),
            Point::from_vec(vec![0.0, 0.0, 0.0, 0.0, 1.0, 0.0]),
            Point::from_vec(vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0]),
        ]);

        let mut constraints = Constraints::new();
        constraints.add_constraint(0, 1, ConstraintType::CannotLink);
        constraints.add_constraint(0, 2, ConstraintType::CannotLink);
        constraints.add_constraint(1, 3, ConstraintType::CannotLink);
        constraints.add_constraint(1, 4, ConstraintType::MustLink);
        constraints.add_constraint(2, 5, ConstraintType::MustLink);

        let number_of_clusters = 4;

        let solution = Solution::new(cluster_indexes, &data_points, &constraints, number_of_clusters);
        callback(&solution)
    }

    #[test]
    // Simplemente comprobamos que estamos almacenando bien los puntos
    fn test_solution_saves_properly_data_points_over_basic_sol(){
        generate_basic_solution(|solution| {
            let data_points = solution.get_data_points().get_points();
            assert_eq!(solution.get_points_in_cluster(0), vec![&data_points[0], &data_points[4]]);
            assert_eq!(solution.get_points_in_cluster(1), vec![&data_points[1], &data_points[5]]);
            assert_eq!(solution.get_points_in_cluster(2), vec![&data_points[2]]);
            assert_eq!(solution.get_points_in_cluster(3), vec![&data_points[3]]);
        });
    }

    #[test]
    // Comprobamos que la distancia maxima entre dos puntos es la que tiene que ser
    fn test_lambda_is_correct_over_basic_sol(){
        generate_basic_solution(|solution| {
            let calculated_lambda = solution.get_lambda();
            let expected_lambda = (2.0 as f64).sqrt() / 5.0;
            assert_approx_eq::assert_approx_eq!(calculated_lambda, expected_lambda, epsilon());

        });
    }

    #[test]
    // Comprobamos que estamos calculando bien el numero de restricciones violadas
    fn test_infeasibility_is_correct_over_basic_sol(){
        generate_basic_solution(|solution| {
            let calc_infea = solution.infeasibility();
            let exp_infea = 2; // Solo se violan las dos must link
            assert_eq!(calc_infea, exp_infea);
        });

        // Hacemos una variacion de la solucion
        generate_basic_solution(|solution| {
            // Modifico la solucion
            let cluster_indexes = vec![1, 1, 2, 3, 0, 1];
            let other_solution = Solution::new(cluster_indexes, solution.data_points, solution.constraints, solution.number_of_clusters);

            let calc_infea = other_solution.infeasibility();
            let exp_infea = 3; // Se violan las dos must link y una CannotLink
            assert_eq!(calc_infea, exp_infea);
        });
    }

    #[test]
    fn test_centroids_over_basic_sol(){
        generate_basic_solution(|solution| {
            // Primer cluster
            let cluster_points = solution.get_points_in_cluster(0);
            let calc_centroid = Point::calculate_centroid(&cluster_points);
            let exp_centroid = Point::from_vec(vec![0.5, 0. , 0. , 0. , 0.5, 0. ]);
            assert_eq!(calc_centroid, exp_centroid);

            // Segundo cluster
            let cluster_points = solution.get_points_in_cluster(1);
            let calc_centroid = Point::calculate_centroid(&cluster_points);
            let exp_centroid = Point::from_vec(vec![0. , 0.5, 0. , 0. , 0. , 0.5]);
            assert_eq!(calc_centroid, exp_centroid);

            // Tercer cluster
            let cluster_points = solution.get_points_in_cluster(2);
            let calc_centroid = Point::calculate_centroid(&cluster_points);
            let exp_centroid = Point::from_vec(vec![0.0, 0.0, 1.0, 0.0, 0.0, 0.0]);
            assert_eq!(calc_centroid, exp_centroid);

            // Cuarto cluster
            let cluster_points = solution.get_points_in_cluster(3);
            let calc_centroid = Point::calculate_centroid(&cluster_points);
            let exp_centroid = Point::from_vec(vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0]);
            assert_eq!(calc_centroid, exp_centroid);


        });

    }

    #[test]
    fn test_intracluser_distance_over_basic_sol(){
        generate_basic_solution(|solution| {
            // Distancia intracluster del primer cluster
            let calc_intra = solution.intra_cluster_distance(0);
            let exp_intra = 0.7071067811865476;
            assert_approx_eq!(calc_intra, exp_intra, epsilon());

            // Distancia intracluster del segundo cluster
            let calc_intra = solution.intra_cluster_distance(1);
            let exp_intra = 0.7071067811865476;
            assert_approx_eq!(calc_intra, exp_intra, epsilon());

            // Distancia intracluster del tercer cluster
            let calc_intra = solution.intra_cluster_distance(2);
            let exp_intra = 0.0;
            assert_approx_eq!(calc_intra, exp_intra, epsilon());

            // Distancia intracluster del cuarto cluster
            let calc_intra = solution.intra_cluster_distance(3);
            let exp_intra = 0.0;
            assert_approx_eq!(calc_intra, exp_intra, epsilon());


        });
    }

    #[test]
    fn test_global_cluster_distance_over_basic_sol(){
        generate_basic_solution(|solution| {
            let calc_global_dist = solution.global_cluster_mean_distance();
            let exp_global_dist = (0.7071067811865476 * 2.0) / 4.0;
            assert_approx_eq!(calc_global_dist, exp_global_dist, epsilon());
        });

    }

    #[test]
    fn test_fitness_is_correct_over_basic_sol(){
        generate_basic_solution(|solution| {
            let calc_fitness = solution.fitness();

            let exp_lambda = (2.0 as f64).sqrt() / 5.0;
            let exp_global_dist = (0.7071067811865476 * 2.0) / 4.0;
            let exp_infea = 2;
            let exp_fitness = exp_lambda * exp_infea as f64 + exp_global_dist;

            assert_approx_eq::assert_approx_eq!(calc_fitness, exp_fitness, epsilon());

        });

    }

    #[test]
    fn test_solutions_are_not_valid(){
        generate_basic_solution(|solution| {
            // A partir de la solucion basica, generamos una solucion que no sea valida
            let cluster_indexes = vec![0, 0, 0, 0, 1, 1];
            let solution = Solution::new(cluster_indexes, &solution.data_points, &solution.constraints, solution.number_of_clusters);

            let expected_is_valid = false;
            let calc_is_valid = solution.is_valid();
            assert_eq!(expected_is_valid, calc_is_valid);

            // Ahora generamos una solucion que no sea valida por el tama??o del vector (peque??o)
            let cluster_indexes = vec![0, 1, 2, 3];
            let solution = Solution::new(cluster_indexes, &solution.data_points, &solution.constraints, solution.number_of_clusters);

            let expected_is_valid = false;
            let calc_is_valid = solution.is_valid();
            assert_eq!(expected_is_valid, calc_is_valid);

            // Ahora generamos una solucion que no sea valida por el tama??o del vector (grande)
            let cluster_indexes = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            let solution = Solution::new(cluster_indexes, &solution.data_points, &solution.constraints, solution.number_of_clusters);

            let expected_is_valid = false;
            let calc_is_valid = solution.is_valid();
            assert_eq!(expected_is_valid, calc_is_valid);

        });
    }

    #[test]
    fn test_repair_bad_solutions(){
        generate_basic_solution(|solution| {
            // A partir de la solucion basica, generamos una solucion que no sea valida, pero con
            // una dimensionalidad correcta
            let cluster_indexes = vec![0, 0, 0, 0, 1, 1];
            let solution = Solution::new(cluster_indexes, &solution.data_points, &solution.constraints, solution.number_of_clusters);

            // Reparamos la solucion. Al tener dependencia aleatoria, hacemos esto muchas veces
            // sobre la solucion mala original y vemos que se hace bien
            let mut rng = StdRng::seed_from_u64(123456789);
            for _ in 0..max_test_iterations(){
                let mut curr_sol = solution.clone();
                curr_sol.repair_solution(&mut rng);

                let expected_is_valid = true;
                let calc_is_valid = curr_sol.is_valid();
                assert_eq!(expected_is_valid, calc_is_valid);
            }

            // Generamos otra solucion que no sea valida, y vemos si se repara bien
            let cluster_indexes = vec![2, 1, 2, 0, 1, 1];
            let solution = Solution::new(cluster_indexes, &solution.data_points, &solution.constraints, solution.number_of_clusters);

            // Reparamos la solucion. Al tener dependencia aleatoria, hacemos esto muchas veces
            // sobre la solucion mala original y vemos que se hace bien
            let mut rng = StdRng::seed_from_u64(123456789);
            for _ in 0..max_test_iterations(){
                let mut curr_sol = solution.clone();
                curr_sol.repair_solution(&mut rng);

                let expected_is_valid = true;
                let calc_is_valid = curr_sol.is_valid();
                assert_eq!(expected_is_valid, calc_is_valid);
            }
        });
    }

    #[test]
    fn test_mutation_generates_valid_population(){
        generate_basic_solution(|solution| {
            let mut rng = StdRng::seed_from_u64(123456789);

            // Dependemos de la aleatoriedad, asi que repetimos un numero dado de veces el
            // experimento
            for _ in 0..max_test_iterations(){
                let mut_sol = solution.mutated(&mut rng);

                let expected_is_valid = true;
                let calc_is_valid = mut_sol.is_valid();
                assert_eq!(expected_is_valid, calc_is_valid);
            }
        });
    }

    #[test]
    fn test_mutated_solution_differs_at_least_one_position(){
        generate_basic_solution(|solution| {
            let mut rng = StdRng::seed_from_u64(123456789);

            // Dependemos de la aleatoriedad, asi que repetimos un numero dado de veces el
            // experimento
            for _ in 0..max_test_iterations(){
                let mut_sol = solution.mutated(&mut rng);

                // Calculamos las diferencias entre el original y el mutado
                let mut calc_diffs = 0;
                for (index, cluster_assignation) in mut_sol.cluster_indexes.iter().enumerate(){
                    if *cluster_assignation != solution.cluster_indexes[index]{
                        calc_diffs += 1;
                    }
                }

                let calc_diffs = calc_diffs;
                let expected_min_diffs = 1;
                assert!(calc_diffs >= expected_min_diffs, "Se esperaba al menos una diferencia, se obtuvieron {} diferencias", calc_diffs);
            }


        });
    }
}
