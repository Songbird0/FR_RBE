// A est un type concret.
struct A;

// Lorsque nous déclarons `Single`, la première occurrence de `A` n'est 
// pas précédée du type générique `<A>`. Le type `Single` et `A` sont donc 
// concrets.
struct Single(A);
//            ^ Voici la première occurrence du type `A`.

// En revanche, ici, `<T>` précède la première occurrence `T`, donc le type 
// `SingleGen` est générique. Puisque le type `T` est générique, cela pourrait être 
// "n'importe quoi", y compris le type concret `A` déclaré au début du fichier.
struct SingleGen<T>(T);

fn main() {
    // `Single` est un type concret et prend explicitement un paramètre 
    // de type `A`.
    let _s = Single(A);

    // On créé une variable nommée `_char` de type `SingleGen<char>`
    // et on lui assigne la valeur `SingleGen('a')`.
    // Le type requis du paramètre passé pour cette instance de `SingleGen` 
    // est spécifié, mais il peut être omis, exemple ---
    let _char: SingleGen<char> = SingleGen('a');

    // --->
    let _t    = SingleGen(A); // On passe une instance 
                              // du type `A` définit en haut.
    let _i32  = SingleGen(6); // On passe un entier de type `i32`.
    let _char = SingleGen('a'); // On passe un `char`.
}
