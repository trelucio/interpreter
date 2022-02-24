/*
Therese Relucio
HW 8: February 24, 2022

1. Fork the repo for our interpreter to your own GitHub account, 
    and then clone that to your local system 
    - https://github.com/rmirabelli/interpreter
2. Write a new main() function that is a new expression, 
    and run the interpreter to watch the result.
3. Submit your main.rs function via Brightspace.
4. Provide the URL for your own fork of the interpreter
    - https://github.com/trelucio/interpreter
*/

enum Primitive {
    Add,
    Multiply,
    Number(i32)
}

fn eval_prim(primitive: &Primitive) -> i32 {
    match primitive {
        Primitive::Number(val) => *val,
        _ => 0
    }
}

fn evaluate(array: Vec<Primitive>) -> i32 {
    match array[0] {
        Primitive::Add => { eval_prim(&array[1]) + eval_prim(&array[2])}
        Primitive::Multiply => { eval_prim(&array[1]) * eval_prim(&array[2])}
        _ => 0
    }
}

fn main() {
    let mut primitives = Vec::<Primitive>::new();
    primitives.push(Primitive::Multiply);
    primitives.push(Primitive::Number(3));
    primitives.push(Primitive::Number(4));
    let result = evaluate(primitives);
    println!("{}", result);

    // New main() function that is a new expression
    // new vector because primitives vector is out of scope
    let mut primitives_two =  Vec::<Primitive>::new();
    primitives_two.push(Primitive::Add);
    primitives_two.push(Primitive::Number(20));
    primitives_two.push(Primitive::Number(result));
     // new_result = 20 + "result" from OG code
    let new_result = evaluate(primitives_two);
    println!("New Result: {}", new_result);
}
