use proc_macro_example::make_answer;

make_answer!();

fn main() {
    println!("{}", answer());
}