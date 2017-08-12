# L'attribut Derive

Le compilateur est capable de fournir de simples implémentations pour cetains traits par le biais de [l'attribut][attribute] `#[derive]`. 
Ces traits peuvent toujours être implémentés manuellement si un traitement plus complexe est attendu.

Voici une liste de traits pouvant être dérivés:

* Les traits de comparaison: [`Eq`][eq], [`PartialEq`][partial_eq], [`Ord`][ord], [`PartialOrd`][partial_ord];
* [`Clone`][clone], pour créer une instance (`T`) à partir d'une référence (`&T`) par copie;
* [`Copy`][copy], pour permettre à un type d'être copié plutôt que transféré;
* [`Hash`][hash], pour générer un hash depuis `&T`;
* [`Debug`][debug], pour formater une valeur en utilisant le formatteur `{:?}`.

{{#playpen source/derivesource0.rs}}

## Voir aussi

[Derive][derive].

[attribute]: ../chapitre11/attributes.html
[eq]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
[partial_eq]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
[ord]: https://doc.rust-lang.org/std/cmp/trait.Ord.html
[partial_ord]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
[clone]: https://doc.rust-lang.org/std/clone/trait.Clone.html
[copy]: https://doc.rust-lang.org/core/marker/trait.Copy.html
[hash]: https://doc.rust-lang.org/std/hash/trait.Hash.html
[debug]: https://doc.rust-lang.org/std/fmt/trait.Debug.html
[derive]: https://doc.rust-lang.org/reference/attributes.html#derive
