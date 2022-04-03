#![deny(clippy::all)]

use fluent_uri::Uri as FUri;
use napi_derive::napi;
#[cfg(all(
  any(windows, unix),
  target_arch = "x86_64",
  not(target_env = "musl"),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[napi(object)]
#[allow(dead_code)]
pub struct Uri {
  pub scheme: String,
  pub authority: String,
  pub path: String,
  pub query: String,
  pub fragment: String,
}

#[napi]
pub fn parse(source: String) -> Option<Uri> {
  match FUri::parse(&source) {
    Ok(uri) => Some(Uri {
      scheme: uri.scheme()?.as_str().to_string(),
      authority: "".to_string(),
      path: uri.path().as_str().to_string(),
      query: uri.query()?.to_string(),
      fragment: uri.fragment()?.as_str().to_string(),
    }),
    Err(_) => None,
  }
}
