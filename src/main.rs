use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "Hello from Yew!" }</h1>
            <p>{ "This is a minimal Yew+Trunk starter." }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
