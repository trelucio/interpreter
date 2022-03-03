/*
Therese Relucio
HW 9: March 2, 2022

1. Please BAIL OUT on this homework if you have too much trouble with it!
2. Take the interpreter we have so far (from GitHub), and add a "subtract" primitive to it.
3. You aren’t going to be able to just use a different rust operator inside a copy of the "Add" arm, 
    so spend some time thinking about this one
4. If you legit can’t do this in 20 or 30 minutes, submit your , 
    and I’ll review those and see how to make it all work out during the next class!
*/
#[derive(Copy, Clone)]
enum Primitive {
    Add,
    Multiply,
    Subtract,
    Number(i32),
}

fn evaluate(array: Vec<Primitive>) -> i32 {
    let element = &array[0];
    let mut iter = array.iter();
    iter.next();

    match element {
        Primitive::Add => iter.fold(0, |total, next| 
            total + evaluate(vec![*next])),
        Primitive::Multiply => iter.fold(1, |total, next| 
            total * evaluate(vec![*next])),
        Primitive::Number(val) => *val,
        Primitive::Subtract => {
            iter.next();
            iter.fold(evaluate(vec![array[1]]), |total, next| total - evaluate(vec![*next]))},
    }
}

fn main() {
    let mut primitives = Vec::new();
    primitives.push(Primitive::Subtract);
    primitives.push(Primitive::Number(10));
    primitives.push(Primitive::Number(4));
    primitives.push(Primitive::Number(5));
    let result = evaluate(primitives);
    println!("result was {}", result);
}
