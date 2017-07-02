// `F` doit être générique.
fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}
# fn main() {}
