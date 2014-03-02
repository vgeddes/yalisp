use std::str;

mod syntax;

enum Node {
    Var(~str),
    Const(~str),
    Fun(~str, ~[~Node], ~Node),
    App(~[~Node]),
}


fn doo() -> uint
{
    1
}

fn main()
{
    let hi = ~"hi!";
    let mut count: uint = 0u;

    if str::eq(&hi, &~"foooo") {
        println!("fp");
    }
}
