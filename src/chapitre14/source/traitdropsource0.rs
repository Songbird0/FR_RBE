struct Droppable {
    name: &'static str,
}

// Implémentation basique de `drop` qui affiche un message dans la console.
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main() {
    let _a = Droppable { name: "a" };

    // block A
    {
        let _b = Droppable { name: "b" };

        // block B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Exiting block B");
        }
        println!("Just exited block B");

        println!("Exiting block A");
    }
    println!("Just exited block A");

    // La variable peut être libérée manuellement en utilisant la fonction `(std::mem::)drop`.
    drop(_a);
    // TODO Essayez de commenter cette ligne.

    println!("end of the main function");

    // `_a` ne *sera pas* libérée une seconde fois ici puisque nous l'avons déjà fait 
    // plus haut, manuellement.
}
