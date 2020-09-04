use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Lit,Meta, parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Index, Type, Path, PathArguments, GenericArgument, Attribute};
use wirefilter::derive::Filterable;
use wirefilter::derive::GetType;
use wirefilter::{Scheme, ExecutionContext};

//https://github.com/dtolnay/syn/blob/master/examples/heapsize/heapsize_derive/src/lib.rs
//https://doc.rust-lang.org/book/ch19-06-macros.html#how-to-write-a-custom-derive-macro
//https://doc.rust-lang.org/reference/procedural-macros.html#derive-macro-helper-attributes
//https://docs.rs/syn/1.0.39/syn/struct.Attribute.html#parsing-from-attribute-to-structured-arguments
#[proc_macro_derive(Filterable, attributes(field))]
pub fn derive_filterable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);
    make_filterable(&input).into()
}

// non recursive for now
fn make_filterable(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let data = &input.data;
    let members = iter_members_filterable(data);

    quote! {
        impl Filterable for #name {
            fn filter_context<'s>(&self, schema: &'s Scheme) -> Result<ExecutionContext<'s>, wirefilter::errors::Error> {
                let mut ctx = ExecutionContext::new(schema);
                #members
                Ok(ctx)
            }
        }
    }
}

fn renamed_field(attrs: &Vec<Attribute>) -> Option<String> {
    for attr in attrs.iter() {
        let parsed_attr: Meta = attr.parse_args().unwrap();
        if let Meta::NameValue(pairs) = parsed_attr {
            let key = pairs.path.segments.first().unwrap();
            if key.ident.to_string() == "name" {
                let value = match pairs.lit {
                    Lit::Str(name) => {
                        return Some(name.value());
                    },
                    _ => {}
                };
            }
        }
    }
    None
}

fn iter_members_filterable(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields.named.iter().map(|f| {
                        let defined_name = renamed_field(&f.attrs);
                        let name = &f.ident;
                        let ty = &f.ty;
                        let check = match defined_name {
                            Some(c_defined_name) => {
                                quote_spanned! {f.span() =>
                                    &self.#name.generate_context(&mut ctx, #c_defined_name);
                                    println!("Type is {}", stringify!(#ty));
                                }
                            },
                            None => {
                                quote_spanned! {f.span() =>
                                    &self.#name.generate_context(&mut ctx, stringify!(#name));
                                    println!("Type is {}", stringify!(#ty));
                                }

                            }
                        };
                        quote_spanned! {f.span() =>
                            #check
                        }
                    });
                    quote! {
                        #(#recurse)*
                    }
                }
                Fields::Unit | Fields::Unnamed(_) => unimplemented!(),
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

#[proc_macro_derive(HasFields, attributes(field))]
pub fn derive_has_fields(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);
    make_has_fields(&input).into()
}

// non recursive for now
fn make_has_fields(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let data = &input.data;
    let members = iter_members_has_fields(data);

    quote! {
        impl HasFields for #name {
            fn fields() -> Vec<(String, Type)> {
                let mut new_fields: Vec<(String, Type)> = Vec::new();
                //println!("HERE {}", stringify!(#members));
                #members

                new_fields
            }
        }
    }
}
//stolen from: https://stackoverflow.com/questions/55271857/how-can-i-get-the-t-from-an-optiont-when-using-syn
fn path_is_option(path: &Path) -> bool {
    path.leading_colon.is_none()
        && path.segments.len() == 1
        && path.segments.iter().next().unwrap().ident == "Option"
}

fn path_is_vec(path: &Path) -> bool { //todo this is bahhhd said a sheep
    path.leading_colon.is_none()
        && path.segments.len() == 1
        && path.segments.iter().next().unwrap().ident == "Vec"
}

fn iter_members_has_fields(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        let ty = &f.ty;
                        let defined_name = renamed_field(&f.attrs).unwrap_or_else(|| name.as_ref().unwrap().to_string());
                        let r = match ty {
                            Type::Path(typepath) if typepath.qself.is_none() && path_is_option(&typepath.path) => {
                                let type_params = &(typepath.path.segments.iter().next()).unwrap().arguments;
                                let generic_arg = match type_params {
                                    PathArguments::AngleBracketed(params) => params.args.iter().next().unwrap(),
                                    _ => panic!("Missing Bracket"),
                                };
                                // This argument must be a type:
                                let gen = match generic_arg {
                                    GenericArgument::Type(ty) => ty,
                                    _ => panic!("Missing Generic"),
                                };
                                quote_spanned! {f.span() =>
                                    new_fields.push((String::from(#defined_name), Option::<#gen>::ty()));
                                }
                            },
                            Type::Path(typepath) if typepath.qself.is_none() && path_is_vec(&typepath.path) => {
                                let type_params = &(typepath.path.segments.iter().next()).unwrap().arguments;
                                let generic_arg = match type_params {
                                    PathArguments::AngleBracketed(params) => params.args.iter().next().unwrap(),
                                    _ => panic!("Missing Bracket"),
                                };
                                // This argument must be a type:
                                let gen = match generic_arg {
                                    GenericArgument::Type(ty) => ty,
                                    _ => panic!("Missing Generic"),
                                };
                                quote_spanned! {f.span() =>
                                    new_fields.push((String::from(#defined_name), Vec::<#gen>::ty()));
                                }
                            }
                            _ => {
                                quote_spanned! {f.span() =>
                                    new_fields.push((String::from(#defined_name), #ty::ty()));
                                    //new_fields.push((String::from(stringify!(#name)), GetType::ty<#ty>()));
                                }

                            }
                        };
                        r

                    });
                    quote! {
                        #(#recurse)*
                    }
                }
                Fields::Unit | Fields::Unnamed(_) => unimplemented!(),
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Filterable)]
    struct Empty;

    #[test]
    fn it_works() {
        let scheme = Scheme!(
          blah: Bytes
        );
        let e = Empty{};
        let exc = e.filter_context(scheme).unwrap();


        assert_eq!(2 + 2, 4);
    }
}
