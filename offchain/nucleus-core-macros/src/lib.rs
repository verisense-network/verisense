use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, FnArg, ItemFn, ReturnType};

#[proc_macro_attribute]
pub fn init(_attr: TokenStream, func: TokenStream) -> TokenStream {
    let func = parse_macro_input!(func as ItemFn);
    let func_block = &func.block;

    let func_decl = &func.sig;
    // let func_name = &func_decl.ident;
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
    println!("{:?}", tys);
    let params: Vec<_> = func_inputs
        .iter()
        .map(|i| match i {
            FnArg::Typed(ref val) => val.pat.clone(),
            _ => unreachable!(""),
        })
        .collect();
    println!("{:?}", params);

    let expanded = quote! {
        #[no_mangle]
        pub fn #func_name(args: Vec<u8>) {
            let mut v = args.as_ref();
            let decoded = codec::Decode::<(#(#tys,)*)>::decode(&mut v).unwrap();
            fn func(args: (#(#tys,)*)) #func_block
            func(decoded);
        }
    };
    expanded.into()
}
