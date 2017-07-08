# Les crates

Une `crate` est une [unité de compilation](https://www.techopedia.com/definition/23963/compilation-unit-programming "https://www.techopedia.com/definition/23963/compilation-unit-programming"), en Rust. Lorsque `rustc some_file.rs` est appelé, `some_file.rs` est considéré comme étant un «fichier paquet» (i.e. un fichier fédérant les autres). Si `some_file.rs` possède plusieurs modules en son sein, chacun d'entre eux verra son contenu fusionné dans ce paquet avant que la compilation n'ait lieu. Autrement dit, les modules ne sont pas compilés individuellement, mais en tant que paquet, en tant qu'ensemble de ressources.

Une `crate` peut être compilée en tant qu'exécutable ou bibliothèque. Par défaut, `rustc` produira un exécutable mais cela peut être modifié en passant le flag `--crate-type` au compilateur.
