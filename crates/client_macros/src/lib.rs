extern crate proc_macro;
use proc_macro::{TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Token, Type, Ident};

struct APIEndpoint {
    method: Ident,
    url: Expr,
    response: Type,
    request: Type,
}

impl Parse for APIEndpoint {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let method: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let url: Expr = input.parse()?;
        input.parse::<Token![,]>()?;
        let response: Type = input.parse()?;
        input.parse::<Token![,]>()?;
        let request: Type = input.parse()?;
        Ok(Self {
            method,
            url,
            request,
            response,
        })
    }
}

/// This macro generates a function that sends an API request to the Pronto API.
/// The generated function has the name of the HTTPS method (e.g., `get`, `post`, `put`, `delete`, etc.)
///
/// This macro takes four arguments:
/// - The HTTP method to use (e.g., `get`, `post`, `put`, `delete`, etc.)
/// - The URL path to send the request to
/// - The response type to expect from the API
/// - The request type to send to the API
///
/// # Example
/// ```no_run
/// // Generates a function named `get` that sends a GET request v2/bubble.info when passed
/// // a Pronto base URL, a reqwest client, and a GetBubbleInfoRequest.
/// client_macros::api!(get, "v2/bubble.info", GetBubbleInfoResponse, GetBubbleInfoRequest);
/// ```
#[proc_macro]
pub fn api(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let APIEndpoint {
        method,
        url,
        response,
        request,
    } = syn::parse_macro_input!(input as APIEndpoint);

    if method != "get" && method != "post" && method != "put" && method != "delete" && method != "patch" {
        return syn::Error::new(method.span(), "Invalid HTTP method. Must be one of: get, post, put, patch, delete").to_compile_error().into();
    }

    let has_request = match request {
        Type::Never(_) => false,
        _ => true,
    };

    // Build the output
    let expanded = if has_request {
        quote! {
            pub async fn #method(
                pronto_base_url: &str,
                client: &reqwest::Client,
                request: #request,
            ) -> Result<#response, crate::ResponseError> {
                let r = client
                    .#method(format!("{pronto_base_url}{}", #url))
                    .json(&request)
                    .send()
                    .await?;
                let text = r.text().await?;
                let json = serde_json::from_str(&text);
                match json {
                    Ok(json) => {
                        return Ok(json);
                    }
                    Err(e) => {
                        log::error!("Error parsing JSON response: {:?}", e);
                        return Err(crate::ResponseError::from(e));
                    }
                }
            }
        }
    } else {
        quote! {
            /// Sends a #method request to #url.
            pub async fn #method(
                pronto_base_url: &str,
                client: &reqwest::Client,
            ) -> Result<#response, crate::ResponseError> {
                let r = client
                    .#method(format!("{pronto_base_url}{}", #url))
                    .send()
                    .await?;
                let text = r.text().await?;
                let json = serde_json::from_str(&text);
                match json {
                    Ok(json) => {
                        return Ok(json);
                    }
                    Err(e) => {
                        log::error!("Error parsing json response: {:?}, text: {}.", e, text);
                        return Err(crate::ResponseError::from(e));
                    }
                }
            }
        }
    };

    TokenStream::from(expanded)
}
