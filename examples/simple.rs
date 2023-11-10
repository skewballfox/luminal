use luminal::{nn::linear::Linear, prelude::*};

fn main() {
    let mut cx = Graph::new();
    let model: Linear<4, 5> = InitModule::initialize(&mut cx);
    let a = cx.new_tensor::<R1<4>>("Input");
    let b = model.forward(a);

    a.set(vec![1., 2., 3., 4.]);
    b.keep().retrieve();
    cx.execute_debug();

    println!("B: {:?}", b.data());
}
