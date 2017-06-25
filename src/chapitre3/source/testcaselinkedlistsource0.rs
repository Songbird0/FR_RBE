use List::*;

enum List {
    // Cons: Un tuple contenant un élément(i.e. u32) et un pointeur vers le noeud suivant (i.e. Box<List>).
    Cons(u32, Box<List>),
    // Nil: Un noeud témoignant de la fin de la liste.
    Nil,
}

// Il est possible de lier, d'implémenter des méthodes 
// pour une énumération.
impl List {
    // Créé une liste vide.
    fn new() -> List {
        // `Nil` est une variante de `List`.
        Nil
    }

    // Consomme, s'approprie la liste et renvoie une copie de cette même liste 
    // avec un nouvel élément ajouté à la suite.
    fn prepend(self, elem: u32) -> List {
        // `Cons` est également une variante de `List`.
        Cons(elem, Box::new(self))
    }

    // Renvoie la longueur de la liste.
    fn len(&self) -> u32 {
        // `self` doit être analysé car le comportement de cette méthode 
        // dépend du type de variante auquel appartient `self`.
        // `self` est de type `&List` et `*self` est de type `List`, rendant 
        // possible l'analyse directe de la ressource plutôt que par le biais d'un alias (e.g. une référence).
        // Pour faire simple: on déréférence `self` avant de l'analyser.
        // Note: Lorsque vous travaillez sur des références, préférez le déréférencement 
        // avant analyse.
        match *self {
            // On ne peut pas prendre "l'ownership", possession de la queue (liste) 
            // puisque l'on emprunte seulement `self` (nous ne le possédons pas);
            // Nous créerons simplement une référence de la queue.
            Cons(_, ref tail) => 1 + tail.len(),
            // De base, une liste vide possède 0 élément.
            Nil => 0
        }
    }

    // Renvoie une représentation de la liste sous une chaîne de caractères 
    // (wrapper)
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` est équivalente à `println!` mais elle renvoie 
                // une chaîne de caractères allouée dans le tas (wrapper) 
                // plutôt que de l'afficher dans la console.
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    // Créé une liste vide.
    let mut list = List::new();

    // On ajoute quelques éléments.
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Affiche l'état définitif de la liste.
    println!("La linked list possède une longueur de: {}", list.len());
    println!("{}", list.stringify());
}