use rand::Rng;
use std::io;

fn main() {
    // Sortear um número entre 1 e 100
    let numero_secreto: u8 = rand::thread_rng().gen_range(1..=100);

    println!("Bem-vindo ao jogo de adivinhação!");

    loop {
        println!("Digite um número entre 1 e 100:");

        // Ler a tentativa do jogador
        let mut tentativa = String::new();
        io::stdin().read_line(&mut tentativa).expect("Falha ao ler a linha");

        // Tentar converter a entrada para um número inteiro
        let tentativa: u8 = match tentativa.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Tem que ser um número inteiro!");
                continue;
            }
        };
        if tentativa < 1 || tentativa > 100 {
            println!("Tem que ser entre 1 e 100!");
            continue;
        }

        // Verificar a tentativa
        if tentativa == numero_secreto {
            println!("Adivinhou mizeravi!");
            break;
        } else if tentativa < numero_secreto {
            println!("Muito baixo!");
        } else {
            println!("Muito alto!");
        }
    }
}
