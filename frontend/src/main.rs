use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            counter.set(*counter + 1);
        }
    };

    let switch = {
        move |_| {

        }
    };

    html! {
        <div class="flex">
            <button onclick={onclick} class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">{ "+1" }</button>
            <p>{ *counter }</p>
            <button onclick={switch} class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">{ "toggle" }</button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
