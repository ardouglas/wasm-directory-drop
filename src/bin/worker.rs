use js_sys::{wasm_bindgen::prelude::Closure, Array};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    wasm_bindgen::JsValue, DedicatedWorkerGlobalScope, FileSystemDirectoryHandle, MessageEvent,
};

fn main() {
    wasm_logger::init(wasm_logger::Config::default().module_prefix("worker"));
    console_error_panic_hook::set_once();

    let scope = DedicatedWorkerGlobalScope::from(JsValue::from(js_sys::global()));

    let onmessage = Closure::wrap(Box::new(move |msg: MessageEvent| {
        let handle = msg.data().dyn_into::<FileSystemDirectoryHandle>().unwrap();

        let entries = handle.entries();

        wasm_bindgen_futures::spawn_local(async move {
            while let Ok(val) = entries.next() {
                let obj = JsFuture::from(val).await.unwrap();
                log::info!("{:?}", obj);
                // not seeing file entries here :(
            }
        });
    }) as Box<dyn Fn(MessageEvent)>);
    scope.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();
    scope
        .post_message(&Array::new().into())
        .expect("posting ready message succeeds");
}
