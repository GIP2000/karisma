extern crate proc_macro;
extern crate quote;
extern crate syn;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

fn get_model_name(attrs: &Vec<syn::Attribute>) -> syn::Ident {
    match attrs[0].parse_meta().expect("model attribute required") {
        syn::Meta::List(lst) => {
            if lst.nested.len() != 1 {
                panic!("need one argument exactly");
            }
            match &lst.nested[0] {
                syn::NestedMeta::Meta(m) => match m {
                    syn::Meta::Word(w) => w.clone(),
                    _ => panic!("parameter must be word"),
                },
                _ => panic!("must be lit type"),
            }
        }
        _ => panic!("Must be List type attribute"),
    }
}

fn are_fields_valid(
    model_name: &String,
    fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
) -> bool {
    true
}

#[proc_macro_derive(DBTraits, attributes(model))]
pub fn db_traits_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = input.generics;

    // maybe make this default to name?
    if input.attrs.len() != 1 {
        panic!("model Attribute required")
    }

    let model_name = get_model_name(&input.attrs);

    let fields = match input.data {
        syn::Data::Struct(ds) => match ds.fields {
            syn::Fields::Named(nfs) => nfs.named,
            _ => panic!("Invalid all fields must be named"),
        },
        _ => panic!("Invalid must be used with struct"),
    };

    if !are_fields_valid(&model_name.to_string(), &fields) {
        panic!("Object fields do not overlap model field")
    }

    let mut select_string = fields
        .iter()
        .filter(|f| f.ident.is_some())
        .fold("".to_string(), |acc, f| {
            format!("{}{},", acc, f.ident.to_owned().unwrap().to_string())
        });
    select_string.pop();

    let struct_fields: Vec<_> = fields
        .iter()
        .filter(|f| f.ident.is_some())
        .enumerate()
        .map(|(i, f)| {
            let name = f.ident.to_owned().unwrap();
            let index = syn::Index::from(i);
            quote! {
                #name : args[#index].db_parse(),
            }
        })
        .collect();

    let where_type_ident = syn::Ident::new(
        &format!("{}EnumTypes", model_name.to_string()),
        model_name.span(),
    );

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let expanded = quote! {
        impl #impl_generics crate::DbMemberTrait for #name #ty_generics #where_clause {
            const SELECT: &'static str = #select_string;
            const NAME: &'static str = "#model_name";
            type WhereFilterType = crate::model::#where_type_ident;
            fn build_from_query(args: Vec<&str>) -> Self {
                Self {
                     #(#struct_fields)*
                }
            }
        }
        impl #impl_generics crate::DbObject<#name> for crate::model::#model_name #ty_generics #where_clause {
            type WhereFilterType = crate::model::#where_type_ident;
        }

        impl #impl_generics crate::DbParseable<#name> for crate::RawDbOutput #ty_generics #where_clause {
            fn db_parse(&self) -> #name {
                #name::build_from_query(self.split(',').collect())
            }
        }

        impl #impl_generics crate::DbParseable<Vec<#name>> for crate::RawDbOutput #ty_generics #where_clause {
            fn db_parse(&self) -> Vec<#name> {
                self.split('\n').map(|r| r.db_parse()).collect()
            }
        }
    };

    return TokenStream::from(expanded);
}
