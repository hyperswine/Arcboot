#![no_std]

// ------------
// API EXPORT
// ------------

// BUILD SEPARATELY

extern crate proc_macro;

use proc_macro::{TokenStream};

#[proc_macro_attribute]
pub fn stivale2hdr(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemStatic);
    let ty = &input.ty;

    quote::quote! {
        // ensures that the type of the header is `v2::StivaleHeader`.
        const _: () = { fn __sheader_ty_chk(e: #ty) -> ::stivale_boot::v2::StivaleHeader { e } };

        #[link_section = ".stivale2hdr"]
        #[no_mangle]
        #[used]
        #input
    }
    .into()
}

#[proc_macro]
fn register_exception_handler() {}
