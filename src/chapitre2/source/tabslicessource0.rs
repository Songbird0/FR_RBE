use std::mem;


// Cette fonction emprunte une slice.
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // Tableau dont la taille est connue à la compilation (le type peut être omis).
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // Tous les éléments peuvent être initialisés à la même valeur.
    let ys: [i32; 500] = [0; 500];

    // L'indexation débute à 0.
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` renvoie la taille du tableau.
    println!("array size: {}", xs.len());

    // Les tableaux sont alloués dans la pile.
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Les tableaux peuvent être automatiquement empruntés en tant que 
    // slice.
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Les slices peuvent pointer sur une partie bien précise d'un tableau.
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // Erreur! Le dépassement de tampon fait planter le programme.
    // println!("{}", xs[5]);
}
