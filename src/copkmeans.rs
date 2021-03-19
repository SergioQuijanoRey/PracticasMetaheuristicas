use crate::problem_datatypes::ConstraintType;
use crate::problem_datatypes::Constraints;
use crate::problem_datatypes::DataPoints;
use crate::problem_datatypes::Point;
use crate::problem_datatypes::Solution;
use rand::seq::SliceRandom;
use std::process::exit; // Para hacer shuffle de un vector
use rand::rngs::StdRng;

// TODO -- TEST -- este modulo puede tener muchos errores porque es muy enrrevesado

/// Ejecuta la metaheuristica de busqueda local y devuelve la solucion encontrada
pub fn run<'a, 'b>(
    data_points: &'a DataPoints,
    constraints: &'b Constraints,
    number_of_clusters: i32,
    rng: &mut StdRng
) -> Option<Solution<'a, 'b>> {

    // Numero de coordenadas que componen cada uno de los puntos
    // Necesario para saber cuantas coordenadas debe tener nuestros centroides aleatorios
    let point_dimension = match data_points.point_dimension() {
        Some(dimension) => dimension as i32,
        None => {
            eprintln!(
                "[Err] la estructura DataPoints no tiene puntos de los que obtener su dimension"
            );
            eprintln!("No se puede continuar con la busqueda greedy");
            exit(-1);
        }
    };

    // Tomamos los centroides aleatorios
    let mut current_centroids = generate_random_centroids(number_of_clusters, point_dimension);

    // Solucion inicial que en cuanto iteremos una vez vamos a sobreescribir
    // Ahora solo nos interesa considerar los indices de los clusters
    //
    // Notar que esta solucion no es valida porque deja todos los clusters
    // menos uno vacio
    //
    // Lo que vamos a hacer es ir modificando esta solucion. Necesitamos dos variables para ir
    // comparando como cambian, y parar en caso de que no cambien
    let mut current_cluster_indixes = vec![0; data_points.len() as usize];

    // Iteramos hasta que los centroides no cambien
    let mut centroids_have_changed = true;
    while centroids_have_changed == true {

        // Realizamos una nueva asignacion de clusters. Recorremos los puntos aleatoriamente y
        // asignando al cluster que menos restricciones viole en esa iteracion. En caso de empates,
        // se toma el cluster con centroide mas cercano
        let new_cluster_indixes = assign_points_to_clusters(&data_points, &constraints, &current_centroids, &current_cluster_indixes, number_of_clusters, rng);

        // Antes de calcular los centroides debemos comprobar que no haya ningun
        // cluster sin puntos. Esto puede ocurrir en la primera pasada en la que
        // generamos centroides aleatorios. No se si esto es exclusivo de la primera
        // iteracion con centroides aleatorios
        if valid_cluster_configuration(&new_cluster_indixes, number_of_clusters) == false {
            eprintln!("[Err] La solucion greedy actual ha dejado clusters sin puntos");
            eprintln!(
                "Estos clusters vacios son: {:?}",
                get_cluster_without_point_indixes(&new_cluster_indixes, number_of_clusters)
            );
            eprintln!("Devolvemos una solucion vacia para que se vuelva a iniciar el algoritmo con otros centroides aleatorios");
            eprintln!("Este contratiempo no lo contamos en el tiempo de ejecucion");

            // Devuelvo Option::None para que desde el punto en el que se llama
            // al algoritmo, se reinicie la búsqueda y se tome la decision de si
            // contabilizar el tiempo extra de volver a genera una primera solucion
            // aleatoria o si no contabilizarlo (mas control al caller)
            return None;
        }

        // Calculamos los nuevos centroides
        let new_centroids =
            calculate_new_centroids(&new_cluster_indixes, &data_points, number_of_clusters);

        // Comprobamos si los centroides han cambiado
        centroids_have_changed = centroids_are_different(&current_centroids, &new_centroids);

        // Cambiamos a la nueva asignacion de clusters y los nuevos centroides
        current_cluster_indixes = new_cluster_indixes;
        current_centroids = new_centroids;
    }

    // Convierto los tipos del vector de clusters
    let current_cluster_indixes = current_cluster_indixes
        .into_iter()
        .map(|x| x as i32)
        .collect();

    // Devuelvo la solucion a partir del vector de asignacion de clusters
    return Some(Solution::new(
        current_cluster_indixes,
        data_points,
        constraints,
        number_of_clusters,
    ));
}

/// Comprueba si dados dos conjuntos de centroides, estos son diferentes o no
/// Esto es util para saber si alguno de los centroides ha cambiado en el proceso,
/// y por tanto, si alguna asignacion de cluster ha cambiado o no
fn centroids_are_different(past_centroids: &Vec<Point>, new_centroids: &Vec<Point>) -> bool {
    for index in 0..past_centroids.len() {
        if (new_centroids[index] == past_centroids[index]) == false {
            return true;
        }
    }
    return false;
}

/// Dado un punto y una configuracion de puntos actual, elige el mejor cluster posible
// current_cluster_indixes lo necesito para saber cual es la configuracion actual
// de los puntos
fn select_best_cluster(
    current_cluster_indixes: &Vec<i32>,
    number_of_clusters: i32,
    constraints: &Constraints,
    current_point_index: i32,
    current_point: &Point,
    centroids: &Vec<Point>,
) -> i32 {
    // Calculo las restricciones que se violan por cada asignacion de cluster
    let mut violated_constraints = vec![];
    for cluster_candidate in 0..number_of_clusters {
        // Calculo el numero de restricciones violadas
        let mut current_violations = 0;
        for (point_index, point_cluster) in current_cluster_indixes.iter().enumerate() {
            // Miramos que restriccion tenemos entre los dos puntos
            match constraints.get_constraint(point_index as i32, current_point_index as i32) {
                // Hay restriccion
                // Tenemos que comprobar con otro match segun el tipo de restriccion que sea
                Some(constraint) => {
                    match constraint {
                        // Sumamos uno si el candidato a cluster no coincide
                        // con el cluster del punto
                        ConstraintType::MustLink => {
                            if *point_cluster != cluster_candidate {
                                current_violations += 1;
                            }
                        }

                        // Sumamos uno si el candidato a cluster coincide con
                        // el cluster del punto
                        ConstraintType::CannotLink => {
                            if *point_cluster == cluster_candidate {
                                current_violations += 1;
                            }
                        }
                    }
                }

                // No hay restricciones entre los dos puntos asi que no tenemos que hacer
                // comprobaciones
                None => (),
            }
        }

        // Asigno el numero al vector
        violated_constraints.push(current_violations);
    }

    // Calculo el valor minimo de violaciones que produce una asignacion de cluster
    let min_value = match violated_constraints.iter().min() {
        Some(value) => value,
        None => {
            eprint!("[Err] No se pudo encontrar el minimo del numero de violaciones que supone una asignacion de cluster");
            eprintln!(
                "El vector de restricciones violadas es: {:?}",
                violated_constraints
            );
            exit(-1);
        }
    };

    // Calculo los clusters cuya asignacion produce el minimo numero de violaciones
    // Este vector guarda los indices de los ya mencionados clusters, por ejemplo:
    // min_cluster_indixes = vec![3, 4, 8]
    let mut min_cluster_indixes: Vec<i32> = vec![];
    for cluster in 0..number_of_clusters {
        if violated_constraints[cluster as usize] == *min_value {
            min_cluster_indixes.push(cluster);
        }
    }

    // Tomo la mejor asignaciom
    // Unico elemento con minimo valor
    if min_cluster_indixes.len() == 1 {
        return min_cluster_indixes[0];
    }

    // No hay un unico elemento, tengo que calcular las distancias y quedarme con la minima
    // Calculo las distancias
    let mut distances = vec![];
    for cluster_candidate in &min_cluster_indixes {
        let distance_to_centroid =
            Point::distance(current_point, &centroids[*cluster_candidate as usize]);
        distances.push(distance_to_centroid);
    }

    // Calculo la minima distancia y el indice del valor minimo
    // TODO -- mejorar el naming porque es algo lioso cuando hago el return
    let mut min_distance = distances[0];
    let mut min_index = 0;
    for (index, distance) in distances.iter().enumerate() {
        if *distance < min_distance {
            min_distance = *distance;
            min_index = index;
        }
    }

    // Devuelvo el indice que da la minima distancia
    return min_cluster_indixes[min_index as usize];
}

/// Genera los centroides de forma aleatoria
/// Como los puntos del problema estan normalizados en el intervalo [0, 1]^2, los
/// centroides aleatorios estarán en dicho intervalo
/// El cluster i-esimo tiene centroide el punto i-esimo del vector
fn generate_random_centroids(number_of_clusters: i32, point_dimension: i32) -> Vec<Point> {
    let mut centroids = vec![];

    for _ in 0..number_of_clusters {
        // Genero un punto aleatorio que sera el centroide actual
        let current_centroid = Point::random_point(point_dimension);

        // Lo añado al vector de centroides
        centroids.push(current_centroid);
    }

    return centroids;
}

/// A partir de una asignacion de clusters y un conjunto de datos, calcula los
/// centroides correspondientes a dicha asignacion con dichos puntos
fn calculate_new_centroids(
    cluster_indixes: &Vec<i32>,
    data_points: &DataPoints,
    number_of_clusters: i32,
) -> Vec<Point> {
    // Generamos un struct Solution para usar algunos de sus metodos
    // Las restricciones no me interesan para estos metodos, por tanto las dejo vacias
    let constraints = Constraints::new();
    let tmp_solution = Solution::new(
        cluster_indixes.clone(),
        data_points,
        &constraints,
        number_of_clusters,
    );

    let mut new_centroids = vec![];
    for cluster in 0..number_of_clusters {
        // Tomamos los puntos que pertenecen a este cluster
        let cluster_points = tmp_solution.get_points_in_cluster(cluster);

        // Añadimos el centroide de ese conjunto de puntos
        new_centroids.push(Point::calculate_centroid(&cluster_points));
    }

    return new_centroids;
}

/// Comprueba que una configuracion de clusters sea valida
/// Una configuracion es invalida cuando hay un cluster sin puntos
/// Es decir, cuando un elemento de {0, 1, ..., number_of_clusters} no aparece
/// en ninguna posicion de cluster_indixes
fn valid_cluster_configuration(cluster_indixes: &Vec<i32>, number_of_clusters: i32) -> bool {
    let cluster_without_point_indixes =
        get_cluster_without_point_indixes(&cluster_indixes, number_of_clusters);
    return cluster_without_point_indixes.len() == 0;
}

/// Toma los indices de clusters que no tienen puntos asignados
/// Por ejemplo, {3, 4} porque ambos clusters no tienen ni un solo punto asignado
fn get_cluster_without_point_indixes(
    cluster_indixes: &Vec<i32>,
    number_of_clusters: i32,
) -> Vec<i32> {
    // Generamos todos los clusters en un vector
    let mut cluster_without_point_indixes: Vec<i32> = (0..number_of_clusters).into_iter().collect();

    // Marcamos los clusters con elementos con un -1
    for cluster in cluster_indixes {
        cluster_without_point_indixes[*cluster as usize] = -1;
    }

    // Devolvemos los clusters que no han sido marcados, y por tanto, que no tienen puntos
    // asociados
    let cluster_without_point_indixes: Vec<i32> = cluster_without_point_indixes
        .into_iter()
        .filter(|&value| value != -1)
        .collect();
    return cluster_without_point_indixes;
}

/// Asigna, en orden aleatorio, los puntos a los clusters asociados a los centroides que pasamos
/// como parametro. Para ello, da prioridad a las restricciones que se violan en cada paso. En caso
/// de empate, se toma el cluster con el centroide mas cercano
fn assign_points_to_clusters(data_points: &DataPoints, constraints: &Constraints, current_centroids: &Vec<Point>, current_cluster_indixes: &Vec<i32>, number_of_clusters: i32, rng: &mut StdRng) -> Vec<i32>{
    // Realizamos una nueva asignacion de clusters
    // -1 para saber que puntos todavia no han sido asignados a un cluster
    let mut new_cluster_indixes: Vec<i32> = vec![-1; data_points.len() as usize];


    // Recorremos aleatoriamente los puntos para irlos asignando a cada cluster
    let mut point_indexes: Vec<u32> = (0..data_points.len() as u32).collect();
    point_indexes.shuffle(rng);

    for index in point_indexes {
        // Calculo el cluster al que asignamos el punto actual
        new_cluster_indixes[index as usize] = select_best_cluster(
            &current_cluster_indixes,
            number_of_clusters,
            &constraints,
            index as i32,
            &data_points.get_points()[index as usize],
            &current_centroids,
            );
    }

    return new_cluster_indixes;
}
