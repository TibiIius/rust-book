use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
  // Lot's of magic that makes Rust code to syntax tree
  let ast = syn::parse(input).unwrap();

  // actual implementation happens here
  impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
  // ident corresponds to the name of the struct in String representation
  let name = &ast.ident;

  let generated = quote! {
    impl HelloMacro for #name {
      fn hello_macro() {
        println!("Hello, Tim! My name is {}!", stringify!(#name));
      }
    }
  };
  generated.into()
}
