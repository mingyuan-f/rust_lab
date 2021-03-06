use std::fmt::Debug;

trait MyFrom<T> {
    fn my_from(from: T) -> Self;
}

trait MyInto<T> {
    fn my_into(self) -> T;
}

struct A {
    value: i32,
}

struct B {
    value: i32,
}

impl MyFrom<A> for B {
    fn my_from(from: A) -> Self {
        Self { value: from.value }
    }
}

// impl MyInto<B> for A {
//     fn my_into(self) -> B {
//         B::my_from(self)
//     }
// }

impl<F: MyFrom<I>, I> MyInto<F> for I {
    fn my_into(self) -> F {
        F::my_from(self)
    }
}

fn main() {
    let a = A { value: 10 };
    let b = B::my_from(a);
    println!("{}", b.value);

    let a = A { value: 10 };
    let b: B = a.my_into();
    println!("{}", b.value);

    fun();
}

#[derive(Debug)]
struct D {}

fn fun() {
    let a: Vec<Box<dyn Debug>> = vec![Box::new(D {})];
    println!("{}", std::mem::size_of::<&D>());
    println!("{}", std::mem::size_of::<&dyn Debug>());
}
