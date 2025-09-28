use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};
use std::collections::HashSet;

#[proc_macro_derive(Getters)]
pub fn derive_getters(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let fields = match input.data {
        syn::Data::Struct(ref data) => &data.fields,
        _ => panic!("Getters can only be derived for structs"),
    };

    let mod_name = quote::format_ident!("Trait{}", name.to_string());
    
    let mut trait_impls = Vec::new();

    for field in fields {
        let ty = &field.ty;
        let ident = field.ident.as_ref().expect("Expected named fields");
        
        trait_impls.push(quote! {
            impl #mod_name::GetterTrait<#ty> for #name {
                fn get_field(&self) -> &#ty {
                    &self.#ident
                }
            }
        });
    }

    // Collect unique field types for Gettable trait implementations
    let mut unique_types = HashSet::new();
    let mut gettable_impls = Vec::new();

    for field in fields {
        let ty = &field.ty;
        if unique_types.insert(ty.clone()) {
            gettable_impls.push(quote! {
                impl #mod_name::Gettable for #ty {}
            });
        }
    }

    let expanded = quote! {
        mod #mod_name {
            use super::*;
            
            pub(crate) trait GetterTrait<T> {
                fn get_field(&self) -> &T;
            }
            
            pub(crate) trait Gettable {}
            
            #(#gettable_impls)*
        }

        impl #name {
            pub(crate) fn get<T>(&self) -> &T
            where 
                Self: #mod_name::GetterTrait<T>,
                T: #mod_name::Gettable,
            {
                use #mod_name::GetterTrait;
                self.get_field()
            }
        }

        
        #(#trait_impls)*
    };

    expanded.into()
}
