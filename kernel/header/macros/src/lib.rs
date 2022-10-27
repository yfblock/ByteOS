// #![no_std]

use syn::parse_macro_input;
use syn::{Token, LitInt, Path};
use syn::parse::{Error, Parse, ParseStream, Result};

#[macro_use]
extern crate quote;

#[proc_macro]
pub fn byteos_module_use(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let module_name = parse_macro_input!(input as syn::Ident);

    let func = quote! {
        #[allow(unused_imports)]
        use #module_name as _;
    };
    
    let token: proc_macro::TokenStream = func.into();
    token
}

enum Args {
    None,
    Path(Path),
    PathPos(Path, usize),
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            return Ok(Args::None);
        }
        let path: Path = input.parse()?;
        if input.is_empty() {
            return Ok(Args::Path(path));
        }
        input.parse::<Token![,]>()?;
        let lit: LitInt = input.parse()?;
        let pos: usize = lit.base10_parse()?;
        if pos > 9999 {
            return Err(Error::new(lit.span(), "maximum 9999 is supported"));
        }
        Ok(Args::PathPos(path, pos))
    }
}

#[proc_macro_attribute]
pub fn distributed_slice(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let attr = parse_macro_input!(attr as Args);
    let item = parse_macro_input!(item as syn::ItemFn);
    let linkme_attr = match attr {
        Args::None => quote!(#[header::linkme::distributed_slice]),
        Args::Path(path) => quote!(#[header::linkme::distributed_slice(#path)]),
        Args::PathPos(_, _) => todo!(),
    };
    quote! {
        #linkme_attr
        #[linkme(crate = header::linkme)]
        #item
    }.into()
}