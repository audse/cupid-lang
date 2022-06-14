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

// fn to_snake_case<S: Into<String>>(s: S) -> String {
//     let upper_chars: Vec<char> = ('A'..='Z').collect();
//     s.into()
//         .split_inclusive(&*upper_chars)
//         .map(|i| i.to_lowercase())
//         .collect::<Vec<String>>()
//         .join("_")
// }

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

                let [
                    pre_analysis,
                    package_resolved,
                    type_names_resolved,
                    scopes_analyzed,
                    names_resolved,
                    types_inferred,
                    types_checked,
                    flow_checked,
                    linted
                ] = &new_fields[0..] else { todo!() };

                let pass_generics = quote::quote!(#(#new_fields),*);
                let new_type = quote::quote!(NodeState<#pass_generics>);
                // Constructs a newtype wrapper for `NodeState<A, B, ..>`
                let output = quote::quote! {
                    #(#attrs)*
                    #vis struct #ident #generics(#new_type);

                    impl #generics #ident #generics {
                        pub fn get_pre_analysis(self) -> Result<#pre_analysis, ErrCode> {
                            self.0.get_pre_analysis()
                        }
                        pub fn get_package_resolved(self) -> Result<#package_resolved, ErrCode> {
                            self.0.get_package_resolved()
                        }
                        pub fn get_type_names_resolved(self) -> Result<#type_names_resolved, ErrCode> {
                            self.0.get_type_names_resolved()
                        }
                        pub fn get_scopes_analyzed(self) -> Result<#scopes_analyzed, ErrCode> {
                            self.0.get_scopes_analyzed()
                        }
                        pub fn get_names_resolved(self) -> Result<#names_resolved, ErrCode> {
                            self.0.get_names_resolved()
                        }
                        pub fn get_types_inferred(self) -> Result<#types_inferred, ErrCode> {
                            self.0.get_types_inferred()
                        }
                        pub fn get_types_checked(self) -> Result<#types_checked, ErrCode> {
                            self.0.get_types_checked()
                        }
                        pub fn get_flow_checked(self) -> Result<#flow_checked, ErrCode> {
                            self.0.get_flow_checked()
                        }
                        pub fn get_linted(self) -> Result<#linted, ErrCode> {
                            self.0.get_linted()
                        }
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

// TODO 
// this whole thing is terrible, only works in specific cases
// needs to be rewritten soon
#[proc_macro_attribute]
pub fn auto_implement(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);

    let trait_block: syn::ItemTrait = syn::parse2(input).expect("expected trait def");
    let trait_name = &trait_block.ident;
    let trait_generics = &trait_block.generics;
    let trait_generic_params = &trait_block.generics.params.iter().collect::<Vec<&syn::GenericParam>>();
    let trait_functions = &trait_block.items
        .iter()
        .filter_map(|item| if let syn::TraitItem::Method(syn::TraitItemMethod { attrs, sig, ..}) = item {
            let args = sig.inputs
                .iter()
                .filter_map(|param| match param {
                    syn::FnArg::Typed(p) => {
                        match &*p.pat {
                            syn::Pat::Ident(i) => Some(&i.ident),
                            _ => None
                        }
                    },
                    _ => None
                })
                .collect::<Vec<&syn::Ident>>();
            Some((attrs, sig, args))
        } else {
            None
        })
        .collect::<Vec<(&Vec<syn::Attribute>, &syn::Signature, Vec<&syn::Ident>)>>();

    let args: syn::AttributeArgs = syn::parse_macro_input!(args as syn::AttributeArgs);
    let mut args = args.iter();

    let mut outputs = vec![quote::quote! {
        #trait_block
    }];

    while let Some(syn::NestedMeta::Meta(syn::Meta::Path(p))) = args.next() {
        
        let arg_name = &p.segments.last().unwrap().ident.to_string();

        let functions = trait_functions
            .iter()
            .map(|(attrs, sig, args)| {
                let syn::Signature { ident, generics, inputs, .. } = sig;
                
                let inner = match arg_name.as_str() {
                    "Vec" => quote::quote! { self.into_iter().map(|i| i.#ident(#(#args)*) ).collect() },
                    "Option" => quote::quote! { self.map(|i| i.#ident(#(#args)*) ).invert() },
                    _ => panic!("Expected either Vec or Option type")
                };
                quote::quote! {
                    #(#attrs)*
                    fn #ident #generics (#inputs) -> PassResult<#p<#(#trait_generic_params),*>> {
                        #inner
                    }
                }

            })
            .collect::<Vec<TokenStream2>>();

        outputs.push(quote::quote! {
            impl<#(#trait_generic_params),* , AutoImpl: #trait_name #trait_generics> #trait_name <#p<#(#trait_generic_params),*>> for #p<AutoImpl> {
                #(#functions)*
            }
        });
    }
    let output = quote::quote! {
        #(#outputs)*
    };
    output.into()
}