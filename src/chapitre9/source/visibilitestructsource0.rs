mod my {
    // Une structure publique avec un champ public générique de type `T`.
    pub struct WhiteBox<T> {
        pub contents: T,
    }

    // Une structure publique avec un champ privé générique de type `T`.
    #[allow(dead_code)]
    pub struct BlackBox<T> {
        contents: T,
    }

    impl<T> BlackBox<T> {
        // Constructeur public.
        pub fn new(contents: T) -> BlackBox<T> {
            BlackBox {
                contents: contents,
            }
        }
    }
}

fn main() {
    // Les structures publiques possédant des champs publics 
    // peuvent être instanciées avec les séparateurs `{}`.
    let white_box = my::WhiteBox { contents: "public information" };

    // et leurs champs peuvent être sollicités normalement.
    println!("The white box contains: {}", white_box.contents);

    // Les structures publiques composées de champs privés ne peuvent pas être 
    // instanciées de manière "classique" (i.e. en précisant le nom des champs).
    // Erreur! `BlackBox` possèdent des champs privés.
    // let black_box = my::BlackBox { contents: "classified information" };
    // TODO ^ Essayez de décommenter cette ligne.

    // En revanche, elles peuvent être créées avec un constructeur public.
    let _black_box = my::BlackBox::new("classified information");

    // Les champs privés d'une structure publique ne peuvent pas être 
    // sollicités directement.
    // Erreur! Le champ `contents` est privé.
    // println!("The black box contains: {}", _black_box.contents);
    // TODO ^ Essayez de décommenter cette ligne.
}
