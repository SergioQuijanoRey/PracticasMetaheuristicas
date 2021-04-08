// Para fijar la semilla de numeros aleatorios
use rand::Rng;
use rand::rngs::StdRng;

// Para hacer shuffle de un vector
use rand::seq::SliceRandom;

use crate::problem_datatypes::{DataPoints, Constraints, Point, ConstraintType, NeighbourGenerator};

/// Estructura que representa una solucion del problema
///
/// La solucion viene representada como un vector de indices
/// En dicho vector, la posicion i-esima indica el cluster al que pertenece el i-esimo
/// punto del conjunto de datos
#[derive(Debug)]
pub struct Solution<'a, 'b> {
    cluster_indexes: Vec<u32>,
    data_points: &'a DataPoints,
    constraints: &'b Constraints,
    number_of_clusters: i32,

    /// Representa el peso de infeasibility en el calculo de fitness
    /// Solo se calcula una vez al invocar a Solution::new
    lambda: f64,
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
    fn is_valid(&self) -> bool {

        // Condicion de seguridad que nunca deberia ocurrir
        // Por eso pongo el panic!, porque es un problema de probramacion
        if self.cluster_indexes.len() != self.data_points.get_points().len(){
            panic!("No puede ocurrir que la longitud de los indices sea distinta al numero de puntos");
        }

        // Comprobamos que no haya clusters vacios
        for cluster in 0..self.number_of_clusters{
            match self.cluster_indexes.iter().find(|&&x| x == cluster as u32){
                // Se ha encontrado, no hacemos nada
                Some(_) =>(),

                // No hemos encontrado ningun valor de indice que apunte a este cluster
                None => return false,
            }
        }

        // No hemos encontrado cluster vacio
        return true;
    }

    /// Calcula el valor de fitness de la solucion
    pub fn fitness(&self) -> f64 {
        return self.global_cluster_mean_distance() + self.lambda * self.infeasibility() as f64;
    }

    /// Devuelve el primer vecino de la solucion valido que mejora la solucion
    /// actual (el primero mejor)
    pub fn get_neighbour(&self, rng: &mut StdRng) -> Option<Self> {

        // Tomo los generadores de vecinos
        let mut neighbours_generator = NeighbourGenerator::generate_all_neighbours(self.data_points.len() as i32, self.number_of_clusters);

        // Mezclo los generadores de vecinos
        neighbours_generator.shuffle(rng);

        for current_generator in neighbours_generator{
            let current_solution = self.generate_solution_from(current_generator);

            if current_solution.is_valid() && current_solution.fitness() < self.fitness(){
                return Some(current_solution);
            }
        }

        // No hemos encontrado un vecino mejor
        return None;
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
    // TODO -- comprobar que no estemos dividiendo por cero, ya sea con un result
    // o con un panic!
    pub fn intra_cluster_distance(&self, cluster: u32) -> f64{
        // Calculamos el vector de puntos que estan en el cluster
        let cluster_points = self.get_points_in_cluster(cluster);

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
}

#[cfg(test)]
mod tests{
    use crate::problem_datatypes::Solution;
    use crate::problem_datatypes::DataPoints;
    use crate::problem_datatypes::Point;
    use crate::problem_datatypes::Constraints;
    use crate::problem_datatypes::ConstraintType;

    // Para comprobar que dos soluciones son practicamente iguales (ignorando problemas
    // del punto flotante)
    use assert_approx_eq::assert_approx_eq;

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
    fn test_solution_saves_properly_data_points(){
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
    fn test_lambda_is_correct(){
        generate_basic_solution(|solution| {
            let calculated_lambda = solution.get_lambda();
            let expected_lambda = 1.122462048309373 / 5.0;
            assert_approx_eq::assert_approx_eq!(calculated_lambda, expected_lambda, 0.01);

        });
    }

}
