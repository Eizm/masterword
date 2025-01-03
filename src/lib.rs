#![recursion_limit = "10000"]
mod home;

use wasm_bindgen::prelude::*;
use wasm_logger;

use yew::prelude::*;
use yew_router::prelude::*;

use rand::prelude::*;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
enum Route {
    #[at("/:id")]
    Game { id: u32 },
    #[at("/")]
    GenerateSeed,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::GenerateSeed => html! { <Index /> },
        Route::Game { id } => html! { <home::Home id={id}/> },
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter basename="/mastermind">
            <main>
                <Switch<Route> render={switch} />
            </main>
        </BrowserRouter>
    }
}

#[function_component]
fn Index() -> Html {
    // let navigator = use_navigator().unwrap();
    // use_effect(move || {
    //     navigator.push(&dbg!(Route::Game {
    //         id: thread_rng().gen(),
    //     }))
    // });

    // let game = use_state(|| 0);
    // {
    //     let game = game.clone();
    //     use_effect(move || game.set(thread_rng().gen()))
    // };

    html! {
        <Redirect<Route> to={Route::Game{ id: thread_rng().gen() } } />
    }
}
