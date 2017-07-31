# Les restrictions

Tout comme les types génériques, les lifetimes (elles-mêmes génériques) utilisent les restrictions. Ici, le caractère `:` a une signification quelque peu différente, mais `+` possède la même fonction. La déclaration se lit comme suit :


1. `T: ‘a` : Toutes les références dans le type `T` doivent, au moins, survivre à la lifetime `‘a` ;
2. `T: Trait + ‘a` : Le type `T` doit implémenter le trait `Trait` et toutes les références doivent survivre à la lifetime `‘a`.

L'exemple ci-dessous illustre les explications précédentes :

{{#playpen source/boundssource0.rs}}

## Voir aussi

[La généricité](../chapitre12/genericite.html), [les restrictions](../chapitre12/restrictions.html) et [les restrictions multiples](../chapitre12/restrictionsmultiples.html).
