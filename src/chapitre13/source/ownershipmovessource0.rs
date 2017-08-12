// Cette fonction prend possession d'un entier alloué dans le tas.
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // `c` est détruit et la mémoire va être libérée.
}

fn main() {
    // Entier alloué dans la pile.
    let x = 5u32;

    // On copie `x` dans `y` - aucune ressource n'a été transféré
    // (l'ownership n'a pas été transféré).
    let y = x;

    // Les deux valeurs peuvent être utilisées indépendamment.
    println!("x is {}, and y is {}", x, y);

    // `a` est un pointeur sur un entier alloué dans le tas.
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // On transfert `a` dans `b`.
    let b = a;
    // L'adresse du pointeur `a` est copié (et non la donnée) dans `b`.
    // `a` et `b` sont désormais des pointeurs sur la même donnée allouée dans le 
    // tas, mais `b` la possède, désormais.

    // Erreur! `a` ne peut plus accéder à la donnée car il ne possède plus 
    // le bloc mémoire.
    // println!("a contains: {}", a);
    // TODO ^ Essayez de décommenter cette ligne.

    // Cette fonction prend possession de la mémoire allouée dans le tas 
    // à partir de `b`.
    destroy_box(b);

    // Puisque la mémoire allouée a été libérée à partir d'ici, 
    // cette action consisterait à déréférencer de la mémoire libérée, 
    // mais cela est interdit par le compilateur.
    // Erreur! `b` ne peut plus accéder à la donnée car il ne possède plus 
    // le bloc mémoire.
    // println!("b contains: {}", b);
    // TODO ^ Essayez de décommenter cette ligne.
}
