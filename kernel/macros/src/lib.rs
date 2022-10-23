use syn::parse_macro_input;

#[macro_use]
extern crate quote;

#[proc_macro]
pub fn byteos_module_use(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let module_name = parse_macro_input!(input as syn::Ident);
    let as_name = format_ident!("module_use_{}", module_name);

    let func = quote! {
        #[allow(unused_imports)]
        use #module_name as #as_name;
    };
    
    let token: proc_macro::TokenStream = func.into();
    token
}
