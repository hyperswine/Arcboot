#![no_std]

extern crate proc_macro;
use proc_macro::TokenStream;

// NOTES:
// ItemStatic => struct and stuff
// ItemFn => what we want for entry and handler

#[proc_macro_attribute]
pub fn arc_entry(args: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    quote::quote! {
        #[no_mangle]
        #input
    }
    .into()
}

#[proc_macro_attribute]
pub fn register_exception_handler(args: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    quote::quote! {
        // .exceptions is an executable, rdonly section linked by arcboot
        #[link_section = ".exceptions"]
        #input
    }
    .into()
}
