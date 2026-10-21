use proc_macro::TokenStream;
use quote::quote;
/*
Реализация derive макроса в src/lib.rs файле.
*/
#[proc_macro_derive(Hello)]
pub fn hello(input: TokenStream) -> TokenStream {
 // ast - Abstract Syntax Tree
 let ast: syn::DeriveInput = syn::parse(input).unwrap();
 let name = &ast.ident; // Получить имя структуры
 quote! { impl #name { fn hello() { println!("hello from {}", stringify!(#name)); } } }.into()
}

