extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote};
use syn::{parse_macro_input, DeriveInput, Data};

#[proc_macro_derive(Cranenum)]
pub fn cranenum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    let variants = match &input.data {
        Data::Enum(data) => &data.variants,
        _ => {
            panic!("#[derive(Cranenum)] can only be applied to enums");
        }
    };

    let outputs = variants.iter().map(|variant| {
        let field = variant.fields.iter().nth(0).unwrap();
        let from = &field.ty;
        let variant_ident = &variant.ident;

        let output = quote! {
            impl std::convert::From<#from> for #ident {
                fn from(item: #from) -> Self {
                    #ident::#variant_ident(item)
                }
            }
        };
        TokenStream::from(output)
    }).collect::<TokenStream>();

    outputs.into()
}
