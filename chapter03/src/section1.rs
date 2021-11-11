/*
Section3-1: 基本的な文法
*/

// use std::io::Write; // write, writelnマクロを使うため

struct Iter {
    current: usize,
    max: usize
}

impl Iterator for Iter {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.current += 1;
        if self.current - 1 < self.max {
            Some(self.current - 1)
        } else {
            None
        }
    }
}

// Eqを実装するためにPartialEqが必要
#[derive(Eq, PartialEq)]
struct A(i32);

// PartialOrdを実装するためにPartialEqが必要
#[derive(PartialEq, PartialOrd)]
struct B(f32);

// Copyを実装するためにCloneが必要
#[derive(Copy, Clone)]
struct C;

#[derive(Clone)]
struct D;

#[derive(Debug)]
struct E;

#[derive(Default)]
struct F;

pub fn main() {
    /* 文字列型 */
    // let s1: String = String::from("Hello, World!");
    // let s2: &str = &s1;
    // let s3: String = s2.to_string();

    /* タプル */
    // let mut t = (1, "2");
    // t.0 = 2;
    // t.1 = "3";

    /* 配列 */
    // let mut a: [i32; 3] = [0, 1, 2];
    // let b: [i32; 3] = [0; 3];
    // a[1] = b[1];
    // a[2] = b[2];
    // println!("{:?}", &a[1..3]);

    /* ユーザー定義型 */
    // struct Person {
    //     name: String,
    //     age: u32
    // }
    // let p = Person {
    //     name: String::from("John"),
    //     age: 8
    // };

    // enum Event {
    //     Quit,
    //     KeyDown(u8),
    //     MouseDown { x: i32, y: i32 }
    // }
    // let e1 = Event::Quit;
    // let e2 = Event::MouseDown { x: 10, y: 10 };

    /* 頻出する標準ライブラリの型 */
    /* Option
    pub enum Option<T> {
        None,
        Some(T)
    }
    */

    /* Result
    pub enum Result<T, E> {
        Ok(T),
        Err(E)
    }
    */
    // let result: Result<i32, String> = Ok(200);
    // match result {
    //     Ok(code) => println!("code: {}", code),
    //     Err(err) => println!("Err: {}", err)
    // }

    // if let Ok(code) = result {
    //     println!("code: {}", code);
    // }

    // println!("code: {}", result.unwrap_or(-1)); // => "code: 200"
    // let result: Result<i32, String> = Err("error".to_string());
    // println!("code: {}", result.unwrap_or(-1)); // => "code: -1"

    // fn func(code: i32) -> Result<i32, String> {
    //     println!("code: {}", code);
    //     Ok(100)
    // }
    // let next_result = result.and_then(func); // func()は実行される
    // let result: Result<i32, String> = Err("error".to_string());
    // let next_result = result.and_then(func); // func()は実行されない

    // fn error_handling(result: Result<i32, String>) -> Result<i32, String> {
    //     let code = result?; // エラーの場合はここでreturn result;
    //     println!("code: {}", code);
    //     Ok(100)
    // }

    /* Vec */
    // let v1 = vec![1, 2, 3, 4, 5]; // 1~5の数を入れて初期化
    // let v2 = vec![0; 5]; // 0を5つ埋めて初期化
    // println!("{}", v1[0]);
    // for element in &v1 {
    //     println!("{}", element);
    // }

    /* Box */
    // let byte_array = [b'h', b'e', b'l', b'l', b'o'];
    // print(Box::new(byte_array));
    // let byte_array = [b'w', b'o', b'r', b'l', b'd', b'!'];
    // print(Box::new(byte_array));
    // fn print(s: Box<[u8]>) {
    //     println!("{:?}", s);
    // }

    /* 制御構文 */
    /* if */
    // let number = 1;
    // if 0 < number {
    //     println!("0 < number");
    // } else if number < 0 {
    //     println!("number < 0");
    // } else {
    //     println!("0 == number");
    // }
    // let result = if 0 <= number {
    //     number
    // } else {
    //     -number
    // };

    /* ループ */
    // let mut count = 0;
    // let result = loop {
    //     println!("count: {}", count);
    //     count += 1;
    //     if count == 10 {
    //         break count;
    //     }
    // };
    // while count < 10 {
    //     
    //     count += 1;
    // }
    // for count in 0..10 {
    //     println!("count: {}", count);
    // }
    // let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    // for element in &array {
    //     println!("element: {}", element);
    // }
    // 'main: loop {
    //     println!("main loop start");
    //     'sub: loop {
    //         println!("sub loop start");
    //         break 'main;
    //         println!("sub loop end"); // 表示されない
    //     }
    //     println!("main loop end"); // 表示されない
    // }

    /* match */
    // let i = 1;
    // match i {
    //     1 => println!("1"),
    //     2 => println!("2"),
    //     3 => println!("3"),
    //     _ => println!("misc"), // アンダースコアは、あらゆる値にマッチする
    // }
    // enum Color {
    //     Red,
    //     Blue,
    //     Green
    // }
    // let c = Color::Red;
    // match c {
    //     Color::Red => println!("Red"),
    //     Color::Blue => println!("Blur"),
    //     Color::Green => println!("Green")
    // }
    // let result: Result<i32, String> = Ok(100);
    // let result_number = match result {
    //     Ok(number) => number,
    //     Err(message) => {
    //         println!("Error: {}", message);
    //         -1
    //     }
    // };

    /* Range */
    // for number in 1..5 {
    //     println!("{}", number);
    // }

    /* Iterator */
    let it = Iter {
        current: 0,
        max: 10
    };
    for num in it {
        println!("{}", num);
    };

    /* 関数 */
    /* fn */
    // fn add (a: i32, b: i32) -> i32 {
    //     a + b
    // }
    // fn abs(number: i32) -> i32 {
    //     if number < 0 {
    //         return -number;
    //     }
    //     number
    // }

    /* impl */
    // struct Person {
    //     name: String,
    //     age: u32
    // }
    // impl Person {
    //     fn say_name(&self) {
    //         println!("I am {}.", self.name);
    //     }
    //     fn say_age(&self) {
    //         println!("I am {} year(s) old.", self.age);
    //     }
    // }
    // let p = Person {
    //     name: String::from("Taro"),
    //     age: 20
    // };
    // p.say_name();
    // p.say_age();
    // impl Person {
    //     fn say_name(&self) -> &Self {
    //         println!("I am {}.", self.name);
    //         self
    //     }
    //     fn say_age(&self) -> &Self {
    //         println!("I am {} year(s) old.", self.age);
    //         self
    //     }
    // }
    // let p = Person {
    //     name: String::from("Taro"),
    //     age: 20
    // };
    // p.say_name().say_age();
    // impl Person {
    //     fn new(name: &str, age: u32) -> Person {
    //         Person {
    //             name: String::from(name),
    //             age: age
    //         }
    //     }
    // }
    // let p = Person::new("Taro", 20);
    // p.say_name().say_age();

    /* マクロ */
    // let s = concat!("A", "b2", 3); // s = String:from("Ab23")と同じ。
    // let s = format!("{}-{:?}", s, ("D", 5)); // s = String::from("Ab23-(\"D\", 5)")と同じ。
    // let s = format!("{}{}", "abc", "def"); // s = String::from("abcdef")と同じ。
    // print!("hello");
    // println!("hello {}", "world");
    // eprint!("hello {}", "error");
    // eprintln!("hello");
    // let mut w = Vec::new(); // バイト列書き込み用のVecを宣言
    // write!(&mut w, "{}", "ABC");
    // writeln!(&mut w, " is 123");
    // dbg!(w); // フォーマット文字列を受け取らない
    // panic!("it will panic");
    // println!("defined in file: {}", file!());
    // println!("defined on line: {}", line!());
    // println!("is test: {}", cfg!(unix));
    // println!("CARGO_HOME: {}", env!("CARGO_HOME"));
    // assert!(true);
    // assert_eq!(1, 1);
    // assert_ne!(1, 0);
    // debug_assert!(false);
    // debug_assert_eq!(1, 1);
    // debug_assert_ne!(1, 0);
    // enum Emotion {
    //     Anger,
    //     Happy
    // }
    // trait Emotional {
    //     fn get_happy(&mut self) -> String;
    //     fn get_anger(&mut self) -> String;
    //     fn tell_state(&self) -> String;
    // }
    // struct HappyPerson {
    //     name: String,
    //     state: Emotion
    // }
    // impl Emotional for HappyPerson {
    //     fn get_anger(&mut self) -> String {
    //         // この関数は呼ばれないので実装しないが、String型を返さなくても型検査を通過させる
    //         unimplemented!()
    //     }
    //     fn get_happy(&mut self) -> String {
    //         format!("{} is always happy.", self.name)
    //     }
    //     fn tell_state(&self) -> String {
    //         // この関数は後で実装したいが、一旦String型を返さなくても型検査を通過させる
    //         todo!()
    //     }        
    // }
    // let mut p = HappyPerson {
    //     name: "Takashi".to_string(),
    //     state: Emotion::Happy,
    // };
    // println!("{}", p.get_happy());
    // fn f(x: usize) -> &'static str {
    //     match x {
    //         n if n * n % 3 == 0 => "3n",
    //         n if n * n % 3 == 1 => "3n+1 or 3n+2",
    //         _ => unreachable!() // コンパイラは上記条件で網羅していることを判定できない。
    //     }
    // }

    // A は一致比較可能
    println!("{:?}", A(0) == A(1));

    // B は大小比較可能
    println!("{:?}", B(1.0) > B(0.0));

    // Cはムーブではなくコピーされる
    let c0 = C;
    let _c1 = c0;
    let _c2 = c0; // Cがムーブならc0はc1へムーブしているのでここでコンパイルエラー

    // Dはclone可能
    let d0 = D;
    let _d1 = d0.clone();

    // Eはデバッグプリント{:?}可能
    println!("{:?}", E);

    // Fはdefault可能
    let _f = F::default();
}
