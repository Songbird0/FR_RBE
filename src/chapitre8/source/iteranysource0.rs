pub trait Iterator {
    // Le type sur lequel on va itérer.
    type Item;

    // `any` prend en paramètre une référence mutable `&mut self` de
    // l'instance courante qui sera empruntée et modifiée, mais pas consommée
    // (possédée).
    fn any<F>(&mut self, f: F) -> bool
    where
        F: FnMut(Self::Item) -> bool,
    {
        true
    }
}
# fn main() {}
