
fn main() {
    //declaración de variables locales

    #[derive(Clone, Copy)]
    struct Tablero{
        x_local: [u16; 9],
        o_local: [u16; 9],
        x_global: u16,
        o_global: u16,

    }
    println!("Hello, world!");
}

fn eleccion_comienzo() -> i32{
     rand::random_range(0..=1)
}
