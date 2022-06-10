#![feature(proc_macro_diagnostic)]


use syn::{
	ImplItem,
	ImplItemMethod,
	spanned::*
};


use proc_macro2::TokenStream as TokenStream2;


use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn trace(_: TokenStream, input: TokenStream) -> TokenStream {
	let input = TokenStream2::from(input);

	let item: ImplItem = syn::parse2(input).expect("failed to parse input");

	match &item {
		ImplItem::Method(ImplItemMethod { attrs, vis, sig, block, ..}) => {
			let ident = &sig.ident;
			let output = if let Some((last_stmt, stmts)) = &block.stmts.split_last() {
				quote::quote!{
					#(#attrs)*
					#vis #sig {
						scope.trace(format!("\n--> {}::{}", fmt_type!(Self), stringify!(#ident)));
						#(#stmts)*
						scope.trace(format!("<-- {}::{}\n", fmt_type!(Self), stringify!(#ident)));
						#last_stmt
					}
				}
			} else {
				quote::quote!{ #(item) }
			};
			output.into()
		}
		_ => {
			err_misuse(&item);
			quote::quote!(#item).into()
		}
	}
}

fn err_misuse(item: &ImplItem) {
	item.span()
		.unstable()
		.error("Trace can only be applied to functions")
		.emit()
}