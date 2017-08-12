struct S; // On déclare le type concret `S`.
struct GenericVal<T>(T); // On déclare le type générique `GenericVal`.

// Implémentation de GenericVal où nous précisons que cette méthode doit être
// implémentée uniquement pour le type `f32`.
impl GenericVal<f32> {
    fn say_hello_f32(&self) -> (){
        println!("I'm a float! :D");
    }
} // On spécifie `f32`
impl GenericVal<S> {
    fn say_hello_s(&self) -> (){
        println!("I'm a S object! :D");
    }
} // On spécifie le type `S` pour les mêmes raisons qu'au-dessus.
// `<T>` doit précéder le type pour le rendre générique.
impl <T> GenericVal<T> {
    fn say_hello(&self) -> (){
        println!("I'm a generic object! :D");
    }
}

struct Val {
    val: f64
}

struct GenVal<T>{
    gen_val: T
}

// Implémentation de Val.
impl Val {
    fn value(&self) -> &f64 { &self.val }
}

// Implémentation de GenVal pour le type générique `T`.
impl <T> GenVal<T> {
    fn value(&self) -> &T { &self.gen_val }
}

fn main() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };

    println!("{}, {}", x.value(), y.value());
    GenericVal(1.0).say_hello_f32();
    GenericVal(S).say_hello_s();
    GenericVal("prout").say_hello();
}
