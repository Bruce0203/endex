use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemEnum};

#[proc_macro_derive(VariantIndex)]
pub fn variant_index(input: TokenStream) -> TokenStream {
    let item_enum = parse_macro_input!(input as ItemEnum);
    let enum_name = &item_enum.ident;
    let enum_generic_params = &item_enum.generics.params.iter().collect::<Vec<_>>();
    let enum_generic_where_clause = &item_enum.generics.where_clause;
    let enum_generic_params_without_bounds = &item_enum
        .generics
        .params
        .iter()
        .map(|param| match param {
            syn::GenericParam::Lifetime(lifetime_param) => &lifetime_param.lifetime.ident,
            syn::GenericParam::Type(type_param) => &type_param.ident,
            syn::GenericParam::Const(const_param) => &const_param.ident,
        })
        .collect::<Vec<_>>();
    let mut last_index: proc_macro2::TokenStream = quote::quote! {0};
    let match_branches = item_enum
        .variants
        .iter()
        .map(|variant| {
            let index = if let Some(discriminant) = &variant.discriminant {
                let index = &discriminant.1;
                last_index = quote::quote! { #index };
                last_index.clone()
            } else {
                last_index = quote::quote! {(#last_index) + 1};
                last_index.clone()
            };

            let variant_name = &variant.ident;
            let fields = match variant.fields {
                syn::Fields::Named(_) => quote::quote! { {..} },
                syn::Fields::Unnamed(_) => quote::quote! { (..) },
                syn::Fields::Unit => quote::quote! {},
            };
            quote::quote! {
                #enum_name::#variant_name #fields => #index
            }
        })
        .collect::<Vec<_>>();
    quote::quote! {
    impl<#(#enum_generic_params),*> endex::VariantIndex
        for #enum_name<#(#enum_generic_params_without_bounds),*> #enum_generic_where_clause {

            fn variant_index(&self) -> usize {
                match self {
                    #(#match_branches),*
                }
            }
        }
    }
    .into()
}
