use proc_macro::TokenStream;
use quote::quote;
use syn::Item;

#[proc_macro]
pub fn run_templates(_input: TokenStream) -> TokenStream{
    let mut tokens = Vec::new();

    let content = std::fs::read_to_string("src/generated.rs").unwrap();
    let file = syn::parse_file(&content).unwrap();
    let items = file.items;

    for i in items{
        match i{
            Item::Mod(m) => {
                let ident = m.ident;
                let token = quote!{
                    #ident::generate_template()
                };
                tokens.push(token)
            },
            _ => unimplemented!()
        }
    }

    let expanded = quote!{
        {
             let mut templates: Vec<Template> = Vec::new();
            #(
            templates.push(#tokens);
             )*
            templates
        }

    };
    TokenStream::from(expanded)
}
