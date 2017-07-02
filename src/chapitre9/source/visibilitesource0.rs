// Un module nommé `my`.
mod my {
    // Les items se trouvant dans le module sont privés, par défaut.
    fn private_function() {
        println!("called `my::private_function()`");
    }

    // Utilisez le mot-clé `pub` pour modifier la visibilité par défaut.
    pub fn function() {
        println!("called `my::function()`");
    }

    // Des items se trouvant dans le même module peuvent se solliciter
    // entre-eux, même lorsqu'ils sont privés.
    pub fn indirect_access() {
        print!("called `my::indirect_access()`, that\n> ");
        private_function();
    }

    // Les modules peuvent également être imbriqués.
    pub mod nested {
        pub fn function() {
            println!("called `my::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my::nested::private_function()`");
        }
    }
    // Les modules imbriqués suivent les mêmes règles vis-à-vis de la 
    // visibilité.
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my::private_nested::function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

fn main() {
    // Les noms des modules rattachés à une ressource peuvent être explicités 
    // pour supprimer toute ambiguïté entre deux ressources possédant le même
    // nom.
    function();
    my::function();

    // Les items publiques, y compris ceux qui se trouvent dans les modules 
    // imbriqués, peuvent être sollicités en dehors du module parent.
    my::indirect_access();
    my::nested::function();


    // Les items privés d'un module ne peuvent pas être directement sollicités,
    // même si ils sont imbriqués dans un module publique:

    // Erreur! `private_function` est privée.
    //my::private_function();
    // TODO ^ Essayez de décommenter cette ligne.

    // Erreur! `private_function` est privée.
    //my::nested::private_function();
    // TODO ^ Essayez de décommenter cette ligne.

    // Erreur! `private_nested` est un module privé.
    //my::private_nested::function();
    // TODO ^ Essayez de décommenter cette ligne.
}
