use yew::prelude::*;

struct Model {
    tasks: Vec<String>,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self{
            tasks: vec![
                "task1".to_string(),
                "task2".to_string(),
                "task3".to_string(),
            ],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        todo!()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let tasks = &self.tasks;
        html!{
            <ul>{tasks.into_iter().map(|task| {html! {
                <li key={task.as_str()}>{task}</li>
            }}).collect::<Html>()}</ul>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}