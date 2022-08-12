use log::info;
use yew::prelude::*;

#[function_component(Root)]
pub fn root() -> Html {
    html! {
        <App />
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div align="center">
            <h1>
                { "TODO: Canvas here" }
            </h1>
        </div>
    }
}

fn main() {
    #[cfg(debug_assertions)]
    {
        // Initialize log and panics to forward to browser log if debug mode
        console_log::init_with_level(log::Level::Trace)
            .expect("Failed to initialise log!");
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }

    yew::start_app::<Root>();
    info!("Starting...");
    my_game::app().run();
}
