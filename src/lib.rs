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
      scheme: uri
        .scheme()
        .map(|scheme| scheme.as_str().to_string())
        .unwrap_or_default(),
      authority: "".to_string(),
      path: uri.path().as_str().to_string(),
      query: uri
        .query()
        .map(|q| q.as_str().to_string())
        .unwrap_or_default(),
      fragment: uri
        .fragment()
        .map(|f| f.as_str().to_string())
        .unwrap_or_default(),
    }),
    Err(_) => None,
  }
}
