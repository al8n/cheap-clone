//! Derive implementation for `cheap_clone::CheapClone`.
#![forbid(unsafe_code)]

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{format_ident, quote, quote_spanned};
use syn::{
  parse_macro_input, parse_quote, parse_quote_spanned, punctuated::Punctuated, spanned::Spanned,
  Attribute, Data, DeriveInput, ExprPath, Fields, Generics, LitStr, Meta, Path, Token,
};

/// Derives field-wise cheap cloning. See `cheap_clone::CheapClone` for usage.
#[proc_macro_derive(CheapClone, attributes(cheap_clone))]
pub fn derive_cheap_clone(input: TokenStream) -> TokenStream {
  expand(parse_macro_input!(input as DeriveInput))
    .unwrap_or_else(syn::Error::into_compile_error)
    .into()
}

#[derive(Default)]
struct Options {
  inherent: bool,
  crate_path: Option<Path>,
}

impl Options {
  fn parse(attrs: &[Attribute]) -> syn::Result<Self> {
    let mut options = Self::default();
    for attr in attrs {
      if attr.path().is_ident("cheap_clone") {
        attr.parse_nested_meta(|meta| {
          if meta.path.is_ident("inherent") {
            if options.inherent {
              return Err(meta.error("duplicate `inherent` option"));
            }
            options.inherent = true;
          } else if meta.path.is_ident("crate") {
            if options.crate_path.is_some() {
              return Err(meta.error("duplicate `crate` option"));
            }
            options.crate_path = Some(meta.value()?.parse::<LitStr>()?.parse()?);
          } else {
            return Err(meta.error("expected `inherent` or `crate = \"path\"`"));
          }
          Ok(())
        })?;
      } else if attr.path().is_ident("repr") {
        let reprs = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
        for repr in reprs {
          if repr.path().is_ident("packed") {
            return Err(syn::Error::new_spanned(
              repr,
              "CheapClone cannot be derived for packed types",
            ));
          }
        }
      }
    }
    Ok(options)
  }

  fn resolve_crate(&self) -> syn::Result<Path> {
    if let Some(path) = &self.crate_path {
      return Ok(path.clone());
    }
    match crate_name("cheap-clone") {
      // An absolute path works in the library (via its self alias), integration
      // tests, examples, and doctests, which share the library's manifest.
      Ok(FoundCrate::Itself) => Ok(parse_quote!(::cheap_clone)),
      Ok(FoundCrate::Name(name)) => {
        // Raw identifiers also handle dependencies renamed to Rust keywords.
        let ident = format_ident!("r#{}", name);
        Ok(parse_quote!(::#ident))
      }
      Err(error) => Err(syn::Error::new(
        Span::call_site(),
        format!("{error}; use #[cheap_clone(crate = \"path\")] to set the crate path"),
      )),
    }
  }
}

fn field_function(attrs: &[Attribute]) -> syn::Result<Option<ExprPath>> {
  let mut function = None;
  for attr in attrs
    .iter()
    .filter(|attr| attr.path().is_ident("cheap_clone"))
  {
    attr.parse_nested_meta(|meta| {
      if !meta.path.is_ident("with") {
        return Err(meta.error("expected `with = \"path::to::function\"` on a field"));
      }
      if function.is_some() {
        return Err(meta.error("duplicate `with` option"));
      }
      function = Some(meta.value()?.parse::<LitStr>()?.parse()?);
      Ok(())
    })?;
  }
  Ok(function)
}

fn expand(input: DeriveInput) -> syn::Result<TokenStream2> {
  let options = Options::parse(&input.attrs)?;
  let crate_path = options.resolve_crate()?;
  let trait_path: Path = parse_quote!(#crate_path::__private::CheapClone);
  let name = &input.ident;
  let mut generics = input.generics.clone();
  generics
    .make_where_clause()
    .predicates
    .push(parse_quote!(Self: ::core::clone::Clone));

  let body = match &input.data {
    Data::Struct(data) => {
      let arm = fields_arm(&data.fields, quote!(Self), &trait_path, &mut generics)?;
      quote!(match self { #arm })
    }
    Data::Enum(data) => {
      let mut arms = Vec::new();
      for variant in &data.variants {
        for attr in &variant.attrs {
          if attr.path().is_ident("cheap_clone") {
            return Err(syn::Error::new_spanned(
              attr,
              "`cheap_clone` attributes belong on the type or its fields, not variants",
            ));
          }
        }
        let ident = &variant.ident;
        arms.push(fields_arm(
          &variant.fields,
          quote!(Self::#ident),
          &trait_path,
          &mut generics,
        )?);
      }
      if arms.is_empty() {
        // Match on the empty enum itself so no wildcard arm is required.
        quote!(match *self {})
      } else {
        quote!(match self { #(#arms),* })
      }
    }
    Data::Union(data) => {
      return Err(syn::Error::new_spanned(
        data.union_token,
        "CheapClone cannot be derived for unions",
      ));
    }
  };

  let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
  let inherent = options.inherent.then(|| {
    quote! {
      #[automatically_derived]
      impl #impl_generics #name #ty_generics #where_clause {
        /// Returns a cheaply cloned copy by calling `CheapClone::cheap_clone`.
        #[inline]
        pub fn cheap_clone(&self) -> Self {
          <Self as #trait_path>::cheap_clone(self)
        }
      }
    }
  });

  Ok(quote! {
    #[automatically_derived]
    impl #impl_generics #trait_path for #name #ty_generics #where_clause {
      #[inline]
      fn cheap_clone(&self) -> Self {
        #body
      }
    }
    #inherent
  })
}

fn fields_arm(
  fields: &Fields,
  constructor: TokenStream2,
  trait_path: &Path,
  generics: &mut Generics,
) -> syn::Result<TokenStream2> {
  let mut bindings = Vec::new();
  let mut values = Vec::new();
  for (index, field) in fields.iter().enumerate() {
    let binding = format_ident!("__cheap_clone_{index}", span = Span::mixed_site());
    let function = field_function(&field.attrs)?;
    let ty = &field.ty;
    let value = if let Some(function) = function {
      quote_spanned!(field.span()=> (#function)(#binding))
    } else {
      generics
        .make_where_clause()
        .predicates
        .push(parse_quote_spanned!(ty.span()=> #ty: #trait_path));
      quote_spanned!(field.span()=> <#ty as #trait_path>::cheap_clone(#binding))
    };
    bindings.push(binding);
    values.push(value);
  }
  Ok(match fields {
    Fields::Named(fields) => {
      let names: Vec<_> = fields.named.iter().map(|field| &field.ident).collect();
      quote! {
        #constructor { #(#names: #bindings),* } => #constructor { #(#names: #values),* }
      }
    }
    Fields::Unnamed(_) => quote! {
      #constructor(#(#bindings),*) => #constructor(#(#values),*)
    },
    Fields::Unit => quote!(#constructor => #constructor),
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  fn error(input: TokenStream2) -> String {
    let input = syn::parse2(quote! {
      #[cheap_clone(crate = "::cheap_clone")]
      #input
    })
    .unwrap();
    expand(input).unwrap_err().to_string()
  }

  #[test]
  fn rejects_unknown_and_misplaced_options() {
    assert!(error(quote! {
      #[cheap_clone(skip)] struct Value;
    })
    .contains("expected `inherent` or `crate"));
    assert!(error(quote! {
      struct Value { #[cheap_clone(inherent)] value: u8 }
    })
    .contains("expected `with"));
    assert!(error(quote! {
      enum Value { #[cheap_clone(inherent)] Empty }
    })
    .contains("not variants"));
    assert!(error(quote! {
      #[cheap_clone(with = "copy")] struct Value;
    })
    .contains("expected `inherent` or `crate"));
  }

  #[test]
  fn rejects_duplicates_across_attributes() {
    assert_eq!(
      error(quote! {
        #[cheap_clone(inherent)] #[cheap_clone(inherent)] struct Value;
      }),
      "duplicate `inherent` option"
    );
    assert_eq!(
      error(quote! {
        #[cheap_clone(crate = "::other")] struct Value;
      }),
      "duplicate `crate` option"
    );
    assert_eq!(
      error(quote! {
        struct Value {
          #[cheap_clone(with = "copy")]
          #[cheap_clone(with = "other")]
          value: u8,
        }
      }),
      "duplicate `with` option"
    );
  }

  #[test]
  fn rejects_invalid_function_paths_and_option_values() {
    for input in [
      quote!(
        struct Value(#[cheap_clone(with = 42)] u8);
      ),
      quote!(
        struct Value(#[cheap_clone(with = "not a path")] u8);
      ),
      quote!(
        struct Value(#[cheap_clone(with = "|x| *x")] u8);
      ),
      quote!(
        struct Value(#[cheap_clone(with)] u8);
      ),
      quote!(
        #[cheap_clone(inherent = true)]
        struct Value;
      ),
    ] {
      assert!(!error(input).is_empty());
    }
  }

  #[test]
  fn rejects_unions_and_all_packed_representations() {
    assert_eq!(
      error(quote! {
        union Value { field: u32 }
      }),
      "CheapClone cannot be derived for unions"
    );
    for repr in [quote!(packed), quote!(C, packed), quote!(C, packed(2))] {
      assert_eq!(
        error(quote! {
          #[repr(#repr)] struct Value(u32);
        }),
        "CheapClone cannot be derived for packed types"
      );
    }
  }
}
