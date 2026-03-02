use proc_macro::TokenStream;
use syn::{DeriveInput, FieldsNamed, parse_macro_input};
use quote::quote;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let bname = format!("{}Builder", name);
    let bident = syn::Ident::new(&bname, name.span()); 
    let fields = if let syn::Data::Struct(syn::DataStruct{fields:syn::Fields::Named(syn::FieldsNamed{ref named,..}),..}) = &input.data {
        named
    } else {
        unimplemented!()
    };
    eprint!("{:#?}", input);

    let optionized = fields.iter().map(|field| {
       let ident = &field.ident;
       let ty = &field.ty;
       quote!{
            #ident: std::option::Option<#ty>
       }
    });

    let methods = fields.iter().map(|field| {
        let ident = &field.ident;
        let ty = &field.ty;
        quote!{
            pub fn #ident(&mut self, #ident: #ty) -> &mut Self {
                self.#ident = Some(#ident);
                self
            }
        }
    });
    let build_fields = fields.iter().map(|field| {
        let ident = &field.ident;
        quote!{
                #ident: self.#ident.clone().ok_or(concat!(stringify!(#ident), " is missing"))?
        }
    });

    let build_empty = fields.iter().map(|field| {
        let ident = &field.ident;
        quote!{
                #ident: None
        }
    });
    
  let expanded = quote! {
        pub struct #bident {
        #(#optionized,)* 
    }


    impl #bident{

        #(#methods)*

        pub fn build(&self)-> Result<#name, Box<dyn std::error::Error>> {
            
            
            
            Ok(#name {
                #(#build_fields,)*
            })
            
        }
    }

        impl #name {
            pub fn builder()-> #bident {
                #bident {
                    #(#build_empty,)*
                }
        }

    }
 };
    expanded.into()
}
