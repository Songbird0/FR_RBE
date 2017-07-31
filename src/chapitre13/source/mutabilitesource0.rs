fn main() {
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // Erreur: `immutable_box` ne peut pas être déréférencé.
    //*immutable_box = 4;

    // On créé une copie mutable de `immutable_box`.
    let mut mutable_box = immutable_box;

    println!("mutable_box contains {}", mutable_box);

    // On modifie le contenu de la box.
    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);
}
