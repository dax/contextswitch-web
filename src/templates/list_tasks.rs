use crate::components::add_task::AddTask;
use crate::components::tasks_list::TasksList;
use contextswitch_types::Task;
use perseus::{Html, Template};
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct ListTasksProps {
    tasks: Vec<Task>,
}

#[perseus::template(ListTasksPage)]
#[component(ListTasksPage<G>)]
pub fn list_tasks_page() -> View<G> {
    let ltasks: Signal<Vec<Task>> = Signal::new(Vec::new());

    if G::IS_BROWSER {
        perseus::spawn_local(cloned!(ltasks => async move {
            let tasks: Vec<Task> = reqwasm::http::Request::get("http://localhost:8000/tasks")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            ltasks.set(tasks);
        }));
    }

    view! {
        AddTask()
        TasksList(ltasks.handle())
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("list_tasks").template(list_tasks_page)
}
