fn main() {
    // Dû à l'annotation(suffixe), le compilateur sait que `elem` possède le type 
    // u8.
    let elem = 5u8;

    // Crée un vecteur vide (un tableau dont la taille n'est pas définie).
    let mut vec = Vec::new();
    // A ce niveau, le compilateur ne connaît pas encore le type exact de `vec`,
    // il sait simplement que c'est un vecteur de quelque chose (`Vec<_>`).

    // On ajoute `elem` dans le vecteur.
    vec.push(elem);
    // Tada! Maintenant le compilateur sait que `vec` est un vecteur 
    // d'entiers non-signés typés `u8` (`Vec<u8>`).
    // TODO ^ Essayez de commenter la ligne où se trouve `vec.push(elem)`.

    println!("{:?}", vec);
}