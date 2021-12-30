use crate::components::task_item::TaskItem;
use contextswitch_types::Task;
use sycamore::prelude::*;

#[component(TasksList<G>)]
pub fn tasks_list(value: ReadSignal<Vec<Task>>) -> View<G> {
    view! {
        ul {
            Keyed(KeyedProps {
                iterable: value,
                template: move |task| view! {
                    TaskItem(Signal::new(task).handle()) // TODO : Signal?
                },
                key: |task| task.uuid,
            })
        }
    }
}
