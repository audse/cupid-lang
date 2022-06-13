#![feature(proc_macro_diagnostic, let_else)]

use proc_macro2::TokenStream as TokenStream2;
use proc_macro::TokenStream;

const VARIANTS: [&str; 9] =  [
    "PreAnalysis",
    "PackageResolved",
    "TypeNameResolved",
    "ScopeAnalyzed",
    "NamesResolved",
    "TypesInferred",
    "TypesChecked",
    "FlowChecked",
    "Linted"
];

fn to_snake_case<S: Into<String>>(s: S) -> String {
    let upper_chars: Vec<char> = ('A'..='Z').collect();
    s.into()
        .split_inclusive(&*upper_chars)
        .map(|i| i.to_lowercase())
        .collect::<Vec<String>>()
        .join("_")
}

fn to_pascal_case<S: Into<String>>(s: S) -> String {
    let lower_chars: Vec<char> = ('a'..='z').collect();
    s.into()
        .split('_')
        .map(|i| {
            let first: char = i.chars().next().unwrap_or_default();
            i.replacen(&*lower_chars, &first.to_string().to_uppercase(), 1) 
        })
        .collect::<Vec<String>>()
        .join("")
}

#[proc_macro_attribute]
pub fn semantic_states(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);

    let item: syn::Item = syn::parse2(input).expect("failed to parse input");

    match item {
        syn::Item::Struct(syn::ItemStruct { attrs, vis, ident, generics, fields, .. }) => {
            

            // Constructs placeholders for each NodeState generic
            // e.g. [("PreAnalysis", None) ...]
            let mut pass_fields = VARIANTS
                .iter()
                .map(|pass| (*pass, Option::<syn::Field>::None))
                .collect::<Vec<(&str, Option<syn::Field>)>>();

            // Replaces the placeholder in `pass_fields` of any field 
            // that is specified in the input struct
            if let syn::Fields::Named(fields) = fields {
                for field in fields.named.into_iter() {
                    let field_name = field.ident.as_ref().expect("expected field ident!").to_string();
                    let pass = pass_fields
                        .iter()
                        .position(|(pass_name, _)| *pass_name == to_pascal_case(&field_name))
                        .expect("expected a valid semantic pass name!");
                    pass_fields[pass].1 = Some(field);
                }
                let mut last_field = 0;
                let new_fields = VARIANTS
                    .iter()
                    .enumerate()
                    .map(|(i, pass_name)| {
                        let true_field = pass_fields
                            .iter()
                            .find(|(name, _)| pass_name == name)
                            .expect("expected a valid semantic pass name!");
                        if let Some(val) = &true_field.1 {
                            last_field = i;
                            Some(val.to_owned())
                        } else {
                            Some(pass_fields[last_field].1
                                .clone()
                                .expect("expected the previous field to have a value"))
                        }.expect("expected type!").ty
                    })
                    .collect::<Vec<syn::Type>>();

                let pass_generics = quote::quote!(#(#new_fields),*);
                let new_type = quote::quote!(NodeState<#pass_generics>);
                // Constructs a newtype wrapper for `NodeState<A, B, ..>`
                let output = quote::quote! {
                    #(#attrs)*
                    #vis struct #ident #generics(#new_type);

                   impl #generics GetNode <#pass_generics> for #ident #generics {
                        fn node(self) -> #new_type { self.0 }
                    }

                };
                output.into()
            } else {
                todo!()
            }
        },
        _ => todo!()
    }
}
