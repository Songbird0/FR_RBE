fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()`, pour les vecteurs, fournit la référence de chaque 
    // élément `&i32`. 
    println!("2 in vec1: {}", vec1.iter()     .any(|&x| x == 2));
    // `into_iter()`, pour les vecteurs, fournit la valeur de chaque élément `i32`.
    // L'itérateur est consommé.
    println!("2 in vec2: {}", vec2.into_iter().any(| x| x == 2));
    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` fournit la référence de chaque élément du tableau `&i32`.
    println!("2 in array1: {}", array1.iter()     .any(|&x| x == 2));
    // `into_iter()` fournit, exceptionnellement, la référence de chaque élément
    // du tableau `&i32` (le type i32 implémente les traits requis).
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));
}
