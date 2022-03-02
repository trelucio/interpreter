/*
Therese Relucio
HW 9: March 3, 2022

1. Take the interpreter we have so far (from GitHub), 
    and add a "subtract" primitive to it.
*/

enum Primitive {
    Add,
    Subtract,
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
        Primitive::Subtract => { eval_prim(&array[1]) - eval_prim(&array[2])}
        Primitive::Multiply => { eval_prim(&array[1]) * eval_prim(&array[2])}
        _ => 0
    }
}

fn main() {
    // from original interpreter
    let mut primitives = Vec::<Primitive>::new();
    primitives.push(Primitive::Multiply);
    primitives.push(Primitive::Number(3));
    primitives.push(Primitive::Number(4));
    let result = evaluate(primitives);
    println!("{}", result);

    // HW 8: New main() function that is a new expression
    // new vector because primitives vector is out of scope
    let mut primitives_two =  Vec::<Primitive>::new();
    primitives_two.push(Primitive::Add);
    primitives_two.push(Primitive::Number(20));
    primitives_two.push(Primitive::Number(result));
    let new_result = evaluate(primitives_two);
    println!("New Result: {}", new_result);
    
    // HW 9: Subtraction (25-20=5)
    let x = 25;
    let y = 20;
    let mut sub_primitives =  Vec::<Primitive>::new();
    sub_primitives.push(Primitive::Subtract);
    sub_primitives.push(Primitive::Number(x));
    sub_primitives.push(Primitive::Number(y));
    let difference = evaluate(sub_primitives);
    println!("Difference of {} and {} = {}", x, y, difference);
}
