use std::io; // indica que usará a biblioteca std com o modulo io

use rand::prelude::*;

use std::cmp::Ordering;

fn main() { // função principal, primeira a ser executada e onde fica o programa principal
    println!("Guess the number!"); // imprime uma mensagem 

    let secret_number= rand::rng().random_range(1..=100);

    //println!("The secret number is: {secret_number}");
    loop{
        println!("Please input your guess."); // imprime outra mensagem, solicitando ao jogador que digite um número.

        let mut guess = String::new(); // cria uma variável mutável (variáveis em rust são naturalmente imutáveis) e designa ela como uma nova instância do tipo string.

        io::stdin() //chama a função stdin do modulo io
            .read_line(&mut guess) // chama o método read_line. O mut guess indica que ele deve armazenar o input na string mutável guess
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => { 
                                println!("You Win!");
                                break; 
                            }
        }
    }
}
