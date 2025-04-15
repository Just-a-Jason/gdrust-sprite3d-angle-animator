use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(SidedAnimation)]
pub fn derive_to_sided_animation(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let variants = if let syn::Data::Enum(syn::DataEnum { variants, .. }) = &input.data {
        variants
    } else {
        panic!("SidedAnimation can only be derived for enums");
    };

    let match_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let lower_name = variant_name.to_string().to_lowercase();

        quote! {
            #name::#variant_name => match dir {
                Direction::Back => concat!(#lower_name, "_back"),
                Direction::Right => concat!(#lower_name, "_side"),
                Direction::Left => concat!(#lower_name, "_side"),
                Direction::Front => concat!(#lower_name, "_front"),
            },
        }
    });

    let expanded = quote! {
        impl SidedAnimation for #name {
            fn to_sided(&self, dir: Direction) -> &'static str {
                match self {
                    #(#match_arms)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
