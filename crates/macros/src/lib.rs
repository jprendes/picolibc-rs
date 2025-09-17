use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, ItemStatic, Type, parse_macro_input};

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let fn_declaration = parse_macro_input!(item as ItemFn);

    let ident = fn_declaration.sig.ident.clone();

    let no_ret = match &fn_declaration.sig.output {
        syn::ReturnType::Type(_, ty) => matches!(**ty, Type::Never(_)),
        _ => false,
    };

    let output = if no_ret {
        quote! {
            #fn_declaration

            const _: () = {
                #[unsafe(no_mangle)]
                fn rust_start() {
                    #ident();
                    ::core::unreachable!();
                }
            };
        }
    } else {
        quote! {
            #fn_declaration

            const _: () = {
                #[unsafe(no_mangle)]
                fn rust_start() {
                    extern crate picolibc;
                    ::picolibc::process::Termination::report(#ident()).exit_process();
                }
            };
        }
    };

    output.into()
}

#[proc_macro_attribute]
pub fn host(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let host_static = parse_macro_input!(item as ItemStatic);

    let ident = host_static.ident.clone();

    let output = quote! {
        #host_static

        const _: () = {
            extern crate picolibc;
            #[unsafe(no_mangle)]
            fn _get_host() -> &'static dyn picolibc::host::Host {
                &#ident
            }
        };
    };

    output.into()
}
