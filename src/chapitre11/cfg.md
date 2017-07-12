# L'attribut `cfg`

La compilation conditionnelle est possible grâce à deux opérateurs:


1. L'attribut cfg: `#[cfg(…)]`;
2. La macro `cfg!`: `cfg!(…)` en tant qu'expression booléenne.

Tous deux possèdent la même syntaxe :

{{#playpen source/cfgsource0.rs}}

## Voir aussi

[La référence de l'attribut cfg](https://doc.rust-lang.org/reference/attributes.html#conditional-compilation), [std::cfg!](https://doc.rust-lang.org/std/macro.cfg.html) et [les macros](../chapitre15/systememacro.html).
