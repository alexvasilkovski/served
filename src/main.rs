use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html!
    { <>
            <h1>{ "served" }</h1>
            <p class="text-3xl font-bold underline">{"hello everyone"}</p>
    </> }
}

fn main() {
    yew::Renderer::<App>::new().render();
}