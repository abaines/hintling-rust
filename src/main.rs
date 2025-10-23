use yew::prelude::*;
use web_sys::console;

#[function_component(App)]
fn app() -> Html {
    let counter = use_state(|| 0i32);

    {
        // Log to the browser console whenever the counter changes
        let counter_val = *counter;
        use_effect_with(
            counter_val,
            move |c| {
                console::log_1(&format!("Counter changed: {}", *c).into());
                || ()
            },
        );
    }

    let on_inc = {
        let counter = counter.clone();
        Callback::from(move |_| {
            counter.set(*counter + 1);
        })
    };

    let on_dec = {
        let counter = counter.clone();
        Callback::from(move |_| {
            counter.set(*counter - 1);
        })
    };

    html! {
        // Make the app area fill the viewport with zero padding/margin so side buttons can touch edges
        <div class="app-container">

            <div class="row">
                // Left column — 20% width, button fills the whole column
                <div class="side-col">
                    <button class="side-button" onclick={on_dec.clone()}>{ "←" }</button>
                </div>

                // Center column — take remaining space, center the counter
                <div class="center-col">
                    <div class="center-content">
                        <h1>{ "Hello from Yew!" }</h1>
                        <p>{ "This is a minimal Yew+Trunk starter." }</p>
                        <strong>{ format!("Count: {}", *counter) }</strong>
                    </div>
                </div>

                // Right column — 20% width, button fills the whole column
                <div class="side-col">
                    <button class="side-button" onclick={on_inc.clone()}>{ "→" }</button>
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
