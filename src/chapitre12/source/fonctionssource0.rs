struct A;          // Le type `A` est concret.
struct S(A);       // Le type `S` est concret.
struct SGen<T>(T); // Le type `SGen` est générique.

// Toutes les fonctions qui suivront possèdent les paramètres 
// qui leur sont passés et libèrent la mémoire aussitôt.

// On définit une fonction nommée `reg_fn` qui prend un argument `_s` de type 
// `S`. Ce dernier n'est pas précédé d'un `<T>` (ou, en l'occurrence un `<S>`)
// donc le type n'est pas générique.
fn reg_fn(_s: S) {}

// On définit une fonction nommée `gen_spec_t` qui prend un argument `_s` de type 
// `SGen<T>`.
// Le type d'argument imposé à la structure SGen<T> est précisé, mais puisque 
// `A` n'est pas précédé par le type générique `<A>`, le type n'est pas générique.
fn gen_spec_t(_s: SGen<A>) {}
//          ^ il aurait fallu déclarer `A` comme générique: `<A>`.

// On définit une fonction nommée `gen_spec_i32` qui prend un argument `_s` de 
// type `SGen<i32>`. 
// Le type de paramètre supporté par la structure SGen a été spécifié, `i32`.
// Puisque `i32` n'est pas un type générique, cette fonction n'est pas générique non plus.
fn gen_spec_i32(_s: SGen<i32>) {}

// On définit une fonction nommée `generic` qui prend un paramètre `_s` de type 
// `SGen<T>`. 
// Puisque `SGen<T>` est précédé par le type générique `<T>`, 
// cette fonction est donc générique.
fn generic<T>(_s: SGen<T>) {}

fn main() {
    // Appel des fonctions qui ne sont pas génériques.
    reg_fn(S(A));          // On passe en paramètre un type concret.
    gen_spec_t(SGen(A));   // Type `A` implicitement spécifié car la fonction 
                           // n'accepte que la signature `SGen<A>`.
    gen_spec_i32(SGen(6)); // Type `i32` implicitement spécifié car la fonction 
                           // n'accepte que la signature `SGen<i32>`.

    // Type du paramètre explicitement spécifié pour 
    // la fonction `generic()`.
    // Le type peut être omis car le compilateur peut inférer 
    // le type à partir du littéral.
    generic::<char>(SGen('a'));

    // Type du paramètre implicitement spécifié pour la fonction 
    // `generic()` car le compilateur est capable d'inférer le type 
    // à partir du littéral.
    generic(SGen('c'));
}
