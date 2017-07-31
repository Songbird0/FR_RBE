# Les labels

Le compilateur se sert des « annotations explicites » (labels) pour déterminer la durée de validité d'une référence. Dans le cas où les lifetimes ne peuvent pas être omises (**note**: Elles peuvent l'être grâce au concept [d'élision](../chapitre13/elision.html)), Rust dispose d'annotations explicites (labels) pour déterminer quelle devrait être la durée de vie d'une référence. Les labels sont précédés d'une `‘apostrophe` :

```rust,ignore
foo<'a>
// `foo` dispose de la durée de vie `'a`.
```

Tout comme [les closures](../chapitre8/anontypes.html), pour utiliser les labels, vous devrez avoir recours à la généricité. De plus, cette syntaxe permet de préciser que la durée de vie de `foo` ne peut pas excéder celle de `‘a`.

Voici la syntaxe des labels lorsqu'ils sont appliqués à un type : `&'a T` (ou `&‘a mut T`) où `‘a` est un label déclaré près de l'identificateur de la fonction.

Si vous devez déclarer plusieurs lifetimes, la syntaxe est tout aussi simple :

```rust,ignore
foo<'a, 'b>
// `foo` dispose des lifetimes `'a` et `'b`.
```

Dans ce cas, la durée de vie de `foo` ne peut pas excéder la lifetime `‘a` ou `‘b`.

{{#playpen source/labelssource0.rs}}

## Voir aussi

[La généricité](../chapitre12/genericite.html) et [les closures](../chapitre8/anontypes.html).
