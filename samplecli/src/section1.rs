use clap::Parser;
// use clap::{App, Arg};
// use std::env;

/*
Section4-1: プロジェクトの準備
*/

#[derive(Parser, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "Your name",
    about = "Super awesome sample RPN calcurator"
)]
struct Opts {
    // Sets the level of verbosity
    #[clap(short, long)]
    verbose: bool,

    // Formulas written in RPN
    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

pub fn main() {
    /*
     * コマンドライン引数の処理 
     * クレートを使用しない場合
     */
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    /*
     * サードパーティー製クレートclapを使用する場合
     * Builderパターンでの記述方式
     */
    // let matches = App::new("My RPN program")
    //     .version("1.0.0")
    //     .author("Your name")
    //     .about("Super awesome sample RPN calcurator")
    //     .arg(
    //         // Arg::with_name(n) <- Deprecated, see [Arg::new]
    //         Arg::new("verbose")
    //             .about("Sets the level of verbosity")
    //             .short('v')
    //             .long("verbose")
    //             .required(false),
    //     )
    //     .get_matches();

    // match matches.value_of("formula_file") {
    //     Some(file) => println!("File specified: {}", file),
    //     None => println!("No file specified."),
    // }
    // let verbose = matches.is_present("verbose");
    // println!("Is verbosity specified?: {}", verbose);

    /* deriveマクロを使った記述方式 */
    let opts = Opts::parse();
    match opts.formula_file {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified."),
    }
    println!("Is verbosity specified?: {}", opts.verbose);
}
