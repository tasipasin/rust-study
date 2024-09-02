use proc_macro::TokenStream;
use quote::quote;

// This will be called everytime someone specified #[derive(HelloMacro)] on a type
// Because the annotation proc_macro_derive and the custom name inside the parenthesis
// The name, by convention, should match the trait name
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // This code inside will be the same for almost every procedural macro

    // Construct a representation of Rust code as a syntax tree
    // that can be manipulated
    // syn takes TokenStream and returns DeriveInput as Rust's parsed code
    // Macro's errors should cause panic!
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    // This code will change depending on the procedural macro's purpose.

    // Get an Ident struct instance of the annotated type
    let name = &ast.ident;
    // quote! helps define tha code wanted to return
    // It will replace #name for the variable's value
    // It generates a default implementation for HelloMacro to any
    // struct that derives it.
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                // The stringify! macro will cause to convert the expression in
                // literal string, as written. Here it will first change #name
                // for the struct's name and this name will be stringified.
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
