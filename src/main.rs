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
    global: usize,
    local: usize, // starts at top left as 0, ends in bottom right as 8
    piece: char,
    is_machine: bool,
}

impl LastMove {
   pub fn empty() -> Self {
       LastMove {
            global: 9,
            local: 9,
            piece: ' ',
            is_machine: false,
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

   pub fn is_local_board_available(&self, local: usize, last_pos: usize) -> bool {
       if local > 8 { return false; }
       
       let x_won = (self.x_global & (1u16 << local)) != 0;
       let o_won = (self.o_global & (1u16 << local)) != 0;
       if x_won || o_won { return false; }  
       last_pos == local
   }

}

fn select_start() -> i32{
     rand::random_range(0..=1)
}

fn get_move_local(board: &Board, local_board: usize) -> usize {  
    loop {  
        let mut input_str = String::new();
        println!("Choose a cell in the local board: {} (0-8):", local_board);  
        io::stdin().read_line(&mut input_str).expect("Error");

        match input_str.trim().parse::<usize>() {
            Ok(num) if num <= 8 && board.is_free(local_board, num) => return num,  
            Ok(_) => println!("Invalid cell or occupied! (0-8)"),
            Err(_) => println!("Invalid number!"),
        }
    }
}

fn get_move_global(board: &Board, last_pos: usize) -> usize {  
    loop {
        let mut input_str = String::new();
        println!("Choose a local board (0-8):");
        io::stdin().read_line(&mut input_str).expect("Error");

        match input_str.trim().parse::<usize>() {
            Ok(num) if num <= 8 && board.is_local_board_available(num, last_pos) => return num,  
            Ok(_) => println!("Invalid local board or unavailable!"),
            Err(_) => println!("Invalid number!"),
        }
    }
} 

fn make_move(last_move: &mut LastMove, board: &mut Board){
    //por hacer
    //1. si es el primer movimiento se juega donde se quiera, para esto necesito tener una forma
    //   global de saber de quien es el turno
    let local;
    let global;
    if last_move.local == 9 || last_move.global == 9 {
       global = get_move_global(board, 4); 
       local = get_move_local(board, global);
       last_move.local = local;
       last_move.global = global;
    }
}

fn write_board_local(last_move: &mut LastMove, board: &mut Board, local: &usize, global: &usize){
    if last_move.piece == 'O' {
        board.x_local[*global] |= 1u16 << *local;
    }
    else if last_move.piece == 'X' {
        board.o_local[*global] = 1u16 << *local;
    }
}

fn main() {
    //declaración de variables locales

    let mut board = Board::empty();
    let mut last_move = LastMove::empty();
    println!("Let's start! First we will randomly assign you X or O, O always starts and X always follows");
    let jugador = select_start();
    last_move.piece= if jugador == 1 {'O'} else {'X'}; //logica al reves para funciones posteriores
    println!("You have been assigned: {}", if jugador == 0 {'O'} else {'X'});
}


