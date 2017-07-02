// Assigne le chemin `deeply::nested::function` à l'identificateur 
// `other_function`.
use deeply::nested::function as other_function;

fn function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`")
        }
    }
}

fn main() {
    // Accès moins verbeux à `deeply::nested::function`.
    other_function();

    println!("Entering block");
    {
        // Ceci est équivalent à `use deeply::nested::function as function`.
        // La nouvelle assignation `function()` prendra le pas 
        // sur `deeply::nested::function()`.
        use deeply::nested::function;
        function();

        // Les assignations `use` ne sont disponibles que dans le contexte 
        // où elles ont vu le jour. Dans ce cas, où `function()` occulte 
        // le chemin de base, ce "shadowing" n'est effectif que dans ce bloc.
        println!("Leaving block");
    }

    function();
}
