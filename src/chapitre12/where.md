# La clause `where`

Une restriction peut également être explicitée par la clause `where`. Cette dernière se trouvera alors avant l'accolade ouvrante (`{`) plutôt qu'à la déclaration du type(e.g. `<A: Display, B: Debug, ...>`). Avec `where`, vous pouvez également ajouter arbitrairement d'autres types en plus de spécifier les traits à implémenter pour les types génériques.

`where` peut être utile dans plusieurs cas :


* Lorsque vous ajoutez des restrictions aux types génériques, facilitant la lecture :

```rust,ignore
impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}

// Les restrictions sont explicitées 
// par la condition `where`.
impl <A, D> MyTrait<A, D> for YourType where
    A: TraitB + TraitC,
    D: TraitE + TraitF {}
```

* Lorsque d'autres types sont ajoutés aux restrictions :

{{#playpen source/wheresource0.rs}}

{{#playpen source/wheresource1.rs}}

## Voir aussi

[RFC pour la condition/clause `where`](https://github.com/rust-lang/rfcs/blob/master/text/0135-where.md), [les structures](../chapitre3/struct.html), [les traits](../chapitre14/traits.html).
