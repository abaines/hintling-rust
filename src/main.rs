use yew::prelude::*;
use web_sys::console;
use gloo_timers::callback::Interval;
use std::rc::Rc;
use std::cell::RefCell;

#[function_component(App)]
fn app() -> Html {
    let skipped = use_state(|| 0u32);
    let correct = use_state(|| 0u32);
    let timer = use_state(|| 0u32); // Timer in tenths of seconds
    let timer_handle = use_mut_ref(|| None::<Rc<RefCell<u32>>>);

    {
        // Timer that increments every 100ms (tenth of a second)
        let timer = timer.clone();
        let timer_value = Rc::new(RefCell::new(0u32));
        *timer_handle.borrow_mut() = Some(timer_value.clone());
        
        use_effect_with(
            (),
            move |_| {
                let interval = Interval::new(100, move || {
                    let mut val = timer_value.borrow_mut();
                    *val += 1;
                    timer.set(*val);
                });
                
                // Return cleanup function that cancels the interval
                move || drop(interval)
            },
        );
    }

    let on_correct = {
        let correct = correct.clone();
        let skipped = skipped.clone();
        Callback::from(move |_| {
            correct.set(*correct + 1);
            console::log_1(&format!("Skipped: {} | Correct: {}", *skipped, *correct + 1).into());
        })
    };

    let on_skip = {
        let skipped = skipped.clone();
        let correct = correct.clone();
        Callback::from(move |_| {
            skipped.set(*skipped + 1);
            console::log_1(&format!("Skipped: {} | Correct: {}", *skipped + 1, *correct).into());
        })
    };

    html! {
        // Make the app area fill the viewport with zero padding/margin so side buttons can touch edges
        <div class="app-container">

            <div class="row">
                // Left column — Skip button (red)
                <div class="side-col">
                    <button class="side-button button-skip" onclick={on_skip.clone()}>{ "Skip" }</button>
                </div>

                // Center column — split into three vertical sections
                <div class="center-col">
                    // Top section: Timer (in tenths of seconds)
                    <div class="center-top">
                        <div class="timer">{ format!("{:.1}", *timer as f32 / 10.0) }</div>
                    </div>

                    // Middle section: Game content placeholder
                    <div class="center-middle">
                        <div>
                            <h1>{ "Hello from Yew!" }</h1>
                            <p>{ "This is a minimal Yew+Trunk starter." }</p>
                        </div>
                    </div>

                    // Bottom section: Counter
                    <div class="center-bottom">
                        <div class="counter-left">{ format!("Skipped: {}", *skipped) }</div>
                        <div class="counter-right">{ format!("Correct: {}", *correct) }</div>
                    </div>
                </div>

                // Right column — Correct button (green)
                <div class="side-col">
                    <button class="side-button button-correct" onclick={on_correct.clone()}>{ "Correct" }</button>
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
