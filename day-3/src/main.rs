use std::io::stdin;

fn main() {
    let mut gifts: Vec<String> = vec![];
    println!();
    loop {
        print!("{esc}c", esc = 27 as char);
        println!("/--- Lista de regalos ---/");
        if gifts.is_empty() {
            println!("No agregaste ningun regalo, probá agregar uno!")
        } else {
            for gift in gifts.iter() {
                print!("⭐ - {}", gift);
            }
        }
        let input_reader = stdin();
        let mut new_gift: String = String::new();
        println!();
        println!("> Agrega un regalo (o apreta 0 para salir)");

        input_reader.read_line(&mut new_gift).unwrap();
        if new_gift.as_str().trim() != "0" {
            gifts.push(new_gift);
        } else {
            println!("Feliz navidad!");
            break;
        }
    }
}
