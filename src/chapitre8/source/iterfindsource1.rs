fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` fournit la référence de chaque élément `&i32` du vecteur.
    let mut iter = vec1.iter();
    // `into_iter()` fournit la valeur de chaque élément du vecteur.
    let mut into_iter = vec2.into_iter();

    // Référence fournie par `iter`: `&&i32`. On déstructure la référence
    // de la référence pour obtenir l'entier `i32`.
    println!("Find 2 in vec1: {:?}", iter     .find(|&&x| x == 2));
    // Référence fournie par `into_iter`: `&i32`. On déstructure la référence 
    // pour obtenir l'entier `i32`.
    println!("Find 2 in vec2: {:?}", into_iter.find(| &x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` fournit la référence de chaque élément du tableau `&i32`.
    println!("Find 2 in array1: {:?}", array1.iter()     .find(|&&x| x == 2));
    // `into_iter()` fournit, exceptionnellement, la référence de chaque élément
    // du tableau `&i32` (le type i32 implémente les traits requis).
    println!("Find 2 in array2: {:?}", array2.into_iter().find(|&&x| x == 2));
}
