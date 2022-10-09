#![feature(proc_macro_quote)]

use proc_macro::{quote, TokenStream};

use syn::{Expr, GenericArgument, Ident, ItemFn, LitStr, parse_macro_input, Path, Stmt, TraitBound, Type, TypeParamBound, TypeParen, TypePath, TypeTraitObject};
use syn::__private::{Span, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::PathArguments::AngleBracketed;
use syn::spanned::Spanned;
use syn::token::{Add, Comma};

struct TraitDefinition {
    ty: TypeTraitObject,
}

impl Parse for TraitDefinition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty: TypeTraitObject = input.parse()?;
        let mut bounds = ty.bounds;
        while input.peek(Comma) {
            input.parse::<Comma>()?;
            let bound: TypeParamBound = input.parse()?;
            bounds.push_punct(Add::default());
            bounds.push(bound);
        }
        Ok(TraitDefinition {
            ty: TypeTraitObject {
                dyn_token: ty.dyn_token,
                bounds,
            },
        })
    }
}

#[proc_macro]
pub fn add_traits(input: TokenStream) -> TokenStream {
    let info = parse_macro_input!(input as TraitDefinition);
    Type::TraitObject(info.ty).into_token_stream().into()
}

struct ServiceIdentifier {
    string: syn::LitStr,
}

fn stringify_path(span: Span, path: Path) -> syn::Result<String> {
    let last = path
        .segments
        .into_iter()
        .last()
        .ok_or_else(|| syn::Error::new(span, "Empty Type"))?;
    let mut result = last.ident.to_string();
    if let AngleBracketed(arguments) = last.arguments {
        result += "<";
        result += arguments
            .args
            .into_iter()
            .map(|argument| match argument {
                GenericArgument::Type(ty) => stringify_type(ty),
                _ => Err(syn::Error::new(span, "")),
            })
            .collect::<Result<Vec<_>, _>>()?
            .join(", ")
            .as_str();
        result += ">";
    }
    Ok(result)
}

fn stringify_type(ty: Type) -> syn::Result<String> {
    let span = ty.span();
    Ok(match ty {
        Type::Paren(TypeParen { elem, .. }) => stringify_type(*elem)?,
        Type::Path(TypePath {
            qself: None {},
            path,
        }) => stringify_path(span, path)?,
        Type::TraitObject(TypeTraitObject { bounds, .. }) => {
            if let Some(TypeParamBound::Trait(TraitBound { path, .. })) =
                bounds.into_iter().find(|bound| match bound {
                    TypeParamBound::Lifetime(_) => false,
                    TypeParamBound::Trait(_) => true,
                })
            {
                stringify_path(span, path)?
            } else {
                return Err(syn::Error::new(span, "Invalid TraitObject"));
            }
        }
        _ => return Err(syn::Error::new(span, "Invalid Type")),
    })
}

impl Parse for ServiceIdentifier {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty: Type = input.parse()?;
        let span = ty.span();
        let string = stringify_type(ty)?;
        let string = LitStr::new(format!("{string}").as_str(), span);
        Ok(ServiceIdentifier { string })
    }
}

#[proc_macro]
pub fn stringify_service_ref(input: TokenStream) -> TokenStream {
    let service_ref = parse_macro_input!(input as ServiceIdentifier);
    service_ref.string.into_token_stream().into()
}

struct ServiceDefinition {
    interface: Type,
    descriptor: Ident,
    lifetime: Expr,
    constructor: ItemFn,
}

impl Parse for ServiceDefinition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let interface = None;
        let descriptor = None;
        let lifetime = None;
        let constructor = None;

        while !matches!(
            (&interface, &descriptor, &lifetime, &constructor),
            (Some(_), Some(_), Some(_), Some(_))
        ) {
            // Todo: parse statement as one of the four parameters
            let item: Stmt = input.parse()?;
        }

        Ok(ServiceDefinition {
            interface: interface.unwrap(),
            descriptor: descriptor.unwrap(),
            lifetime: lifetime.unwrap(),
            constructor: constructor.unwrap(),
        })
    }
}

#[proc_macro]
pub fn minimax_service(input: TokenStream) -> TokenStream {
    let service_def = parse_macro_input!(input as ServiceDefinition);
    quote! {
        type Asd = i32;
    }.into()
}
