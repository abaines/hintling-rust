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

            <div style="display: flex; width: 100%; height: 64vh; margin-top: 12px;">
                // Left column — 20% width, button fills the whole column
                <div style="width: 20%; display: flex; align-items: center; justify-content: center;">
                    <button style="width: 100%; height: 100%; font-size: 24px; padding: 12px 16px; display:flex; align-items:center; justify-content:center;" onclick={on_dec.clone()}>{ "←" }</button>
                </div>

                // Center column — take remaining space, center the counter
                <div style="flex: 1; display: flex; align-items: center; justify-content: center;">
                    <div style="text-align: center;">
                        <strong style="font-size: 20px;">{ format!("Count: {}", *counter) }</strong>
                    </div>
                </div>

                // Right column — 20% width, button fills the whole column
                <div style="width: 20%; display: flex; align-items: center; justify-content: center;">
                    <button style="width: 100%; height: 100%; font-size: 24px; padding: 12px 16px; display:flex; align-items:center; justify-content:center;" onclick={on_inc.clone()}>{ "→" }</button>
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
