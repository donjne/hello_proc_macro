use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloProcMacro)]
pub fn hello_proc_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap;

    // Build the trait implementation
    impl_hello_proc_macro(&ast)
}

/*
Our hello_proc_macro_derive function first converts the input from a TokenStream to a data
structure that we can then interpret and perform operations on. This is where syn comes
into play. The parse function in syn takes a TokenStream and returns a DeriveInput
struct representing the parsed Rust code.

DeriveInput {
// --snip--
ident: Ident {
ident: "Mountain",
span: #0 bytes(95..103)
},
data: Struct(
DataStruct {
struct_token: Struct,
fields: Unit,
semi_token: Some(
Semi
)
}
)
}

Note - that the output for our derive macro is also
a TokenStream . The returned TokenStream is added to the code that our crate users write,
so when they compile their crate, they’ll get the extra functionality that we provide in the
modified TokenStream.
*/

fn impl_hello_proc_macro(ast: &syn::DeriveInput) {
    let name = &ast.ident;
    let gen = quote!{
        impl HelloProcMacro for name {
            fn hello_proc_macro() {
                println!("Hello, the name of your type is {}", stringify!(#name))
            }
        }
    };
    gen.into()
}
