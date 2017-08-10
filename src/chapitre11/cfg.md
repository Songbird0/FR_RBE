# L'attribut `cfg`

La compilation conditionnelle est possible grâce à deux opérateurs:


1. L'attribut cfg: `#[cfg(…)]`;
2. La macro `cfg!`: `cfg!(…)` en tant qu'expression booléenne.

Tous deux possèdent la même syntaxe :

{{#playpen source/cfgsource0.rs}}

## Voir aussi

[La référence de l'attribut cfg][cfg], [std::cfg!][cfg_macro] et [les macros][macros].

[cfg]: https://doc.rust-lang.org/reference/attributes.html#conditional-compilation
[cfg_macro]: https://doc.rust-lang.org/std/macro.cfg.html
[macros]: ../chapitre15/systememacro.html
