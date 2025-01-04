extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashMap;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Expr, Ident, Token, Type};

struct APIEndpoint {
    method: Ident,
    url: Expr,
    response: Type,
    request: Type,
    extra_args: HashMap<String, Option<Expr>>,
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
        let mut extra_args = HashMap::new();
        while !input.is_empty() {
            input.parse::<Token![,]>()?;
            let key: Ident = input.parse()?;
            if let Ok(_) = input.parse::<Token![=]>() {
                let value: Expr = input.parse()?;
                extra_args.insert(key.to_string(), Some(value));
            } else {
                extra_args.insert(key.to_string(), None);
            }
        }
        Ok(Self {
            method,
            url,
            request,
            response,
            extra_args,
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
/// // client_macros::api!(get, "v2/bubble.info", GetBubbleInfoResponse, GetBubbleInfoRequest);
/// ```
#[proc_macro]
pub fn api(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let APIEndpoint {
        method,
        url,
        response,
        request,
        extra_args,
    } = syn::parse_macro_input!(input as APIEndpoint);

    if method != "get"
        && method != "post"
        && method != "put"
        && method != "delete"
        && method != "patch"
    {
        return syn::Error::new(
            method.span(),
            "Invalid HTTP method. Must be one of: get, post, put, patch, delete",
        )
        .to_compile_error()
        .into();
    }

    let has_request = !matches!(request, Type::Never(_));

    let response_name = match response.clone() {
        Type::Path(p) => {
            let segments = p.path.segments;
            if segments.len() != 1 {
                return syn::Error::new(
                    response.span(),
                    "Response type must be a wrapped Result type",
                )
                .to_compile_error()
                .into();
            }
            let ident = segments[0].ident.clone();
            // TODO: hack lol
            format_ident!("{}", &ident.to_string().replace("Result", "Response"))
        }
        _ => unimplemented!("response type must be a Path Type"),
    };

    let process = quote! {
        let text = r.text().await?;
        log::trace!("Response: {}", text);
    };

    let parse = if let Some(parse_expr) = extra_args.get("parse") {
        quote! {
            return #parse_expr;
        }
    } else {
        quote! {
            let json = serde_json::from_str(&text);
            match json {
                Ok(json) => {
                    return Ok(json);
                }
                Err(e) => {
                    let json = serde_json::from_str::<#response_name>(&text);
                    let e = json.unwrap_err();
                    log::error!("Error parsing json response: {:?}.", e);
                    let json = serde_json::from_str::<serde_json::Value>(&text);
                    if json.is_err() {
                        return Err(crate::ResponseError::NotJson(text));
                    }
                    return Err(crate::ResponseError::from(e));
                }
            }
        }
    };

    let send_request = if let Some(request_expr) = extra_args.get("request") {
        quote! {
            #request_expr
        }
    } else if has_request {
        if method == "get" {
            quote! {
                client
                    .#method(format!("{pronto_base_url}{}", #url))
                    .query(&request)
                    .send()
                    .await?
            }
        } else {
            quote! {
                client
                    .#method(format!("{pronto_base_url}{}", #url))
                    .json(&request)
                    .send()
                    .await?
            }
        }
    } else {
        quote! {
            client
                .#method(format!("{pronto_base_url}{}", #url))
                .send()
                .await?
        }
    };

    let function_name = quote! {
        #method
    };

    let types = if has_request {
        quote! {
            pronto_base_url: &str,
            client: &reqwest::Client,
            request: #request,
        }
    } else {
        quote! {
            pronto_base_url: &str,
            client: &reqwest::Client,
        }
    };

    // Build the output
    let expanded = quote! {
        #[doc = "Sends a #method request to #url."]
        pub async fn #function_name(
            #types
        ) -> Result<#response, crate::ResponseError> {
            let initial_time = std::time::Instant::now();
            let r = #send_request;
            let elapsed = initial_time.elapsed();
            log::debug!(target: "request_perf", "Network: {} ms", elapsed.as_millis());
            #process
            #parse
        }
    };

    TokenStream::from(expanded)
}
