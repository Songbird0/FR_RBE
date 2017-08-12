# Les chaînes de caractères

Il y a deux types de chaînes de caractères en Rust: `String` et `&str`.

Une instance de `String` est stockée en tant que vecteur d'octets (`Vec<u8>`) mais garantit de toujours fournir une séquence valide encodée en UTF-8. `String` est alloué dans le tas, redimensionnable et non-nul.

`&str` est une slice (`&[u8]`) qui pointe toujours sur une séquence UTF-8 valide et peut être utilisée comme une vue sur une `String`. Tout comme `&[T]` est une vue sur une instance `Vec<T>`.

{{#playpen source/stringwrappersource0.rs}}

Les méthodes rattachées à `str`/`String` peuvent être trouvées dans les modules [std::str][str] et [std::string][string].

[str]: https://doc.rust-lang.org/std/str/
[string]: https://doc.rust-lang.org/std/string/
