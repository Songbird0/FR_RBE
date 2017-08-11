# FFI

Rust fournit une Interface pour Fonction Externe ("Foreign Function Interface", dans la langue de Shakespear) pour les bibliothèques écrites en C. Les fonctions externes peuvent être déclarées dans un bloc `extern` annoté de l'attribut `#[link]` contenant le nom de la bibliothèque externe.

```rust,ignore
// Dans le fichier ffi.rs
use std::fmt;

// Ce bloc externe lie la bibliothèque libm.
#[link(name = "m")]
extern {
    // Ceci est une fonction externe 
    // qui calcule la racine carrée d'un nombre complexe à précision simple.
    fn csqrtf(z: Complex) -> Complex;
}

fn main() {
    // z = -1 + 0i
    let z = Complex { re: -1., im: 0. };

    // Appeler une fonction externe est une opération dite "à risque".
    let z_sqrt = unsafe {
        csqrtf(z)
    };

    println!("the square root of {:?} is {:?}", z, z_sqrt);
}

// Implémentation minimale d'un nombre complex à précision simple.
#[repr(C)]
#[derive(Clone, Copy)]
struct Complex {
    re: f32,
    im: f32,
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0. {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}
```

```bash
$ rustc ffi.rs && ./ffi
the square root of -1+0i is 0+1i
```
Puisque l'appel de fonctions externes est considéré comme "à risque", il est courant d'écrire des wrappers sécurisés.

```rust,ignore
// Dans le fichier safe.rs
use std::fmt;

#[link(name = "m")]
extern {
    fn ccosf(z: Complex) -> Complex;
}

// Wrapper sécurisé.
fn cos(z: Complex) -> Complex {
    unsafe { ccosf(z) }
}

fn main() {
    // z = 0 + 1i
    let z = Complex { re: 0., im: 1. };

    println!("cos({:?}) = {:?}", z, cos(z));
}

// Implémentation minimale d'un nombre complexe à précision simple.
#[repr(C)]
#[derive(Clone, Copy)]
struct Complex {
    re: f32,
    im: f32,
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0. {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}
```
