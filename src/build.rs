use anyhow::Error;
use fehler::throws;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::fs;
use std::io::prelude::*;

#[throws(_)]
fn main() {
    println!("cargo:rerun-if-changed=tests/");
    let mut stream = TokenStream::new();
    for entry in fs::read_dir("tests/")? {
        let entry = entry?;
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "md" {
                let name = path.file_stem().unwrap().to_str().unwrap();
                let test_fn_name = Ident::new(&name, Span::call_site());
                let tokens = quote! {
                    #[test]
                    fn #test_fn_name() -> Result<(), crate::Error> {
                        crate::document::test(#name)?;
                        Ok(())
                    }
                };
                stream.extend(tokens);
            }
        }
    }
    let mut f = fs::File::create("src/tests.rs")?;
    f.write_all(stream.to_string().as_bytes())?;
}
