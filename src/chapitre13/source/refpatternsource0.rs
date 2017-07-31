#[derive(Clone, Copy)]
struct Point { x: i32, y: i32 }

fn main() {
    let c = 'Q';

    // Un emprunt effectué avec le mot-clé `ref` (placé dans la l-value) 
    // est équivalent à une assignation "C-like" avec le `&` (placé dans la r-value).
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point { x: 0, y: 0 };

    // `ref` peut également être utilisé lors de la déstructuration 
    // d'une structure (aussi valable pour les tuples(structures et littéraux)).
    let _copy_of_x = {
        // `ref_to_x` est une référence du champ `x` contenu dans 
        // l'instance `point`.
        let Point { x: ref ref_to_x, y: _ } = point;

        // Renvoie une copie du champ `x` de l'instance `point`.
        *ref_to_x
    };

    // Copie mutable de l'instance `point`.
    let mut mutable_point = point;

    {
        // `ref` peut être combiné avec `mut` pour récupérer 
        // une référence mutable.
        let Point { x: _, y: ref mut mut_ref_to_y } = mutable_point;

        // On modifie le champ `y` de l'instance `mutable_point` par 
        // le biais de la référence mutable qu'on a récupéré.
        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);

    // Un tuple mutable qui contient un pointeur et un entier non-signé 
    // codé sur 32 bits.
    let mut mutable_tuple = (Box::new(5u32), 3u32);

    {
        // Nous déstructurons le tuple `mutable_tuple` pour modifier 
        // la valeur de `last` (dernière élément indexé par le tuple).
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
        // `_` est ajouté dans la valeur de gauche car on ne souhaite 
        // pas en récupérer le contenu.
    }

    println!("tuple is {:?}", mutable_tuple);
}
