#![feature(proc_macro_diagnostic)]

use syn::spanned::Spanned;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro::TokenStream;
use quote::quote as q;


// TODO 
// this whole thing is terrible, only works in specific cases
// needs to be rewritten soon
#[proc_macro_attribute]
pub fn auto_implement(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);

    let trait_block: syn::ItemTrait = syn::parse2(input).expect("expected trait def");
    let trait_name = &trait_block.ident;
    let trait_generics = &trait_block.generics;
    
    let trait_functions = &trait_block.items
        .iter()
        .filter_map(|item| if let syn::TraitItem::Method(method) = item {
            Some((method, get_args_from_trait_method(&method)))
        } else {
            None
        })
        .collect::<Vec<(&syn::TraitItemMethod, Vec<&syn::Ident>)>>();

    let args: syn::AttributeArgs = syn::parse_macro_input!(args as syn::AttributeArgs);
    let mut args = args.iter();

    let mut outputs = vec![q! {
        #trait_block
    }];

    while let Some(syn::NestedMeta::Meta(syn::Meta::Path(p))) = args.next() {
        
        let arg_name = path_string(&p);

        let return_type_name = if trait_generics.params.len() >= 1 {
            q!( Output )
        } else {
            q!( Input )
        };

        let input_type = match &*arg_name {
            "Vec" => q!( Vec<Input> ),
            "Option" => q!( Option<Input> ),
            "Box" => q!( Box<Input> ),
            "Str" => q!( std::borrow::Cow<'static, str> ),
            _ => return err_bad_type_args(trait_block)
        };
        let return_type = match &*arg_name {
            "Vec" => q!( Vec<#return_type_name> ),
            "Option" => q!( Option<#return_type_name> ),
            "Box" => q!( Box<#return_type_name> ),
            "Str" => q!( std::borrow::Cow<'static, str> ),
            _ => return err_bad_type_args(trait_block)
        };

        let get_trait_generics = |new_type: &TokenStream2| -> Option<TokenStream2> {
            let new_trait_generics: Vec<&syn::GenericParam> = trait_generics.params.iter().collect();
            new_trait_generics.split_first().map(|(_, rest)| {
                q!( <#new_type #(,#rest)*> )
            })
        };

        let new_trait_generics = get_trait_generics(&return_type);
        
        let generics = match &*arg_name {
            "Vec" | "Option" | "Box" => {
                let inner_type_trait_bounds = get_trait_generics(&return_type_name);
                
                if trait_generics.params.len() >= 1 {
                    q!( <#return_type_name, Input: #trait_name #inner_type_trait_bounds> )
                } else {
                    q!( <#return_type_name: #trait_name #inner_type_trait_bounds> )
                }
            },
            "Str" => q!(  ),
            _ => return err_bad_type_args(trait_block)
        };

        let functions = trait_functions
            .iter()
            .map(|(method, args)| {
                let attrs = &method.attrs;
                let syn::Signature { ident, generics, inputs, output, .. } = &method.sig;
                let output = match output {
                    syn::ReturnType::Type(_, t) => match &**t {
                        syn::Type::Path(path) => {
                            replace_first_generic_with_type(&path.path, &return_type)
                        },
                        _ => panic!("expected path, found {t:#?}")
                    },
                    _ => todo!()
                };
                
                let inner = match &*arg_name {
                    "Vec" => q!( self.into_iter().map(|i| i.#ident(#(#args)*) ).collect() ),
                    "Option" => q!( self.map(|i| i.#ident(#(#args)*) ).invert() ),
                    "Box" => q!{ 
                        let output = (*self).#ident(#(#args)*)?;
                        Ok(Box::new(output)) 
                    },
                    "Str" => q!( Ok(self) ),
                    _ => return Err(err_bad_type_args(&trait_block))
                };
                Ok(q! {
                    #(#attrs)*
                    fn #ident #generics (#inputs) -> #output {
                        #inner
                    }
                })
            })
            .collect::<Result<Vec<TokenStream2>, TokenStream>>();
        // let x = q! {
        //     impl #generics #trait_name #new_trait_generics for #input_type
        // }.to_string();
        // panic!("{x:#?}");
        match functions {
            Err(e) => return e,
            Ok(functions) => outputs.push(q! {
                impl #generics #trait_name #new_trait_generics for #input_type {
                    #(#functions)*
                }
            })
        }
    }
    let output = q! {
        #(#outputs)*
    };
    output.into()
}

fn err_bad_type_args<T: Spanned>(item: T) -> TokenStream {
    item.span().unstable().error("expected Vec, Option, or Str").emit();
    q!(item).into()
}

fn get_args_from_trait_method(method: &syn::TraitItemMethod) -> Vec<&syn::Ident> {
    method.sig.inputs.iter().filter_map(|param| {
        if let syn::FnArg::Typed(p) = param {
            if let syn::Pat::Ident(i) = &*p.pat {
                return Some(&i.ident)
            }
        }
        None
    }).collect()
}

fn path_string(ident: &syn::Path) -> String {
    ident.segments.last().unwrap().ident.to_string()
}

fn replace_first_generic_with_type(path: &syn::Path, new_type: &TokenStream2) -> TokenStream2 {
    let segments = &path.segments;
    let segments = segments.iter()
        .collect::<Vec<&syn::PathSegment>>();
    let (last_segment, segments): (&&syn::PathSegment, &[&syn::PathSegment]) = segments
        .split_last()
        .unwrap();
    let last_segment_ident = &last_segment.ident;
    let last_segment_arguments: Vec<&syn::GenericArgument> = match &last_segment.arguments {
        syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments { args, ..}) => args.iter().collect(),
        _ => panic!("expected angle bracket generics"),
    };
    let new_last_args = last_segment_arguments.split_first().unwrap().1;
    quote::quote! {
        #(#segments::)*#last_segment_ident<#new_type #(,#new_last_args)*>
    }
}