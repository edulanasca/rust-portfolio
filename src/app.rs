use yew::prelude::*;
use yew_router::prelude::*;
use edu_portfolio::*;

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::About => html! { <About/> },
        Route::Projects => html! { <Projects/> },
        Route::NotFound => html! { <h1>{ "Not Found" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch}/>
        </BrowserRouter>
    }
}
