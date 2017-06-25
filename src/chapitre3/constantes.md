# Les constantes

Rust possède deux types de constantes qui peuvent être déclarées dans n'importe quel « scope » global.

Chacun dispose d'un mot-clé :

* `const`: Une valeur immuable (état par défaut de toute variable);

* `static`: Une variable pouvant être accédée en lecture et (accessoirement) en écriture possédant la « lifetime » `‘static`.

Exception pour les `"chaînes de caractères"` littérales qui peuvent être directement assignées à une variable statique sans modification de votre part, car leur type `&'static str` dispose déjà de la lifetime `‘static`. Tous les autres types de référence doivent être explicitement annotés pour étendre leur durée de vie.

{{#playpen source/constantessource0.rs}}

### Voir aussi

La RFC des mot-clés [const et static](https://github.com/rust-lang/rfcs/blob/master/text/0246-const-vs-static.md), [la lifetime `'static`](../chapitre13/static.html).