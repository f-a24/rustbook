use tomlstruct::tomlstruct;

// tomlstruct! {
//     struct A {
//         x: i32
//     }
// }

// ↓　シンタックスエラーになってまう…(´;ω;｀)
tomlstruct! {
    [Hello]
    name = "hello"
    version = 1.0
}

fn main() {
    // let x = A { x: 0 };
    // dbg!(x.x);

    let _ = Hello {
        name: String::from("hello"),
        version: 1.0,
    };
}
