/// Yew seems much like an amalgamation of React an Elm, taking styles from each to provide a somewhat
/// familiar an intuitive framework.
use yew::{html, Callback, MouseEvent, Component, ComponentLink, Html, ShouldRender};

/// The state of our application, equivalent to `State` in a React app.
pub struct App {
    clicked: bool,
    onclick: Callback<MouseEvent>,
}

/// The allowed messages/events/actions in the application, equivalent to `Msg` in an
/// Elm app.
pub enum Msg {
    Click,
}

/// The `component` trait of `App` is much like a React class of `React.Component<{}, State>` in JS.
///
/// The component features three main parts:
/// - `create`: Equivalent to our `constructor` in a React class, setting up our initial state.
/// - `update`: Handle all events in the component, a sorta mix between the `update` function in an Elm app,
///             and `shouldComponentUpdate` in React.
/// - `view`  : Our render function.
///
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            clicked: false,
            onclick: link.callback(|_| Msg::Click),
        }
    }

    /// The update function returns a boolean indicating whether a re-render should happen or
    /// not.
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                // Update any internal state.
                self.clicked = true;
                // Indicate that the Component should re-render by returning `true`.
                true
            }
        }
    }

    fn view(&self) -> Html {
        let button_text = if self.clicked {
            "Clicked!"
        } else {
            "Click me!"
        };

        // The `html!` macro allows us to write in a JSX-esque style.
        html! {
            <button onclick=&self.onclick>{ button_text }</button>
        }
    }
}
