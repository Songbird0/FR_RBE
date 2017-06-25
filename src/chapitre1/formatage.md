# Formatage

Nous avons vu que le formatage désiré était spécifié par des « chaînes de formatage » :


* `format!("{}", foo) -> "3735928559"` ;
* `format!("0x{:X}", foo) -> "0xDEADBEEF"` ;
* `format!("0o{:o}", foo) -> "0o33653337357"`.

La même variable (`foo`) peut être formatée de différentes manières suivant le type d'argument utilisé dans le marqueur (e.g. `X`, `o`, rien).

Cette fonctionnalité est implémentée à l'aide de traits, et il y en a un pour chaque type d'argument. Le plus commun est, bien entendu, `Display`. Il est chargé de gérer les cas où le type d'argument n'est pas spécifié (i.e. `{}`).

{{#playpen source/formatagesource0.rs}}

N'hésitez pas à consulter [la liste complète des traits](http://doc.rust-lang.org/std/fmt/#formatting-traits) dédiés au formatage ainsi que leurs types d'argument dans la documentation du module [std::fmt](http://doc.rust-lang.org/std/fmt/).

### Activité

Implémentez le trait `fmt::Display` pour la structure `Color` dans l'exemple ci-dessus de manière à obtenir un résultat identique à celui-ci:

```text
RGB (128, 255, 90) 0x80FF5A
RGB (0, 3, 254) 0x0003FE
RGB (0, 0, 0) 0x000000
```

Indices:


* Vous pourriez [avoir besoin d'itérer plusieurs fois](http://doc.rust-lang.org/std/fmt/#argument-types) sur vos couleurs;
* Vous pouvez [créer une « compensation »](http://doc.rust-lang.org/std/fmt/#width) (remplissant votre chaîne de zéros) d'une largeur `n` avec `:0n`.

### Voir aussi

[std::fmt](http://doc.rust-lang.org/std/fmt/)