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

    let mut outputs = vec![q! {
        #trait_block
    }];

    while let Some(syn::NestedMeta::Meta(syn::Meta::Path(p))) = args.next() {
        
        let arg_name = p.segments.last().unwrap().ident.to_string();
        let arg_name = arg_name.as_str();

        let generics = match arg_name {
            "Vec" | "Option" => q!( <Output, Input: #trait_name <Output>> ),
            "Str" => q!(  ),
            _ => return err_bad_type_args(trait_block)
        };
        let input_type = match arg_name {
            "Vec" | "Option" => q!( #p<Input> ),
            "Str" => q!( std::borrow::Cow<'static, str> ),
            _ => return err_bad_type_args(trait_block)
        };
        let return_type = match arg_name {
            "Vec" => q!( Vec<Output> ),
            "Option" => q!( Option<Output> ),
            "Str" => q!( std::borrow::Cow<'static, str> ),
            _ => return err_bad_type_args(trait_block)
        };

        let functions = trait_functions
            .iter()
            .map(|(attrs, sig, args)| {
                let syn::Signature { ident, generics, inputs, .. } = sig;
                
                let inner = match arg_name {
                    "Vec" => q!( self.into_iter().map(|i| i.#ident(#(#args)*) ).collect() ),
                    "Option" => q!( self.map(|i| i.#ident(#(#args)*) ).invert() ),
                    "Str" => q!( Ok(self) ),
                    _ => return Err(err_bad_type_args(&trait_block))
                };
                Ok(q! {
                    #(#attrs)*
                    fn #ident #generics (#inputs) -> PassResult<#return_type> {
                        #inner
                    }
                })
            })
            .collect::<Result<Vec<TokenStream2>, TokenStream>>();
        match functions {
            Err(e) => return e,
            Ok(functions) => outputs.push(q! {
                impl #generics #trait_name<#return_type> for #input_type {
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