//! Default Compute template program.

use fastly::http::{header, Method, StatusCode};
use fastly::{mime, Error, Request, Response};

/// Fastly Compute server for Trusted Types Test site.
///
/// Serves and index page that includes a "fiddle" page in an iframe.
/// Serves the "fiddle" page with different varitions of Content Security Policy headers.

#[fastly::main]
fn main(mut req: Request) -> Result<Response, Error> {
    // Log service version
    println!(
        "FASTLY_SERVICE_VERSION: {}",
        std::env::var("FASTLY_SERVICE_VERSION").unwrap_or_else(|_| String::new())
    );

    // Block requests with unexpected methods
    match req.get_method() {
        &Method::PUT | &Method::PATCH | &Method::DELETE => {
            return Ok(
                Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
                    .with_header(header::ALLOW, "GET, HEAD, PURGE")
                    .with_body_text_plain("This method is not allowed\n"),
            );
        }
        // Let any other requests through
        _ => (),
    }

    // Serve site content
    match (req.get_method(), req.get_path()) {
        (&Method::GET, "/") => Ok(
            Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_body(include_str!("index.html")),
        ),
        (&Method::GET, "/off") => Ok(
            Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_body(include_str!("fiddle.html")),
        ),
        (&Method::GET, "/report-only") => Ok(
            Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_header(
                    "Content-Security-Policy-Report-Only",
                    "require-trusted-types-for 'script'; report-uri https://foote.report-uri.com/r/d/csp/reportOnly",
                )
                .with_body(include_str!("fiddle.html")),
        ),
        (&Method::GET, "/block") => Ok(
            Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_header(
                    "Content-Security-Policy",
                    "require-trusted-types-for 'script'; report-uri https://foote.report-uri.com/r/d/csp/block",
                )
                .with_body(include_str!("fiddle.html")),
        ),
        // Catch all other requests and return a 404.
        _ => Ok(
            Response::from_status(StatusCode::NOT_FOUND)
                .with_body_text_plain("The page you requested could not be found\n"),
        ),
    }
}
