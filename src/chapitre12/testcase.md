# Exemple d'utilisation : Petite précision

Nous allons créer une méthode chargée de calculer dans deux unités de mesure différentes ([le pied][pied] et [le millimètre][mm]) et nous implémenterons le `trait` `Add` avec un type générique fantôme. Voici l'implémentation du `trait` `Add` :

```rust,ignore
// Cette implémentation devrait imposer: `Self + RHS = Output`
// où `Self` est la valeur par défaut de `RHS` si elle n'est pas spécifiée 
// dans l'implémentation.

pub trait Add<RHS = Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}

// `Output` doit être de type `T<U>` pour que `T<U> + T<U> = T<U>`.
impl<U> Add for T<U> {
    type Output = T<U>;
    ...
}
```

Voici l'implémentation complète :

{{#playpen source/testcasesource0.rs}}

## Voir aussi

[Le système d'emprunts][emprunts], [les restrictions][bounds], [les énumérations][enums], [impl et self][impl_n_self], [la surcharge des opérateurs][overloading], [le pattern `ref`][ref], [les `traits`][traits] et [les tuples][tuples].

[pied]: https://fr.wikipedia.org/wiki/Pied_%28unité%29
[mm]: https://fr.wikipedia.org/wiki/Mètre#Description_des_sous-multiples
[emprunts]: ../chapitre13/borrowing.html
[bounds]: ../chapitre12/restrictions.html
[enums]: ../chapitre3/enum.html
[impl_n_self]: ../chapitre8/methodes.html
[overloading]: ../chapitre14/opoverloading.html
[ref]: ../chapitre13/refpattern.html
[traits]: ../chapitre14/traits.html
[tuples]: ../chapitre3/struct.html
