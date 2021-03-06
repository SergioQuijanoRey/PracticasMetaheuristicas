use crate::problem_datatypes;
use crate::problem_datatypes::ConstraintType;
use crate::problem_datatypes::Constraints;
use crate::problem_datatypes::DataPoints;
use crate::problem_datatypes::Point;
use crate::problem_datatypes::Solution;
use crate::fitness_evolution::FitnessEvolution;
use crate::arg_parser::ProgramParameters;
use crate::arg_parser::SearchType;
use crate::utils;

use rand::rngs::StdRng;
use std::process::exit; // Para hacer shuffle de un vector
use rand::seq::SliceRandom;
use std::time::Instant;

/// Lanza el algoritmo y muestra los resultados (solucion, tiempos...)
/// Esto para que la funcion main no sea demasiado grande
pub fn run_and_show_results(data_points: &DataPoints, constraints: &Constraints, program_arguments: ProgramParameters, rng: &mut StdRng, robust: bool){
    // Realizamos la busqueda greedy
    //
    // Si devuelve None, es porque la generacion aleatoria de centroides ha dejado
    // clusters sin elementos, y hay que repetir el algoritmo
    //
    // Estoy contabilizando el tiempo que perdemos cuando tenemos que repetir la asignacion de
    // centroides aleatorios, pero gracias a que devolvemos Option<Solution> esto es muy facil de
    // cambiar
    //
    // Permitimos un numero maximo de reseteos para evitar ciclar infinitamente
    let before = Instant::now();
    let mut greedy_solution: Option<problem_datatypes::Solution>;
    let mut fitness_evolution: FitnessEvolution;
    let max_resets = 100;
    let mut current_reset = 0;
    loop {
        let (greedy_result, fit_result) = run(&data_points, &constraints, program_arguments.get_number_of_clusters(), rng, robust);
        greedy_solution = greedy_result;
        fitness_evolution = fit_result;

        match greedy_solution {
            // Hemos contrado solucion, paramos de iterar
            Some(_) => break,

            // No hemos encontrado solucion, por lo que no hacemos nada, lo que provoca que sigamos
            // iterando
            None => (),
        }

            current_reset = current_reset + 1;
            if current_reset == max_resets{
                println!("--> Se han agotado los {} reseteos maximos por dejar clusters vacios", max_resets);
                break;
            }
    }
    let after = Instant::now();

    // Calculamos la duracion en el formato que se nos especifica
    let duration = after.duration_since(before);
    let duration_numeric = duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9;

    // Tomamos la solucion del Option
    let greedy_solution = match greedy_solution{
        Some(sol) => sol,
        None => {
            println!("Como hemos agotado todos los reseteos, no podemos mostrar m??tricas");
            exit(-1);
        }
    };

    // Para que no sea mutable
    let duration_numeric = duration_numeric;

    // Mostramos los resultados
    println!("==> Busqueda greedy");
    println!("La distancia global instracluster de la solucion es: {}", greedy_solution.global_cluster_mean_distance());
    println!("Las distancias intraclusters son:");
    println!("El numero de restricciones violadas (infeasibility) es: {}", greedy_solution.infeasibility());
    println!("El valor de fitness es: {}", greedy_solution.fitness());
    println!("El valor de lambda es: {}", greedy_solution.get_lambda());
    println!("Tiempo transcurrido (segundos): {}", duration_numeric);
    println!("Salvado del fitness: {:?}", fitness_evolution.save_as_numpy_file(&utils::generate_file_name(SearchType::LocalSearch)));
    println!("");


}

/// Ejecuta la metaheuristica de busqueda local y devuelve la solucion encontrada
/// Si robust es true, entonces aplicamos como centroides iniciales puntos
/// del dataset, como nos indica el profesor de pr??cticas, porque asi es mas robusto
fn run<'a, 'b>(
    data_points: &'a DataPoints,
    constraints: &'b Constraints,
    number_of_clusters: i32,
    rng: &mut StdRng,
    robust: bool
) -> (Option<Solution<'a, 'b>>, FitnessEvolution) {
    // Para llevar la cuenta de como evoluciona el valor del fitness de las
    // iteraciones obtenidas
    let mut fitness_evolution = FitnessEvolution::new();

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
    // Si robust = false, entonces son centroides completamente aleatorios
    // En otro caso, son puntos de data_points aleatorios
    let mut current_centroids = generate_random_centroids(number_of_clusters, point_dimension, data_points, rng, robust);

    // Solucion inicial que en cuanto iteremos una vez vamos a sobreescribir
    // Ahora solo nos interesa considerar los indices de los clusters
    //
    // Notar que esta solucion no es valida porque deja todos los clusters
    // menos uno vacio
    //
    // Lo que vamos a hacer es ir modificando esta solucion. Necesitamos dos variables para ir
    // comparando como cambian, y parar en caso de que no cambien
    let mut current_cluster_indixes: Vec<u32> = vec![0; data_points.len() as usize];

    // Iteramos hasta que los centroides no cambien
    let mut centroids_have_changed = true;

    // Controlamos las iteracione en caso de que robust = true
    // En otro caso, iteramos hasta que los centroides no cambien
    let max_iterations = 50;
    let mut curr_iteration = 0;

    while centroids_have_changed == true && curr_iteration < max_iterations{

        // Realizamos una nueva asignacion de clusters. Recorremos los puntos aleatoriamente y
        // asignando al cluster que menos restricciones viole en esa iteracion. En caso de empates,
        // se toma el cluster con centroide mas cercano
        let new_cluster_indixes = assign_points_to_clusters(
            &data_points,
            &constraints,
            &current_centroids,
            &current_cluster_indixes,
            number_of_clusters,
            rng,
        );

        // Antes de calcular los centroides debemos comprobar que no haya ningun
        // cluster sin puntos. Esto puede ocurrir en la primera pasada en la que
        // generamos centroides aleatorios. No se si esto es exclusivo de la primera
        // iteracion con centroides aleatorios
        if valid_cluster_configuration(&new_cluster_indixes, number_of_clusters) == false {
            // Para mostrar algunos datos de la solucion problematica
            let tmp_solution = Solution::new(new_cluster_indixes.clone(), data_points, constraints, number_of_clusters);

            eprintln!("[Err] La solucion greedy actual ha dejado clusters sin puntos");
            eprintln!(
                "Estos clusters vacios son: {:?}",
                get_cluster_without_point_indixes(&new_cluster_indixes, number_of_clusters)
            );
            eprintln!("Restricciones violadas: {:?}", tmp_solution.infeasibility());
            eprintln!("Devolvemos una solucion vacia para que se vuelva a iniciar el algoritmo con otros centroides aleatorios");
            eprintln!("Este contratiempo cuenta en el tiempo de ejecucion del algoritmo");

            // Devuelvo Option::None para que desde el punto en el que se llama
            // al algoritmo, se reinicie la b??squeda y se tome la decision de si
            // contabilizar el tiempo extra de volver a genera una primera solucion
            // aleatoria o si no contabilizarlo (mas control al caller)
            return (None, FitnessEvolution::new());
        }

        // Calculamos los nuevos centroides
        let new_centroids =
            calculate_new_centroids(&new_cluster_indixes, &data_points, number_of_clusters);

        // Comprobamos si los centroides han cambiado
        centroids_have_changed = centroids_are_different(&current_centroids, &new_centroids);

        // Cambiamos a la nueva asignacion de clusters y los nuevos centroides
        current_cluster_indixes = new_cluster_indixes;
        current_centroids = new_centroids;


        // Calculamos la solucion actual para tener el fitness en esta iteracion
        let curr_sol = Solution::new(
            current_cluster_indixes.clone(),
            data_points,
            constraints,
            number_of_clusters,
        );
        fitness_evolution.add_iteration(curr_sol.fitness());

        // En caso de que robust = true, acotamos el numero de iteraciones de forma
        // efectiva aumentando el contador. En otro caso, al no tocar el contador
        // no estamos teniendo en cuenta este parametro
        if robust == true{
            curr_iteration = curr_iteration + 1;
        }
    }

    // Mostramos si hemos acabado por agotar las iteraciones cuando robust = true
    if robust == true && curr_iteration > 0{
        println!("--> Hemos acabado copkmeans al agotar las {} iteraciones maximas", max_iterations);
    }

    // Devuelvo la solucion a partir del vector de asignacion de clusters y la cuenta
    // de la evolucion del fitness
    return (Some(Solution::new(
        current_cluster_indixes,
        data_points,
        constraints,
        number_of_clusters,
    )), fitness_evolution);
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
/// para asignar al punto pasado
// current_cluster_indixes lo necesito para saber cual es la configuracion actual
// de los puntos y tomar una decision en base a ello
// Selecciona el cluster que menos aumento de violaciones de restricciones produce
// En caso de que haya empate, se toma el cluster mas cercano al punto
// TODO -- borrar las comprobaciones de seguridad
fn select_best_cluster(
    current_cluster_indixes: &Vec<u32>,
    number_of_clusters: i32,
    constraints: &Constraints,
    current_point_index: u32,
    current_point: &Point,
    centroids: &Vec<Point>,
) -> u32 {
    // Calculo las restricciones que se violan cuando asinamos al punto representado
    // por current_point_index a cada uno de los clusters
    let violated_constraints = get_violated_constraints_per_cluster_assignment(
        current_cluster_indixes,
        number_of_clusters,
        constraints,
        current_point_index,
    );

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
        return min_cluster_indixes[0] as u32;
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
    return min_cluster_indixes[min_index as usize] as u32;
}

/// Calcula un vector con las restricciones que se viola al realizar cada una
/// de las asignaciones de un punto fijado (current_point_index) a cada uno de los
/// posibles clusters
/// Es decir, vector de violaciones consecuencia de asgnar el punto current_point_index
/// al cluster i-esimo
fn get_violated_constraints_per_cluster_assignment(
    current_cluster_indixes: &Vec<u32>,
    number_of_clusters: i32,
    constraints: &Constraints,
    current_point_index: u32,
) -> Vec<u32> {
    // Vector que construimos con las restricciones violadas
    let mut violated_constraints = vec![];

    for cluster_candidate in 0..number_of_clusters as u32 {
        // Calculo el numero de restricciones violadas para este cluster en concreto
        // Para ello, itero sobre los indices de los puntos y los clusters a los
        // que estan asignados dichos puntos
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

    return violated_constraints;
}

/// Genera los centroides de forma aleatoria
/// Si robust = false, son completamente aleatorio. En otro caso, se toman datos
/// aleatorios del conjunto de puntos
fn generate_random_centroids(
    number_of_clusters: i32,
    point_dimension: i32,
    data_points: &DataPoints,
    rng: &mut StdRng,
    robust: bool
) -> Vec<Point> {

    if robust == false{
        return generate_random_centroids_randomly(number_of_clusters,point_dimension,rng);
    }else{
        return generate_random_centroids_from_data_points(number_of_clusters, data_points, rng);
    }
}

/// Genera los centroides de forma aleatoria
/// Como los puntos del problema estan normalizados en el intervalo [0, 1]^2, los
/// centroides aleatorios estar??n en dicho intervalo
/// El cluster i-esimo tiene centroide el punto i-esimo del vector
fn generate_random_centroids_randomly(
    number_of_clusters: i32,
    point_dimension: i32,
    rng: &mut StdRng,
) -> Vec<Point> {
    let mut centroids = vec![];

    for _ in 0..number_of_clusters {
        // Genero un punto aleatorio que sera el centroide actual
        let current_centroid = Point::random_point(point_dimension, rng);

        // Lo a??ado al vector de centroides
        centroids.push(current_centroid);
    }

    return centroids;
}

/// Genera una lista de centroides aleatorios como puntos aleatorios de los puntos
/// pasados como parametros
fn generate_random_centroids_from_data_points(
    number_of_clusters: i32,
    data_points: &DataPoints,
    rng: &mut StdRng,
) -> Vec<Point> {
    let mut centroids = vec![];

    // Genero indices aleatorios de los que vamos a tomar los puntos y los mezclamos
    let mut indexes: Vec<u32> = (0 as u32.. data_points.len() as u32).collect();
    indexes.shuffle(rng);

    for i in 0..number_of_clusters {
        let curr_rand_index = indexes[i as usize];
        let selected_point_as_centroid = data_points.get_points()[curr_rand_index as usize].clone();
        centroids.push(selected_point_as_centroid);
    }

    return centroids;
}

/// A partir de una asignacion de clusters y un conjunto de datos, calcula los
/// centroides correspondientes a dicha asignacion con dichos puntos
fn calculate_new_centroids(
    cluster_indixes: &Vec<u32>,
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
    for cluster in 0..number_of_clusters as u32 {
        // Tomamos los puntos que pertenecen a este cluster
        let cluster_points = tmp_solution.get_points_in_cluster(cluster);

        // A??adimos el centroide de ese conjunto de puntos
        new_centroids.push(Point::calculate_centroid(&cluster_points));
    }

    return new_centroids;
}

/// Comprueba que una configuracion de clusters sea valida
/// Una configuracion es invalida cuando hay un cluster sin puntos
/// Es decir, cuando un elemento de {0, 1, ..., number_of_clusters} no aparece
/// en ninguna posicion de cluster_indixes
fn valid_cluster_configuration(cluster_indixes: &Vec<u32>, number_of_clusters: i32) -> bool {
    let cluster_without_point_indixes =
        get_cluster_without_point_indixes(&cluster_indixes, number_of_clusters);
    return cluster_without_point_indixes.len() == 0;
}

/// Toma los indices de clusters que no tienen puntos asignados
/// Por ejemplo, {3, 4} porque ambos clusters no tienen ni un solo punto asignado
fn get_cluster_without_point_indixes(
    cluster_indixes: &Vec<u32>,
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
/// Devuelve el vector que representa la asignacion de cada punto a su cluster
fn assign_points_to_clusters(
    data_points: &DataPoints,
    constraints: &Constraints,
    current_centroids: &Vec<Point>,
    current_cluster_indixes: &Vec<u32>,
    number_of_clusters: i32,
    rng: &mut StdRng,
) -> Vec<u32> {
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
            index,
            &data_points.get_points()[index as usize],
            &current_centroids,
        ) as i32;
    }

    // Devuelvo los indices. Pero para ello primero tengo que hacer la conversion
    // de vector de i32 a vector de u32
    return new_cluster_indixes.into_iter().map(|x| x as u32).collect();
}

#[cfg(test)]
mod tests{
    use crate::copkmeans::centroids_are_different;
    use crate::copkmeans::get_violated_constraints_per_cluster_assignment;
    use crate::copkmeans::select_best_cluster;
    use crate::problem_datatypes::Point;
    use crate::problem_datatypes::Constraints;
    use crate::problem_datatypes::ConstraintType;

    #[test]
    fn test_centroids_are_different(){
        let first_centroids = vec![
            Point::from_vec(vec![0.0, 1.0, 0.0, 1.0]),
            Point::from_vec(vec![1.0, 1.0, 0.0, 1.0]),
            Point::from_vec(vec![1.0, 1.0, 1.0, 1.0]),
        ];
        let second_centroids = vec![
            Point::from_vec(vec![0.0, 1.0, 0.0, 1.0]),
            Point::from_vec(vec![1.0, 1.0, 0.0, 1.0]),
            Point::from_vec(vec![1.0, 1.0, 1.0, 1.0]),
        ];
        let calc_diff = centroids_are_different(&first_centroids, &second_centroids);
        let exp_diff = false;
        assert_eq!(calc_diff, exp_diff);

        let second_centroids = vec![
            Point::from_vec(vec![1.0, 1.0, 0.0, 1.0]),
            Point::from_vec(vec![1.0, 1.0, 0.0, 1.0]),
            Point::from_vec(vec![1.0, 1.0, 1.0, 1.0]),
        ];
        let calc_diff = centroids_are_different(&first_centroids, &second_centroids);
        let exp_diff = true;
        assert_eq!(calc_diff, exp_diff);

        let first_centroids = vec![
            Point::from_vec(vec![1.0, 1.0, 0.0, 1.0]),
            Point::from_vec(vec![1.0, 1.0, 0.0, 1.0]),
            Point::from_vec(vec![1.0, 1.0, 1.0, 1.0]),
        ];
        let calc_diff = centroids_are_different(&first_centroids, &second_centroids);
        let exp_diff = false;
        assert_eq!(calc_diff, exp_diff);

        let first_centroids = vec![
            Point::from_vec(vec![1.0, 1.0, 0.0, 1.0]),
            Point::from_vec(vec![1.0, 1.0, 0.0, 1.0]),
            Point::from_vec(vec![1.0, 1.0, 1.1, 1.0]),
        ];
        let calc_diff = centroids_are_different(&first_centroids, &second_centroids);
        let exp_diff = true;
        assert_eq!(calc_diff, exp_diff);

    }

    #[test]
    fn test_get_violated_constraints_per_cluster_assignment(){
        let current_cluster_indixes = vec![2, 0, 1, 2, 3];
        let number_of_clusters = 4;
        let mut constraints = Constraints::new();
        constraints.add_constraint(0, 1, ConstraintType::MustLink);
        constraints.add_constraint(3, 4, ConstraintType::CannotLink);

        let calc_violated_constraints_per_cluster = get_violated_constraints_per_cluster_assignment(&current_cluster_indixes, number_of_clusters, &constraints, 1);
        let exp_violated_constraints_per_cluster = vec![1, 1, 0, 1];
        assert_eq!(calc_violated_constraints_per_cluster, exp_violated_constraints_per_cluster);

        let calc_violated_constraints_per_cluster = get_violated_constraints_per_cluster_assignment(&current_cluster_indixes, number_of_clusters, &constraints, 3);
        let exp_violated_constraints_per_cluster = vec![0, 0, 0, 1];
        assert_eq!(calc_violated_constraints_per_cluster, exp_violated_constraints_per_cluster);
    }

    #[test]
    fn test_select_best_cluster(){
        let current_cluster_indixes = vec![0, 1, 1, 2, 3];
        let number_of_clusters = 4;
        let mut constraints = Constraints::new();
        constraints.add_constraint(0, 1, ConstraintType::MustLink);
        let current_point_index = 0;
        let centroids = vec![Point::from_vec(vec![0.0]),Point::from_vec(vec![0.0]),Point::from_vec(vec![0.0]),Point::from_vec(vec![0.0]),Point::from_vec(vec![0.0])];

        let calc_best_cluster = select_best_cluster(&current_cluster_indixes, number_of_clusters, &constraints, current_point_index, &centroids[0], &centroids);
        let exp_best_cluster = 1;
        assert_eq!(calc_best_cluster, exp_best_cluster);
    }
}
