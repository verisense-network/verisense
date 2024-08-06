use codec::Decode;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("cargo::rerun-if-changed=../runtime/src");
    let blob: Vec<u8> = vrs_runtime::Runtime::metadata().into();
    let mut f = File::create("metadata.scale").expect("couldn't create metadata.scale");
    f.write_all(&blob).expect("couldn't write metadata blob");
    let mut codegen = subxt_codegen::CodegenBuilder::new();
    codegen.no_docs();
    codegen.set_target_module(subxt_codegen::syn::parse_str("pub mod codegen {}").unwrap());
    let metadata =
        subxt_metadata::Metadata::decode(&mut &*blob).expect("couldn't decode metadata blob");
    let code = codegen
        .generate(metadata)
        .expect("couldn't resolve metadata blob");
    let mut f = File::create("src/runtime.rs").expect("couldn't create codegen file");
    writeln!(f, "{code}").expect("couldn't generate code");
    std::process::Command::new("cargo-fmt").spawn().expect("");
}
