use yew::prelude::*;
use yew::{classes, html};

#[function_component(App)]
fn app() -> Html { 
    let header_text = "HelloWorld".to_string();
    let header_html: Html = html! {
        <h1>{header_text}</h1>
    };
    let count: usize = 5;
    let counter_html: Html = html! {
        <p>{"My age is "}{count}</p>
    };
    let combined_html: Html = html! {
        <div>{header_html}{counter_html}</div>
    };
    html! {
        <div>
            {combined_html}
            <div class={classes!("container")}></div>
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