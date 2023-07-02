use yew::prelude::*;
use yew_router::prelude::Link;
use crate::Route;

#[function_component(EduLogo)]
pub fn edu_logo() -> Html {
    html! {
        <h2 class={classes!("heading", "yellow-purple")}>
            <Link<Route> to={Route::Home}>{"EL"}</Link<Route>>
        </h2>
    }
}