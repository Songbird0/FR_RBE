// On créé une constante avec la lifetime `'static` en utilisant 
// le mot-clé `static`.
static NUM: i32 = 18;

// Renvoie une référence de `NUM` où sa lifetime `'static` 
// est obligée de s'aligner avec la durée de vie du paramètre 
// passé à la fonction.
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        // On créé une chaîne de caractères littérale, primitive 
        // et on l'affiche.
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // Lorsque `static_string` sortira du contexte, la référence 
        // ne pourra plus être utilisée, mais la ressource restera 
        // présente dans le binaire.
    }

    {
        // On créé un entier à passer à la 
        // fonction `coerce_static`:
        let lifetime_num = 9;

        // On aligne, adapte la durée de vie de `NUM à 
        // celle de `lifetime_num`:
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);
}
