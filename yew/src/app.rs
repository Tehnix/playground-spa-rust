use yew::html;
use yew_functional::function_component;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <h1>{"Hello world!"}</h1>
            <div>
                {"Invalid stuff"}
            </div>
        </>
    }
}
