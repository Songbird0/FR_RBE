# Exemple d'utilisation : Petite précision

Nous allons créer une méthode chargée de calculer dans deux unités de mesure différentes ([le pied](https://fr.wikipedia.org/wiki/Pied_%28unité%29) et [le millimètre](https://fr.wikipedia.org/wiki/Mètre#Description_des_sous-multiples)) et nous implémenterons le `trait` `Add` avec un type générique fantôme. Voici l'implémentation du `trait` `Add` :

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

[Le système d'emprunts](../chapitre13/borrowing.html), [les restrictions](../chapitre12/restrictions.html), [les énumérations](../chapitre3/enum.html), [impl et self](../chapitre8/methodes.html), [la surcharge des opérateurs](../chapitre14/opoverloading.html), [le pattern `ref`](../chapitre13/refpattern.html), [les `traits`](../chapitre14/traits.html) et [les tuples](../chapitre3/struct.html).
