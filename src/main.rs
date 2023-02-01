use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
    <>
            <h1>{ "served" }</h1>
            <p class=classes!("bg-red-100")>{"Test!"}</p>
            <p>{"A lightweight, containerized server management utility running on rust wasm."}</p>
            <button type="button" href="">{"Get Started"}</button>
            <button type="button">{"View Source"}</button>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}