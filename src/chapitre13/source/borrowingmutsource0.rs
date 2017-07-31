#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // `&'static str` est une référence d'une chaîne de caractères allouée 
    // dans un bloc mémoire qui ne peut être accédé qu'en lecture.
    author: &'static str,
    title: &'static str,
    year: u32,
}

// Cette fonction prend une référence d'une instance 
// de la structure `Book` en paramètre.
fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

// Cette fonction prend une référence mutable d'une instance de la 
// structure `Book` en paramètre, et initialise sa date de publication 
// à 2014.
fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main() {
    // On créé une instance immuable de la 
    // structure `Book` nommée `immutabook`.
    let immutabook = Book {
        // Les chaînes littérales sont typées `&'static str`.
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    // On créé une copie mutable de `immutabook` nommée
    // `mutabook`.
    let mut mutabook = immutabook;

    // Emprunte un objet en lecture seule.
    borrow_book(&immutabook);

    // Emprunte un objet en lecture seule.
    borrow_book(&mutabook);

    // Récupère une référence mutable d'un objet mutable.
    new_edition(&mut mutabook);

    // Erreur! Vous ne pouvez pas récupérer une référence 
    // mutable d'un objet immuable.
    // new_edition(&mut immutabook);
    // FIXME ^ Décommentez cette ligne.
}
