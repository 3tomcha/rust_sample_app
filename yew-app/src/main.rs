use yew::prelude::*;

#[function_component(App)]
fn app() -> Html { 
    html! {
        <div>
            <div data-key="abc"></div>
            <div class="parent">
                <span class="child" value="anything"></span>
                <label for="first-name">{ "First Name" }</label>
                <input type="text" id="first-name" value="placeholder" />
                <input type="checkbox" checked=true />
                <textarea value="write a story" />
                <select name="status">
                    <option selected=true disabled=false value="">{ "Selected" }</option>
                    <option selected=false disabled=true value="">{ "Unselected" }</option>
                </select>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}