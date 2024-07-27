use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, FnArg, ItemFn, LitStr, Result, ReturnType, Type, TypeTuple};

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
    expand(attr, func, "__nucleus_post_")
}

#[proc_macro_attribute]
pub fn get(attr: TokenStream, func: TokenStream) -> TokenStream {
    expand(attr, func, "__nucleus_get_")
}

fn expand(attr: TokenStream, func: TokenStream, rename_prefix: &str) -> TokenStream {
    let func = parse_macro_input!(func as ItemFn);
    let func_block = &func.block;
    let func_decl = &func.sig;
    let origin_name = &func_decl.ident;
    let func_generics = &func_decl.generics;
    let func_inputs = &func_decl.inputs;
    let func_output = &func_decl.output;
    let attr = parse_macro_input!(attr as RenameFuncAttributeInput);
    let func_name = match attr.rename_to {
        Some(rename_to) => format_ident!("{}{}", rename_prefix, rename_to.value()),
        None => format_ident!("{}{}", rename_prefix, &func_decl.ident),
    };
    if !func_generics.params.is_empty() {
        panic!("api function should not have generics");
    }
    let tys: Vec<_> = func_inputs
        .iter()
        .map(|i| match i {
            FnArg::Typed(ref val) => val.ty.clone(),
            _ => unreachable!(""),
        })
        .collect();
    let rty = match func_output {
        ReturnType::Type(_, ty) => ty.clone(),
        ReturnType::Default => Box::new(Type::Tuple(TypeTuple {
            paren_token: Default::default(),
            elems: Default::default(),
        })),
    };
    let expanded = quote! {
        #[no_mangle]
        pub extern "C" fn #func_name(ptr: *const u8, len: usize) -> (std::boxed::Box<Vec<u8>>, usize) {
            let mut bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
            let decoded = <(#(#tys,)*) as codec::Decode>::decode(&mut bytes).unwrap();
            fn #origin_name(args: (#(#tys,)*)) #func_output #func_block
            let r = #origin_name(decoded);
            let v = <#rty as codec::Encode>::encode(&r);
            let v_len = v.len();
            (std::boxed::Box::new(v), v_len)
        }
    };
    expanded.into()
}
