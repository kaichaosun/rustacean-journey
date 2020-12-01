use proc_macro::TokenStream;

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer_func() -> u32 { 42 }".parse().unwrap()
}

#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
    "fn answer_derive() -> u32 { 42 }".parse().unwrap()
}