# Les types associés

L'utilisation des « types associés » améliore la lisibilité du code en assignant les types génériques, au sein du trait, comme « types de sortie ». La syntaxe pour les déclarer est la suivante :

```rust,ignore
// `A` et `B` sont déclarés au sein du trait par le biais du mot-clé `type`.
// Notez toutefois que `type`, dans ce contexte, n'a pas la même fonction 
// que le `type` utilisé pour créer des alias.
trait Contains {
    type A;
    type B;

    // La syntaxe a été modifiée pour faire référence aux types génériques.
    fn contains(&self, &Self::A, &Self::B) -> bool;
}
```

Notez que les fonctions qui utilisent le trait `Contains` n'ont plus du tout besoin de déclarer les types génériques `A` et `B` :

```rust,ignore
// Sans les types associés.
fn difference<A, B, C>(container: &C) -> i32 where
    C: Contains<A, B> { ... }

// Avec les types associés.
fn difference<C: Contains>(container: &C) -> i32 { ... }
```

Éditons l'exemple de la section précédente en utilisant les types associés :

{{#playpen source/typesassociessource0.rs}}
