# Prácticas Metaheurísticas

* Repositorio para desarrollar las prácticas de la asignatura Metaheurísticas
* A continuación, el contenido LEEME para los profesores:

# Fichero de instrucciones para los profesores de practicas

## Instalación de Rust

* `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` y seguir las instrucciones
* `rustup update`: por si algún componente se encuentra desactualizado

## Ejecución del programa sin compilar

* Como se indica en el guion de las practicas, en este directorio BIN solo incluimos el binario ejecutable y los ficheros de datos
* El script launch_all_programs de este directorio BIN es capaz de lanzar el programa con todos las distintas semillas y casos del problema
* Para lanzar el programa, debemos especificar los siguientes parametros:
    * `./PracticasMetaheuristicas <data_file> <constraints_file> <seed> <number_of_clusters> <search_type>`
    * Donde tenemos los siguientes valores: `<search_type>: copkmeans | copkmeans_robust | local_search`
* Toda esta información se muestra cuando lanzamos el programa sin parámetros o con unos valores erróneos de los parámetros

## Ejecución del programa previa compilación por parte del profesor

* Para ello tendremos que ir al directorio FUENTES
* Para compilar el programa, lanzamos: `cargo build --release` lo que genera el fichero `software/FUENTES/target/release/PracticasMetaheuristicas`
* Podemos ejecutar el programa directamente usando `cargo run --release <parametros de entrada>`
* En el directorio FUENTES, el script `launch_all_programs`, además de lanzar todos los casos y semillas del problema, se encarga de compilar el programa

## Contenidos de la carpeta FUENTES

* src: código de las prácticas escrito en Rust
* data: ficheros de datos y restricciones
* analisis: ficheros excel y de otro tipo usados a la hora de escribir la memoria de las prácticas. Incluye un makefile para generar la memoria que se entrega en este zip
* target: directorios donde se guardan los ficheros binarios generados en los procesos de compilacion
* salida_datos.txt: salida del programa a partir de una ejecución de `launch_all_programs`, a partir de la cual hemos escrito la tabla de excel y el análisis de la memoria (la arquitectura y otros factores influye en que a pesar de usar las mismas semillas la generación de números aleatorios puede ser distinta)
* graficas.py: script que hemos usado para generar las gráficas de las prácticas
