struct Fibonacci {
    curr: u32,
    next: u32,
}

// Implémentation de `Iterator` pour le type `Fibonacci`.
// Le trait `Iterator` nécessite l'implémentation d'une méthode seulement pour l'élément `next`.
impl Iterator for Fibonacci {
    type Item = u32;

    // Ici, nous définissons la séquence utilisant `.curr` et `.next`.
    // Le type de renvoi est `Option<T>`:
    //     * Lorsque l'`Iterator` est terminé, `None` est renvoyé;
    //     * Autrement, la valeur suivante est enveloppé dans une instance `Some` et renvoyée.
    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        // Puisqu'il n'y a pas de limite à une suite de Fibonacci, l'`Iterator` 
        // ne renverra jamais `None`.
        Some(self.curr)
    }
}

// Renvoie un générateur de suites de Fibonacci.
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 1, next: 1 }
}

fn main() {
    // `0..3` est un `Iterator` qui génère: 0, 1, et 2 (i.e. intervalle `[0, 3[`).
    let mut sequence = 0..3;

    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());


    // `for` parcourt un `Iterator` jusqu'à ce qu'il renvoie `None`.
    // Chaque valeur contenue par un objet `Some` est libérée de son conteneur 
    // puis assignée à une variable (en l'occurrence, `i`).
    println!("Iterate through 0..3 using `for`");
    for i in 0..3 {
        println!("> {}", i);
    }

    // La méthode `take(n)` réduit un `Iterator` à ces `n` premiers éléments.
    println!("The first four terms of the Fibonacci sequence are: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    // La méthode `skip(n)` tronque les `n` premiers éléments d'un `Iterator`.
    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];

    // La méthode `iter` construit un `Iterator` sur un(e) tableau/slice.
    println!("Iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }
}
