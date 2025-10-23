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
        <div style="font-family: Arial, sans-serif; padding: 16px;">
            <h1>{ "Hello from Yew!" }</h1>
            <p>{ "This is a minimal Yew+Trunk starter." }</p>

            <div style="display: flex; align-items: center; justify-content: space-between; width: 320px; margin-top: 12px;">
                <button onclick={on_dec.clone()}>{ "←" }</button>
                <div style="min-width: 120px; text-align: center;">
                    <strong>{ format!("Count: {}", *counter) }</strong>
                </div>
                <button onclick={on_inc.clone()}>{ "→" }</button>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
