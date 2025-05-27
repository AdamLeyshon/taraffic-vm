use proc_macro::TokenStream;
use quote::{quote};
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(DisplayInstruction)]
pub fn derive_display(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    
    // Get the name of the enum
    let name = &input.ident;
    
    // Only process if it's an enum
    let data_enum = match &input.data {
        Data::Enum(data_enum) => data_enum,
        _ => panic!("Display can only be derived for enums"),
    };
    
    // Generate match arms for each variant
    let match_arms = data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        
        match &variant.fields {
            // No fields (e.g., NOP, SCR)
            Fields::Unit => {
                quote! {
                    #name::#variant_name => write!(f, "{}", stringify!(#variant_name)),
                }
            },
            // Named fields - not used in this enum
            Fields::Named(_) => {
                panic!("Named fields are not supported")
            },
            // Tuple fields (e.g., PUSH(OperandValueType), ADD(Register, Register))
            Fields::Unnamed(fields) => {
                let field_count = fields.unnamed.len();
                
                match field_count {
                    0 => {
                        quote! {
                            #name::#variant_name => write!(f, "{}", stringify!(#variant_name)),
                        }
                    },
                    1 => {
                        quote! {
                            #name::#variant_name(a) => write!(f, "{} {}", stringify!(#variant_name), a),
                        }
                    },
                    2 => {
                        quote! {
                            #name::#variant_name(a, b) => write!(f, "{} {}, {}", stringify!(#variant_name), a, b),
                        }
                    },
                    3 => {
                        quote! {
                            #name::#variant_name(a, b, c) => write!(f, "{} {}, {}, {}", stringify!(#variant_name), a, b, c),
                        }
                    },
                    _ => panic!("More than 3 fields are not supported"),
                }
            },
        }
    });
    
    // Generate the implementation
    let expanded = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#match_arms)*
                }
            }
        }
    };
    
    // Return the generated code
    TokenStream::from(expanded)
}