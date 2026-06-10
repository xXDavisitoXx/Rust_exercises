fn main() {
    
    use std::io;
    println!("");
    println!("Calculadora en Rust");
    println!("--------------------");
    println!("Por favor introduce el primer número:");
    println!("");
    let mut num1 = String::new();
        io::stdin().read_line(&mut num1).expect("Error al leer la entrada");
        let num1: f64 = num1.trim().parse().expect("Por favor introduce un número válido");

    println!("Por favor introduce el segundo número:");
        println!("");
    let mut num2 = String::new();
        io::stdin().read_line(&mut num2).expect("Error al leer la entrada");
        let num2: f64 = num2.trim().parse().expect("Por favor introduce un número válido");

    println!("Por favor elige la operación que deseas realizar:");
    println!("1. Suma");
    println!("2. Resta");
    println!("3. Multiplicación");
    println!("4. División");
    println!("5. Porcentaje");
    println!("6. Potencia");
    println!("0. Salir");
    println!("");

    let mut opcion = String::new();
        io::stdin().read_line(&mut opcion).expect("Error al leer entrada");
    let opcion: u32 = opcion.trim().parse().expect("Por favor introduce una opción válida");    

    let resultado: f64= match opcion {
        1 => {
            println!("Has elegido Suma");
            num1 + num2
         },
        2 => {
            println!("Has elegido Resta");
            num1 - num2
        },
        3 => {
            println!("Has elegido Multiplicación");
            num1 * num2
        },
        4 => {
            println!("Has elegido División");
            num1 / num2
        },
        5 => {
            println!("Has elegido Porcentaje");
            (num1 / 100.0) * num2
        },
        6 => {
            println!("Has elegido Potencia");
            num1.powf(num2)
        },
        0 => {
            println!("Saliendo de la calculadora. ¡Hasta luego!");
            return;
        },
        _ => {
            println!("Opción no válida");
            return;
        },
    };

    println!("");
   println!("Resultado: {}", resultado);
    println!("");

}
