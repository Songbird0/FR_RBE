fn main() {
    // Les éléments des itérateurs peuvent être collectés et 
    // ajoutés dans un vecteur.
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    // La macro `vec!` peut être utilisée pour initialiser un vecteur.
    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    // On ajoute un nouvel élément à la fin du vecteur.
    println!("Push 4 into the vector");
    xs.push(4);
    println!("Vector: {:?}", xs);

    // Error! Immutable vectors can't grow
    // Erreur! Les vecteurs immuables ne peuvent pas être 
    // agrandis.
    // collected_iterator.push(0);
    // FIXME ^ Commentez/décommentez cette ligne

    // La méthode `len` renvoie la taille actuelle du vecteur.
    println!("Vector size: {}", xs.len());

    // L'indexation peut être faite à l'aide des "[]" (l'indexaction débute à 0).
    println!("Second element: {}", xs[1]);

    // `pop` supprime le dernier élément du vecteur et le renvoie.
    println!("Pop last element: {:?}", xs.pop());

    // Une indexaction hors des capacités du vecteur 
    // mène un plantage du programme.
    println!("Fourth element: {}", xs[3]);
}
