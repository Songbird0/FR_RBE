pub trait Iterator {
    // Le type sur lequel on va itérer.
    type Item;

    // `find` prend en paramètre une référence mutable de l'instance courante
    // `&mut self`. Elle sera donc empruntée et modifiée, mais pas consommée.
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        None
    }
}
#fn main() {}
