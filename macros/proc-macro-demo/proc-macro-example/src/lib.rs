use proc_macro::TokenStream;

// Function-like procedural macros
#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer_func() -> u32 { 42 }".parse().unwrap()
}

// Derive macros
#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
    "fn answer_derive() -> u32 { 42 }".parse().unwrap()
}

// Derive macro with helper attributes
#[proc_macro_derive(HelperAttr, attributes(helper))]
pub fn derive_helper_attr(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}

// Attribute macros
#[proc_macro_attribute]
pub fn return_as_is(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}