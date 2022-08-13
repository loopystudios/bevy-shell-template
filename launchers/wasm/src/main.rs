use bevy::prelude::*;
use stylist::yew::styled_component;
use stylist::{css, global_style};
use yew::prelude::*;

#[function_component(Root)]
pub fn root() -> Html {
    html! {
        <App />
    }
}

#[styled_component(App)]
fn app() -> Html {
    global_style! {
        r#"
        html {
            min-height: 100%;
            position: relative;
        }
        body {
            height: 100%;
            background-color: black;
            padding: 0;
            margin: 0;
        }
        "#
    }
    .unwrap();

    let style = css! {
        r#"
        & {
            position: absolute;
            top: 0;
            bottom: 0;
            left: 0;
            right: 0;
            overflow: hidden;
            width: 100%;
            height: 100%;
        }
        "#
    };

    html! {
        <div id="ctr" class={ style }>
            <canvas id="bevy"></canvas>
        </div>
    }
}

fn main() {
    // Mount the DOM
    yew::start_app::<Root>();

    // Start the Bevy App
    let mut app = my_game::app();
    info!("Starting launcher: WASM");
    app.run();
}
