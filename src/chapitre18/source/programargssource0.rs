use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Le premier argument est le chemin à partir duquel le programme 
    // a été appelé.
    println!("My path is {}.", args[0]);

    // Le reste des arguments sont ceux passés en ligne de commande au programme.
    // On appelle le programme comme ceci:
    //  $ ./args arg1 arg2
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
}
