use proc_macro::TokenStream;
use quote::quote;

/// `cache_this` macro enabling easy caching of expressions
#[proc_macro]
pub fn cache_this(input: TokenStream) -> TokenStream {
    let cache_file_name = format!("{}", input).replace(' ', "");
    let ast: syn::Expr = syn::parse(input).expect("Unable to parse expr");

    let tokens = quote! {
         {
            use std::io::{Write, Read};
            use std::path::Path;
            use std::fs::File;

            const CACHE_FOLDER_NAME : &str = ".cache-this";

            let cache_file_path = Path::new(CACHE_FOLDER_NAME).join(#cache_file_name);

            if cache_file_path.exists() {
                let mut buffer = String::new();

                File::open(&cache_file_path)
                    .expect("Unable to open cached file")
                    .read_to_string(&mut buffer)
                    .expect("Unable to read cached file into buffer");

                ::cache_this::from_str(&buffer).expect("Unable to deserialize cached result")
            } else {
                let result = #ast;

                std::fs::create_dir_all(CACHE_FOLDER_NAME)
                    .expect("Unable to create .cache-this folder");

                let serialized_result = ::cache_this::to_string(&result).unwrap();
                File::create(&cache_file_path)
                    .expect("Unable to create cache file")
                    .write_all(serialized_result.as_bytes())
                    .expect("Unable to write cached result to file");

                result
            }
        }
    };

    tokens.into()
}
