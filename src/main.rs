
fn main() {
    //declaración de variables locales

    #[derive(Clone, Copy)]
    struct Tablero{
        x_local: [u16; 9],
        o_local: [u16; 9],
        x_global: u16,
        o_global: u16,

    }

    println!("Let's start! First we will randomly assign you X or O, O always starts and X always follows");
    let jugador = eleccion_comienzo();
    println!("You have been assigned: {}", if jugador == 0 {"O"} else {"X"});
}

fn eleccion_comienzo() -> i32{
     rand::random_range(0..=1)
}
