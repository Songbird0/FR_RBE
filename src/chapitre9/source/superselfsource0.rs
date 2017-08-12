fn function() {
    println!("called `function()`");
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }

    pub fn indirect_call() {
        // Appelons toutes les fonctions nommées `fonction` depuis ce 
        // contexte!
        print!("called `my::indirect_call()`, that\n> ");

        // Le mot-clé `self` fait référence au module courant (en l'occurrence 
        // `my`). Appeler `self::function()` ou `function()` revient exactement 
        // au même, puisqu'ils font tous deux référence à la même fonction.
        self::function();
        function();

        // Vous pouvez également utiliser `self` pour accéder à un autre module 
        // imbriqué dans `my`.
        self::cool::function();

        // Le mot-clé `super` fait référence au contexte parent (en-dehors du 
        // module `my`, pour cet exemple).
        super::function();

        // Si vous ne spécifiez aucun des deux mot-clés, le compilateur 
        // comprendra que vous essayez d'utiliser une ressource se trouvant 
        // dans le contexte de la crate.
        // On va donc assigner un nouvel identificateur à `cool::function()` se 
        // trouvant dans le contexte de la crate.
        // Le contexte de la crate représente tout ce qui se trouve en-dehors des modules.
        {
            use cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}
