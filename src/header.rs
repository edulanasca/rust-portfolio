use yew::prelude::*;
use yew_router::prelude::*;
use crate::route::Route;
use crate::edu_logo::*;

struct Section {
    pub href: Route,
    pub text: String,
}

#[function_component(Header)]
pub fn header() -> Html {
    let links: Vec<Section> = vec![
        Section { href: Route::About, text: String::from("About") },
        Section { href: Route::Projects, text: String::from("Projects") },
    ];

    let location = use_location().unwrap();
    //web_sys::console::log_1(&location.path().into_js_result().unwrap());

    html! {
        <div class={classes!("header")}>
            <div>
                {
                    if location.path().ne("/") {
                        html! { <EduLogo/> }
                    } else {
                        html! {}
                    }
                }
            </div>
            <div class={classes!("header")}>
                {
                    links.iter().map(|link| {
                        html! {
                            <Link<Route>
                                key={link.text.clone()}
                                to={link.href.clone()}
                                classes={
                                    if location.path().contains(&link.text.clone().to_lowercase())
                                        { classes!("menu","link") } else { classes!("menu","current") }
                                }
                            >
                                {link.text.clone()}
                            </Link<Route>>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}