fn main() {
    use std::mem;

    let color = "green";

    // Une closure destinée à afficher la variable `color` qui emprunte 
    // (`&`) `color` et stocke l'emprunt ainsi que la closure 
    // dans la variable `print`. Elle (`color`) restera "empruntée" 
    // jusqu'à ce que l'exécution de `print` prend fin. 
    // La macro println! ne fait qu'emprunter les ressources, cela 
    // n'impose pas de contraintes supplémentaires pour la closure.
    let print = || println!("`color`: {}", color);

    // On appelle la closure en empruntant `color`.
    print();
    print();

    let mut count = 0;

    // Une closure qui incrémente la variable `count`. Cette dernière
    // pourrait être exploitée par référence `&mut count` ou valeur
    // `count`, mais puisque `&mut count` est moins restrictif la capture 
    // par référence mutable sera choisie.
    //
    // La variable `inc` est annotée comme `mut` car une référence mutable 
    // `&mut` est stockée à l'intérieur. 
    // Appeler la closure(du moins, dans ce contexte) modifie 
    // son propre état et donc requiert un `mut`.
    
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // Appelle la closure.
    inc();
    inc();

    // let reborrow = &mut count;
    // ^ TODO: Essayez de décommenter cette ligne.

    // Un entier non-copiable.
    let movable = Box::new(3);

    // `mem::drop` prend possession de ses paramètres. 
    // Un type pouvant être copié devrait être copié dans la closure, 
    // laissant la ressource originale intacte. Un type qui ne peut pas 
    // être copié doit être déplacé et donc `movable` appartiendra à la closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` prend possession de la variable et ne peut donc être appelée qu'une seule fois.
    consume();
    // consume();
    // ^ TODO: Essayez de décommenter ce second appel.
}
