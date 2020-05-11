extern crate proc_macro;
#[macro_use] extern crate quote;
extern crate syn;

use quote::quote;
use syn::*;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn auto_vec(_head: TokenStream, body: TokenStream) -> TokenStream {
    match parse::<Item>(body).unwrap() {
        Item::Fn(ref func) => {
            let ItemFn {
                attrs, vis, constness, unsafety,
                abi, ident, decl, block,
            } = func;
            use std::ops::Deref;
            let FnDecl {
                generics, inputs, variadic, output, ..
            } = decl.deref();

            let mut args = vec![];
            let mut arg_names = vec![];
            let mut i = 0;

            for arg in inputs.iter() {
                match *arg {
                    FnArg::Captured(ref cap) => {
                        let ty = &cap.ty;
                        let pat = &cap.pat;
                        args.push(quote!(#pat: Vec<#ty>));
                        arg_names.push(quote!(#pat));
                    }
                    _ => panic!("Unexpected argument {:?}", arg)
                }
                i += 1;
            }

            if i < 2 {
                println!(" i = {}", i);
            }

            let newout;
            let ret_type;
            match output {
                ReturnType::Type(a, b) => {
                     newout = quote!(#a Vec<#b>); 
                     ret_type = quote!(#b); 
                     println!("newout: {}", newout);
                }
                _ => panic!("Not supported on functions without return types!"),
            }

            let fname = Ident::from(format!("{}_inner", ident));
            let first_ident = &arg_names.first();
            let fn_args = &arg_names[1..];

            let gen = quote! {
                #(#attrs)* #vis #constness #unsafety #abi
                fn #fname #generics ( #( #inputs ),* #variadic ) #output {
                    #block;
                }

                #(#attrs)* #vis #constness #unsafety #abi
                fn #ident #generics ( #( #args ),* #variadic ) #newout {
                    println!("begin");

                    let mut len = #first_ident.len();
                    #( if len != #fn_args.len() { panic!("Length is wrong"); } )*

                    let mut res: Vec<#ret_type> = Vec::new();
                    let i : i32;

                    for i in 0..len {
                        let mut tmp = #fname(#(#arg_names[i]),*);
                        res.push(tmp);
                    }
                    return res;
                }

            };
            println!("gen: {}", gen);
            gen.into()
        }
        _ => panic!("Only fn is allowed!"),
    }
}

