use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{
    parse_macro_input, FnArg, Index, ItemFn, LitStr, Result, ReturnType, Type, TypeTuple,
    Visibility,
};

#[derive(Debug)]
struct RenameFuncAttributeInput {
    pub rename_to: Option<LitStr>,
}

impl Parse for RenameFuncAttributeInput {
    fn parse(input: ParseStream) -> Result<Self> {
        match <LitStr as Parse>::parse(input) {
            Ok(rename_to) => Ok(Self {
                rename_to: Some(rename_to),
            }),
            Err(_) => Ok(Self { rename_to: None }),
        }
    }
}

#[proc_macro_attribute]
pub fn init(_attr: TokenStream, func: TokenStream) -> TokenStream {
    let func = parse_macro_input!(func as ItemFn);
    let func_block = &func.block;
    let func_decl = &func.sig;
    let origin_name = &func_decl.ident;
    let func_generics = &func_decl.generics;
    let func_inputs = &func_decl.inputs;
    let func_output = &func_decl.output;

    let func_name = format_ident!("__nucleus_init_{}", &func_decl.ident);
    if !func_generics.params.is_empty() {
        panic!("init function should not have generics");
    }
    if !matches!(func_output, ReturnType::Default) {
        panic!("init function should have default return type");
    }
    let tys: Vec<_> = func_inputs
        .iter()
        .map(|i| match i {
            FnArg::Typed(ref val) => val.ty.clone(),
            _ => unreachable!(""),
        })
        .collect();
    let expanded = quote! {
        #[no_mangle]
        pub fn #func_name(args: std::vec::Vec<u8>) {
            let mut v: &[u8] = args.as_ref();
            let decoded = <(#(#tys,)*) as codec::Decode>::decode(&mut v).unwrap();
            fn #origin_name(args: (#(#tys,)*)) #func_block
            #origin_name(decoded);
        }
    };
    expanded.into()
}

#[proc_macro_attribute]
pub fn post(attr: TokenStream, func: TokenStream) -> TokenStream {
    expand(attr, func, "post")
}

#[proc_macro_attribute]
pub fn get(attr: TokenStream, func: TokenStream) -> TokenStream {
    expand(attr, func, "get")
}
fn expand(_attr: TokenStream, item: TokenStream, rename_prefix: &str) -> TokenStream {
    let mut input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let decoded_fn_name = format_ident!("__nucleus_{}_{}", rename_prefix, fn_name);

    // Remove the visibility modifier from the original function
    input_fn.vis = Visibility::Inherited;

    let args = &input_fn.sig.inputs;
    let return_type = &input_fn.sig.output;

    let param_types: Vec<_> = args
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                Some(&*pat_type.ty)
            } else {
                None
            }
        })
        .collect();

    let output_type = match return_type {
        ReturnType::Default => quote! { () },
        ReturnType::Type(_, ty) => quote! { #ty },
    };

    let function_call = if param_types.is_empty() {
        quote! {
            let result = #fn_name();
        }
    } else if param_types.len() == 1 {
        quote! {
            // Decode the single argument
            let mut args_bytes = unsafe { std::slice::from_raw_parts(__ptr, __len) };
            let args: #(#param_types)* = match Decode::decode(&mut args_bytes) {
                Ok(arg) => arg,
                Err(_) => return encode_error("Failed to decode argument".to_string()),
            };
            // Call the original function with the single parameter
            let result = #fn_name(args);
        }
    } else {
        let tuple_type = quote! { (#(#param_types),*) };
        let arg_indices: Vec<Index> = (0..param_types.len()).map(|i| Index::from(i)).collect();
        quote! {
            // Decode the tuple of arguments
            let mut args_bytes = unsafe { std::slice::from_raw_parts(__ptr, __len) };
            let args: #tuple_type = match Decode::decode(&mut args_bytes) {
                Ok(tuple) => tuple,
                Err(_) => return encode_error("Failed to decode arguments tuple".to_string()),
            };
            // Call the original function with extracted parameters
            let result = #fn_name(#(args.#arg_indices),*);
        }
    };

    let expanded = quote! {
        #[no_mangle]
        pub fn #decoded_fn_name(__ptr: *const u8, __len: usize) -> *const u8 {
            use codec::{Decode, Encode};

            fn encode_result<T: Encode>(result: T) -> *const u8 {
                let encoded = result.encode();
                let len = encoded.len() as u32;
                let mut output = Vec::with_capacity(4 + len as usize);
                output.extend_from_slice(&len.to_ne_bytes());
                output.extend_from_slice(&encoded);
                let ptr = output.as_ptr();
                std::mem::forget(output);
                ptr
            }

            fn encode_error(error: String) -> *const u8 {
                encode_result(Err::<Vec<u8>, String>(error))
            }

            // Place the original function inside the decoded function
            #input_fn

            // Call the function based on whether it has parameters or not
            #function_call

            let encoded = result.encode();
            let wrapped_result: Result<Vec<u8>, String> = Ok(encoded);
            encode_result(wrapped_result)
        }
    };

    TokenStream::from(expanded)
}
