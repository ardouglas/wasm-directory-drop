use gloo::utils::window;
use js_sys::Array;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    wasm_bindgen::{prelude::Closure, JsCast},
    Blob, BlobPropertyBag, DataTransfer, FileSystemDirectoryHandle, MessageEvent, Url, Worker,
};
use yew::Component;

fn worker_new(name: &str) -> Worker {
    let origin = window()
        .location()
        .origin()
        .expect("origin to be available");

    let script = Array::new();
    script.push(
        &format!(r#"importScripts("{origin}/{name}.js");wasm_bindgen("{origin}/{name}_bg.wasm");"#)
            .into(),
    );

    let properties = BlobPropertyBag::new();
    properties.set_type("text/javascript");
    let blob = Blob::new_with_str_sequence_and_options(&script, &properties)
        .expect("blob creation succeeds");

    let url = Url::create_object_url_with_blob(&blob).expect("url creation succeeds");

    Worker::new(&url).expect("failed to spawn worker")
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    log::info!("hello from main!");
    let worker = worker_new("worker");

    let document = gloo::utils::document();
    let element = document.get_element_by_id("drop-zone").unwrap();
    let _app = yew::Renderer::<App>::with_root(element).render();

    let onmessage = Closure::wrap(Box::new(move |msg: MessageEvent| {
        let data = Array::from(&msg.data());

        if data.length() == 0 {
            log::info!("worker initialized")
        } else {
            // not expected to send back anything besides the first empty message
        }
    }) as Box<dyn Fn(MessageEvent)>);
    worker.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
    onmessage.forget();

    set_dragover_cb();
    set_drop_cb(worker.clone());
}

fn set_dragover_cb() {
    let document = gloo::utils::document();
    let elt = document
        .get_element_by_id("drop-zone")
        .expect("Couldn't get the drop element");
    let cb = Closure::<dyn FnMut(_)>::new(move |event: web_sys::Event| {
        event.prevent_default();
    });
    elt.add_event_listener_with_callback("dragover", cb.as_ref().unchecked_ref())
        .unwrap();

    cb.forget();
}

fn set_drop_cb(worker: Worker) {
    let document = gloo::utils::document();
    let elt = document
        .get_element_by_id("drop-zone")
        .expect("Couldn't get the drop element");
    let cb = Closure::<dyn FnMut(_)>::new(move |event: web_sys::Event| {
        event.prevent_default();

        let drop_event = event.dyn_into::<web_sys::DragEvent>().unwrap();

        if let Some(dt) = drop_event.data_transfer() {
            if dt.items().length() > 0 {
                parse_dt_items(&dt, worker.clone())
            } else {
                unimplemented!("focusing on directory drag 'n drop");
            };
        }
    });
    elt.add_event_listener_with_callback("drop", cb.as_ref().unchecked_ref())
        .unwrap();

    cb.forget();
}

struct App {}

enum Msg {}

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        yew::html! {
            <p>{"Drag 'n drop a directory here!"}</p>
        }
    }
}

fn parse_dt_items(dt: &DataTransfer, worker: Worker) {
    for i in 0..dt.items().length() {
        let item = dt.items().get(i).expect("index < items.length()");

        let entry = item.webkit_get_as_entry().unwrap().unwrap();

        if entry.is_directory() {
            let wc = worker.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let entry = JsFuture::from(item.get_as_file_system_handle())
                    .await
                    .unwrap()
                    .dyn_into::<FileSystemDirectoryHandle>()
                    .unwrap();

                wc.post_message(&entry.into())
                    .expect("the worker should be alive and receiving messages");
            });
        } else if item.kind() == "file" {
            unimplemented!("focusing on just directory drop for this example")
        }
    }
}
