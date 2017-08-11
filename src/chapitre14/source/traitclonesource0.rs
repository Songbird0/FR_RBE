// Une structure unitaire sans ressources.
#[derive(Debug, Clone, Copy)]
struct Nil;

// Un tuple avec des ressources qui implémente le trait `Clone`.
#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    // Un instancie `Nil`.
    let nil = Nil;
    // On copie l'objet `Nil`, aucune ressource à transférer.
    let copied_nil = nil;

    // Les deux instances peuvent être utilisées indépendament l'une de l'autre.
    println!("original: {:?}", nil);
    println!("copy: {:?}", copied_nil);

    // On crée un objet `Pair`.
    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    // On copie `pair` dans `moved_pair`, transfert de ressources.
    let moved_pair = pair;
    println!("copy: {:?}", moved_pair);

    // Erreur! `pair` ne possède plus ses ressources.
    // println!("original: {:?}", pair);
    // TODO ^ Essayez de décommenter cette ligne.

    // On copie `moved_pair` dans `cloned_pair` (ressources incluses).
    let cloned_pair = moved_pair.clone();
    // On libère la ressource originale avec `std::mem::drop`.
    drop(moved_pair);

    // Erreur! `moved_pair` a été libérée.
    // println!("copy: {:?}", moved_pair);
    // TODO ^ Essayez de décommenter cette ligne.

    // La copie obtenue par `.clone()` peut toujours être utilisée !
    println!("clone: {:?}", cloned_pair);
}
