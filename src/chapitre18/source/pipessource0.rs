use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};

static PANGRAM: &'static str =
"the quick brown fox jumped over the lazy dog\n";

fn main() {
    // On créé un processus dans lequel la commande `wc` va s'exécuter.
    let process = match Command::new("wc")
                                .stdin(Stdio::piped())
                                .stdout(Stdio::piped())
                                .spawn() {
        Err(why) => panic!("couldn't spawn wc: {}", why.description()),
        Ok(process) => process,
    };

    // On écrit quelque chose dans l'entrée standard de `wc`.
    // `stdin` est de type `Option<ChildStdin>`, mais puisque nous savons que 
    // cette instance en possède une, nous pouvons directement l'`unwrap()`.
    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {}",
                           why.description()),
        Ok(_) => println!("sent pangram to wc"),
    }

    // Puisque `stdin` ne survit pas après l'appel du dessus, elle va être libérée et 
    // et le pipe fermé.
    //
    // C'est très important sinon `wc` ne pourrait pas commencer à traiter l'entrée 
    // que nous avons soumis.

    // La champ `stdout` est également de type `Option<ChildStdout>` et doit donc être `unwrap()`.
    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}",
                           why.description()),
        Ok(_) => print!("wc responded with:\n{}", s),
    }
}
