# Les restrictions

Lorsque nous travaillons avec la généricité, il est courant d'assigner une « restriction » à un type générique pour spécifier quel trait il doit implémenter. Dans l'exemple suivant, nous utilisons le trait `Display` pour afficher quelque chose en console, il est alors assigné au type `T`. Autrement dit, `T` doit implémenter `Display`.

```rust,ignore
// On déclare une fonction nommée `printer` qui prend, en entrée, 
// un type générique `T` qui doit implémenter le trait `Display`.
fn printer<T: Display>(t: T) {
    println!("{}", t);
}
```

Les ressources passées en paramètre sont toutes soumises à ces restrictions et doivent forcément remplir les conditions:

```rust,ignore
struct S<T: Display>(T);

// Erreur! `Vec<T>` n'implémente pas le trait `Display`.
let s = S(vec![1]);
```

Les instances des types génériques peuvent également accéder aux méthodes appartenant au(x) trait(s) présent(s) dans les restrictions du type. Par exemple :

{{#playpen source/restrictionssource0.rs}}

Les conditions d'entrée pour les paramètres génériques peuvent également être spécifiées en utilisant le mot-clé [where](../chapitre12/where.html), les rendant plus explicites, plus lisibles.

## Voir aussi

Les traits dédiés à l'affichage et le formatage [std::fmt](../chapitre1/affichage.html), [les structures](../chapitre3/struct.html) et [les traits](../chapitre14/traits.html).
