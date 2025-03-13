extern crate proc_macro;
use proc_macro::TokenStream;

use syn::parse_macro_input;

#[proc_macro_derive(EnumCount)]
pub fn derive_enum_count(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);

    if let syn::Data::Enum(data_enum) = &ast.data {
        let enum_name = &ast.ident;
        let count = data_enum.variants.len();

        quote::quote! {
            impl #enum_name {
                pub const COUNT: usize = #count;
            }
        }
    } else {
        panic!("EnumCount can only be derived for enums");
    }
    .into()
}
