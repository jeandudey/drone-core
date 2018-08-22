use inflector::Inflector;
use proc_macro2::{Span, TokenStream};
use syn::{parse2, DeriveInput, Ident};

pub fn proc_macro_derive(input: TokenStream) -> TokenStream {
  let def_site = Span::def_site();
  let input = parse2::<DeriveInput>(input).unwrap();
  let DeriveInput {
    ident, generics, ..
  } = input;
  let scope = Ident::new(
    &format!("__RESOURCE_{}", ident.to_string().to_screaming_snake_case()),
    def_site,
  );
  let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

  quote! {
    const #scope: () = {
      use extern::drone_core::drv::Resource;

      impl #impl_generics Resource for #ident #ty_generics #where_clause {
        type Source = Self;

        #[inline(always)]
        fn from_source(source: Self) -> Self {
          source
        }
      }
    };
  }
}