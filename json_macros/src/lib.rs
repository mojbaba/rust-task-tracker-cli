extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Serialize)]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Extract the fields from the struct
    let fields = if let syn::Data::Struct(ref data_struct) = input.data {
        &data_struct.fields
    } else {
        panic!("Serialize can only be derived for structs");
    };

    // create field serialization logic
    let serialize_fields = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_str = field_name.to_string();
        quote! {
            let field_json = format!("\"{}\": {}", #field_str , self.#field_name.serialize());
            result.push(field_json);
        }
    });

    // generate the serialize implementation
    let expanded = quote! {
        impl Serialize for #name {
            fn serialize(&self) -> String {
                let mut result = Vec::new();
                #(#serialize_fields)*
                format!("{{{}}}", result.join(", "))
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Deserialize)]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Extract the fields from the struct
    let fields = if let syn::Data::Struct(ref data_struct) = input.data {
        &data_struct.fields
    } else {
        panic!("Deserialize can only be derived for structs");
    };

    // Create field deserialization logic
    let deserialize_fields = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();  // unwrap safely
        let field_str = field_name.to_string();          // convert field name to string
        let field_ty = &field.ty;                        // get the type of the field
        quote! {
            let #field_name: #field_ty = {
                let value = json_map.remove(#field_str).ok_or("Field missing")?;
                #field_ty::deserialize(&value)?
            };
        }
    });

    // Generate the final deserialize implementation with proper iterables
    let field_names = fields.iter().map(|field| field.ident.as_ref().unwrap());

    let expanded = quote! {
        impl Deserialize for #name {
            fn deserialize(json: &str) -> Result<Self, &'static str> {
                let mut json_map = json_module::parse_json_to_map(json)?;
                #(#deserialize_fields)*

                Ok(Self {
                    #(#field_names),*
                })
            }
        }
    };

    TokenStream::from(expanded)
}
