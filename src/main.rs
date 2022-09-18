use std::io;

fn main() {
   loop {
    println!("Enter A: ");
    let a: f64 = {
        let mut number = String::new();
        io::stdin().read_line( &mut number).expect("error");
        let nn:f64 = match number.trim().parse(){
            Ok(num) => num,
            Err(_) => 0.0,
        };
        nn
    };
    println!("\n print B: ");
    let b = {
        let mut number = String::new();
        io::stdin().read_line( &mut number).expect("error");
        let nn:f64 = match number.trim().parse(){
            Ok(num) => num,
            Err(_) => 0.0,
        };
        nn
    };
    println!("\n use one of these: + - * /");
    let sigs = {
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("error");
        let nn:f64 = match number.trim().parse(){
            Ok(num) => num,
            Err(_) => 0.0,
        };
        nn
    };
   }
}
