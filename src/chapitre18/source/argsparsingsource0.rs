use std::env;

fn increase(number: i32) {
    println!("{}", number + 1);
}

fn decrease(number: i32) {
    println!("{}", number - 1);
}

fn help() {
    println!("usage:
match_args <string>
    Check whether given string is the answer.
match_args {{increase|decrease}} <integer>
    Increase or decrease given integer by one.");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // Pas d'arguments passés.
        1 => {
            println!("My name is 'match_args'. Try passing some arguments!");
        },
        // Un seul argument passé.
        2 => {
            match args[1].parse() {
                Ok(42) => println!("This is the answer!"),
                _ => println!("This is not the answer."),
            }
        },
        // Une commande et un argument passé.
        3 => {
            let cmd = &args[1];
            let num = &args[2];
            // On traite le nombre.
            let number: i32 = match num.parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    println!("error: second argument not an integer");
                    help();
                    return;
                },
            };
            // On traite la commande.
            match &cmd[..] {
                "increase" => increase(number),
                "decrease" => decrease(number),
                _ => {
                    println!("error: invalid command");
                    help();
                },
            }
        },
        // all the other cases
        // On couvre tous les autres cas...
        _ => {
            // ... en affichant l'aide.
            help();
        }
    }
}
