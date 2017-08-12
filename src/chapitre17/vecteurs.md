# Les vecteurs

Les vecteurs sont des tableaux redimensionnables. Tout comme les slices, leur taille n'est pas connue à la compilation mais ils peuvent être agrandis ou tronqués au cours de l'exécution. Un vecteur est représenté par trois (3) mots: un pointeur sur la ressource, sa taille et sa capacité. La capacité indique la quantité de mémoire réservée au vecteur. La taille peut augmenter à volonté, tant qu'elle est inférieure à la capacité. Lorsqu'il est nécessaire de franchir cette limite, le vecteur est réalloué avec une capacité plus importante.

{{#playpen source/vecteurssource0.rs}}

Les méthodes rattachées à la structure `Vec` peuvent être trouvées au sein du module [`std::vec`][vec].

[vec]: https://doc.rust-lang.org/std/vec/
