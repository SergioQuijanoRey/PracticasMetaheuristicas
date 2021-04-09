// Para fijar la semilla de numeros aleatorios
use rand::Rng;
use rand::rngs::StdRng;

// Modulo para manejar arrays
use ndarray::Array;

/// Representa un punto
#[derive(Debug, PartialEq)]
pub struct Point {
    coordinates: ndarray::Array1<f64>,
}

impl Point {
    pub fn new(coordinates: ndarray::Array1<f64>) -> Self{
        return Self{coordinates};
    }

    /// Genera un punto a partir de sus coordenadas dadas en un vector de flotantes
    pub fn from_vec(coordinates: Vec<f64>) -> Self {
        return Self { coordinates: Array::from(coordinates) };
    }


    /// Dados dos puntos, devuelve su distancia euclidea
    pub fn distance(first: &Self, second: &Self) -> f64{
        return first.distance_to(second);
    }

    /// Genera un punto aleatorio cuyas coordenadas se encuentran siempre en
    /// el intervalo [0, 1]
    pub fn random_point(number_of_coordinates: i32, rng: &mut StdRng) -> Self{

        // Array de ceros
        let coordinates: ndarray::Array1<f64> = ndarray::Array1::zeros(number_of_coordinates as usize);

        // En cada elemento colocamos un valor aleatorio
        let coordinates = coordinates.mapv(|_| rng.gen_range(0.0 .. 1.0));

        return Self{coordinates};

    }

    // Dado otro punto, devuelve su distancia euclidea al punto dado
    fn distance_to(&self, other: &Self) -> f64{
        // Hacemos la diferencia en coordenadas
        // Elevamos al cuadrado
        // Sumamos y devolvemos la raiz n-esima
        let diff = &self.coordinates - &other.coordinates;
        let diff = diff.mapv(|x| x*x);
        return diff.scalar_sum().sqrt();
    }


    /// Dado un conjunto de puntos, calcula su centroide
    /// El vector de puntos debe tener al menos un punto, en otro caso hace panic!
    // TODO -- TEST -- muy facil de testear
    pub fn calculate_centroid(points: &Vec<&Self>) -> Self{
        // Condicion de seguridad
        if points.len() == 0{
            panic!("No se puede calcular el centroide de un conjunto vacio de puntos")
        }

        // Array de zeros con el mismo shape que el primer punto que le pasemos
        let mut sum_point = ndarray::Array1::zeros(points[0].coordinates.len());

        // Calculamos el centroide
        for point in points{
            sum_point = sum_point + &point.coordinates;
        }
        sum_point = sum_point / points.len() as f64;

        return Self{coordinates: sum_point};
    }

    /// Dado un conjunto de puntos, calcula la maxima distancia entre dos de ellos
    pub fn max_distance_among_two(points: &Vec<Point>) -> f64{
        let mut max_dist = 0.0;

        for i in 0 .. points.len(){
            for j in i .. points.len(){
                let curr_dist = Self::distance(&points[i], &points[j]);

                if curr_dist > max_dist{
                    max_dist = curr_dist;
                }
            }
        }

        return max_dist;

    }

    /// Devuelve la dimension del punto
    /// Es decir, el numero de coordenadas del punto
    pub fn dimension(&self) -> usize {
            return self.coordinates.len();
        }
    }

#[cfg(test)]
mod tests{
    use crate::problem_datatypes::Point;

    // Para comprobar que dos soluciones son practicamente iguales (ignorando problemas
    // del punto flotante)
    use assert_approx_eq::assert_approx_eq;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    // Añado este test porque antes tenia un error logico, usando la raiz cuadrada
    // para la distancia en vez de la raiz n-esima. Dejo este test para que
    // no pueda volver a ocurrir
    fn test_simple_distances_are_correct(){
        struct TestCase{
            first: Point,
            second: Point,
            expected_distance: f64
        };


        let test_cases = vec![
            TestCase{
                first: Point::from_vec(vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
                second: Point::from_vec(vec![0.0, 1.0, 0.0, 0.0, 0.0, 0.0]),
                expected_distance: 1.4142135623730951
            },

            TestCase{
                first: Point::from_vec(vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
                second: Point::from_vec(vec![0.5, 0. , 0. , 0. , 0.5, 0. ]),
                expected_distance: 0.7071067811865476
            }
        ];

        for test_case in test_cases{
            let calc_distance = Point::distance(&test_case.first, &test_case.second);
            assert_approx_eq::assert_approx_eq!(calc_distance, test_case.expected_distance, 0.01);
        }

    }

    #[test]
    // Propiedad muy facil de comprobar
    fn test_distance_to_self_is_zero(){
        // Generador de numeros aleatorios. No me hace falta que dependa de la
        // semilla porque solo estamos haciendo tests
        let mut rng = StdRng::seed_from_u64(123141241514);

        let iterations = 1000;
        let coordinates = 6;

        for _ in 0 .. iterations{
            let curr = Point::random_point(coordinates, &mut rng);
            let calc_distance = Point::distance(&curr, &curr);
            let exp_distance = 0.0;

            assert_approx_eq::assert_approx_eq!(calc_distance, exp_distance, 0.01);
        }
    }
}
