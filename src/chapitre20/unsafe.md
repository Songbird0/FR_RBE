# Opérations à risque

Pour reprendre ce que la documentation officielle dit: "Il faudrait essayer de minimiser la quantité de code à risque dans la base du code."  Avec ceci en tête, commençons ! 
Les blocs unsafe en Rust sont utilisés pour contourner les protections mises en place par le compilateur; plus précisément, il y a quatre principaux cas d'utilisation que nous pouvons retrouver dans les blocs unsafe:

1. Le déréférencement des pointeurs bruts;
2. L'appel d'une fonction à partir de la FFI (cette partie est couverte à d'autres endroits dans le livre);
3. La modification des types par le biais de `std::mem::transmute`;
4. Inliner de l'assembleur.

## Les pointeurs bruts

 Les pointeurs bruts `*` et références `&T` fonctionnent de la même manière mais les références sont toujours sécurisées parce qu'elles garantissent de pointer sur une ressource valide grâce au vérificateur d'emprunts. Le déréférencement d'un pointeur brut ne peut se faire que par le biais d'un bloc unsafe.

 ```rust,ignore
 // Dans le fichier pointer.rs
fn main() {
    let raw_p: *const u32 = &10;

    unsafe {
        assert!(*raw_p == 10);
    }
}
```

## Transmuter

Permet une simple conversion d'un type à l'autre, cependant les deux types doivent disposer de la même taille en mémoire et le même alignement:

```rust,ignore
// Dans le fichier transmute.rs
fn main() {
    let u: &[u8] = &[49, 50, 51];

    unsafe {
        assert!(u == std::mem::transmute::<&str, &[u8]>("123"));
    }
}
```
