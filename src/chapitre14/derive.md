# L'attribut Derive

Le compilateur est capable de fournir de simples implémentations pour cetains traits par le biais de [l'attribut](../chapitre11/attributes.html) `#[derive]`. 
Ces traits peuvent toujours être implémentés manuellement si un traitement plus complexe est attendu.

Voici une liste de traits pouvant être dérivés:

* Les traits de comparaison: [`Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html), [`PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html), [`Ord`](https://doc.rust-lang.org/std/cmp/trait.Ord.html), [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html);
* [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html), pour créer une instance (`T`) à partir d'une référence (`&T`) par copie;
* [`Copy`](https://doc.rust-lang.org/core/marker/trait.Copy.html), pour permettre à un type d'être copié plutôt que transféré;
* [`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html), pour générer un hash depuis `&T`;
* [`Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html), pour formater une valeur en utilisant le formatteur `{:?}`.

{{#playpen source/derivesource0.rs}}

## Voir aussi

[Derive](https://doc.rust-lang.org/reference/attributes.html#derive).
