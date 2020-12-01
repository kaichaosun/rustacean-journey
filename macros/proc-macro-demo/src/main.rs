use proc_macro_example::make_answer;
use proc_macro_example::AnswerFn;
use proc_macro_example::HelperAttr;
use proc_macro_example::show_streams;

make_answer!();

#[derive(AnswerFn)]
struct MyStruct;

#[derive(HelperAttr)]
struct MyStructAttr {
    #[helper] field: ()
}

#[show_streams]
fn invoke1() {}

#[show_streams(bar)]
fn invoke2() {}

#[show_streams(multiple => tokens)]
fn invoke3() {}

#[show_streams { delimiters} ]
fn invoke4() {}

fn main() {
    println!("{}", answer_func());
    println!("{}", answer_derive());
}