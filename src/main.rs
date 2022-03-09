use yew::prelude::*;

struct Task {
    name: String,
    done: bool,
}
struct Model {
    tasks: Vec<Task>,
}

pub enum Msg {
    SetInputChecked {
        idx: usize
    },
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self{
            tasks: vec![
                Task {
                    name: "task1".to_string(),
                    done: false,
                },
                Task {
                    name: "task2".to_string(),
                    done: false,
                },
                Task {
                    name: "task3".to_string(),
                    done: true,
                },
            ],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetInputChecked { idx } => {
                self.tasks[idx].done = !self.tasks[idx].done;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|idx| Msg::SetInputChecked { idx });
        let tasks = &self.tasks;
        html!{
            <ul>{tasks.into_iter().enumerate().map(|(idx, task)| {html! {
                <li key={task.name.as_str()} class={if task.done { "is-done" } else { "" }}>
                    <input type="checkbox" checked={task.done} onclick={onclick.reform(move |_| idx).clone()} />{task.name.clone()}
                </li>
            }}).collect::<Html>()}</ul>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}