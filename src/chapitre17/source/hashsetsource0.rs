use std::collections::HashSet;

fn main() {
    let mut a: HashSet<i32> = vec!(1i32, 2, 3).into_iter().collect();
    let mut b: HashSet<i32> = vec!(2i32, 3, 4).into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));

    // `HashSet::insert()` renvoie false si 
    // une valeur était déjà présente.
    // assert!(b.insert(4), "Value 4 is already in set B!");
    // FIXME ^ Commentez/décommentez cette ligne

    b.insert(5);

    // Si le type d'un élément de la collection implémente le trait `Debug`,
    // alors la collection devra, elle aussi, implémenter `Debug`.
    // Elle affiche généralement ses éléments dans le format `[elem1, elem2, ...]`.
    println!("A: {:?}", a);
    println!("B: {:?}", b);

    // Affiche [1, 2, 3, 4, 5] dans un ordre arbitraire.
    println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());

    // Ceci devrait afficher [1].
    println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());

    // Affiche [2, 3, 4] dans un ordre arbitraire.
    println!("Intersection: {:?}", a.intersection(&b).collect::<Vec<&i32>>());

    // Affiche [1, 5].
    println!("Symmetric Difference: {:?}",
             a.symmetric_difference(&b).collect::<Vec<&i32>>());
}
