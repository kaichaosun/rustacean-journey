use proc_macro_example::make_answer;
use proc_macro_example::AnswerFn;

make_answer!();

#[derive(AnswerFn)]
struct MyStruct;

fn main() {
    println!("{}", answer_func());
    println!("{}", answer_derive());
}