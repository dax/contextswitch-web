use contextswitch_types::Task;
use sycamore::prelude::*;

#[component(TaskItem<G>)]
pub fn task_item(value: ReadSignal<Task>) -> View<G> {
    view! {
        p { (value.get().description) }
    }
}
