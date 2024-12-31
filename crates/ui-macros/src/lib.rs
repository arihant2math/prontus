extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn unlock(_: TokenStream) -> TokenStream {
    let expanded = quote::quote! {
        let state = state.inner().inner();
        let state = state.read().await;
        let state = state.try_inner()?;
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn unlock_mut(_: TokenStream) -> TokenStream {
    let expanded = quote::quote! {
        let state = state.inner().inner();
        let mut state = state.write().await;
        let state = state.try_inner_mut()?;
    };

    TokenStream::from(expanded)
}
