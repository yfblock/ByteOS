use std::sync::Mutex;

use proc_macro2::Ident;
use quote::ToTokens;
use syn::parse_macro_input;

#[macro_use]
extern crate quote;

// 也许可以利用这个函数获得需要初始化的函数列表
#[proc_macro]
pub fn byteos_module_use(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let module_name = parse_macro_input!(input as syn::Ident);
    
    let func = quote! {
        #[allow(unused_imports)]
        use #module_name;
    };
    println!("func: {}", func);
    func.into()
}
