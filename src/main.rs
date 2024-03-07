#![windows_subsystem = "windows"]

use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Adivinhe o número!");
    let secret_number:u32 = rand::thread_rng().gen_range(1..=100);

   
    println!("Por favor, insira seu palpite.");

    let mut guess_count = 0;

    let mut is_first_iteration = true;

    loop {
        
        if is_first_iteration {
            is_first_iteration = false;
        } else {
            println!("Vamos tentar novamente, insira seu palpite.");
        }

        let mut guess:String = String::new();

        io::stdin().read_line(&mut guess)
        .expect("Falha ao ler a linha");

        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(err) => {
                println!("Erro: {}", err);
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            }, 
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Less => println!("Muito baixo!"),
        };

        guess_count +=1;
    }

    println!("Você acertou em {} tentativas!", guess_count);
}
