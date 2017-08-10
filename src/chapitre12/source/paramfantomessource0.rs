use std::marker::PhantomData;

// Un tuple qui prend un type générique `A` et un paramètre fantôme `B`.
#[derive(PartialEq)] // Permet de tester l'égalité entre les instances du type.
struct PhantomTuple<A, B>(A,PhantomData<B>);

// Un tuple qui prend un type générique `A` et un paramètre fantôme `B`.
#[derive(PartialEq)] // Permet de tester l'égalité entre les instances du type.
struct PhantomStruct<A, B> { first: A, phantom: PhantomData<B> }

// Note: De la mémoire sera allouée pour le type générique `A`, mais pas pour `B`.
//       En revanche, `B` ne pourra pas être utilisé à l'exécution.

fn main() {
    // Ici, les types `f32` et `f64` sont des paramètres fantômes.
    // Types spécifiés pour PhantomTuple: `<char, f32>`.
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    // Types spécifiés pour PhantomTuple: `<char, f64>`.
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    // Types spécifiés pour PhantomStruct: `<char, f32>`.
    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    // Types spécifiés pour PhantomStruct: `<char, f64>`.
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // Erreur! Les deux ressources ne peuvent pas être comparées:
    // println!("_tuple1 == _tuple2 yields: {}",
    //          _tuple1 == _tuple2);

    // Erreur! Les deux ressources ne peuvent pas être comparées:
    // println!("_struct1 == _struct2 yields: {}",
    //          _struct1 == _struct2);
}
