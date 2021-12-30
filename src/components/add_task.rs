use contextswitch_types::TaskDefinition;
use log::info;
use log::Level;
use sycamore::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, KeyboardEvent};

#[component(AddTask<G>)]
pub fn add_task() -> View<G> {
    let new_task_definition = Signal::new(String::new());
    console_log::init_with_level(Level::Debug).unwrap();

    let handle_submit = cloned!(new_task_definition => move |event: Event| {
        let event: KeyboardEvent = event.unchecked_into();

        if event.key() == "Enter" {
            let task_definition = new_task_definition.get().as_ref().clone().trim().to_string();

            if !task_definition.is_empty() {
                perseus::spawn_local(cloned!(task_definition => async move {
                    let body = serde_json::to_string(&TaskDefinition {
                        definition: task_definition,
                    }).unwrap();
                    info!("body: {:?}", body);
                    let _: serde_json::Value = reqwasm::http::Request::post("http://localhost:8000/tasks")
                        .header("content-type", "application/json")
                        .body(body)
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    //ltasks.set(tasks);
                }));
                new_task_definition.set("".to_string());
            }
        }
    });

    view! {
        input(bind:value=new_task_definition, on:keyup=handle_submit)
    }
}
