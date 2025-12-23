
#[derive(Clone, Copy)]
struct Board{
    x_local: [u16; 9],
    o_local: [u16; 9],
    x_global: u16,
    o_global: u16,

}

impl Board {
   pub fn empty() -> Self {
       Board {
            x_local: [0u16; 9],
            o_local: [0u16; 9],
            x_global: 0,
            o_global: 0,
       }
   } 
}

fn eleccion_comienzo() -> i32{
     rand::random_range(0..=1)
}

fn main() {
    //declaración de variables locales

    let board = Board::empty();
    println!("Let's start! First we will randomly assign you X or O, O always starts and X always follows");
    let jugador = eleccion_comienzo();
    println!("You have been assigned: {}", if jugador == 0 {"O"} else {"X"});
}


