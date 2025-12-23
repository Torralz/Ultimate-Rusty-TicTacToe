use std::io;

#[derive(Clone, Copy)]
struct Board{
    x_local: [u16; 9],
    o_local: [u16; 9],
    x_global: u16,
    o_global: u16,

}

#[derive(Clone, Copy)]
struct LastMove{
    local_board: usize,
    position: usize, // starts at top left as 0, ends in bottom right as 8
}

impl LastMove {
   pub fn empty() -> Self {
       LastMove {
            local_board: 9,
            position: 9,
       }
   } 
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

   pub fn is_free(&self, local: usize, pos: usize) -> bool{
       let mask = 1u16 << pos;
       (self.x_local[local] & mask ) == 0 && (self.o_local[local] & mask ) == 0 
   }
}

fn select_start() -> i32{
     rand::random_range(0..=1)
}

fn get_move(board: &Board, local_board: usize){
    let mut terminado = false;

    while !terminado {
        let mut input_str = String::new();
        println!("Elija una casilla (0-8):");
        io::stdin()
            .read_line(&mut input_str)
            .expect("Error de lectura");

        let input: usize = match input_str.trim().parse() {
            Ok(num) if num >= 0 && num <= 8 && board.is_free(local_board, num)=> num,
            _ => {
                println!("Entrada inválida, intente de nuevo.");
                continue;
            }
        };

        terminado = true;
    }
}

fn make_move(){

}

fn main() {
    //declaración de variables locales

    let mut board = Board::empty();
    println!("Let's start! First we will randomly assign you X or O, O always starts and X always follows");
    let jugador = select_start();
    println!("You have been assigned: {}", if jugador == 0 {"O"} else {"X"});
}


