use crate::{Configuration, Create};
use dioxus::prelude::*;
use ory_kratos_client_wasm::apis::metadata_api::is_ready;

#[component]
pub fn Index() -> Element {
  let ready = use_resource(move || async move { is_ready(&Configuration::create()).await });

  return match &*ready.read() {
    Some(result) => match result {
      Ok(status) => {
        rsx! {
          p { {format!("{status:?}")} }
        }
      }
      Err(err) => {
        rsx! {
          p { {format!("{err:?}")} }
        }
      }
    },
    None => rsx! {
      p { "Waiting..." }
    },
  };
}
