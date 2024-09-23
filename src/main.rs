use yew::prelude::*;
use yew_bootstrap::component::*;
use yew_bootstrap::util::*;
use yew_router::prelude::*;

// Pages
mod pages;
use pages::projects::*;
use pages::*;

#[function_component(MyNavBar)]
fn my_nav_bar() -> Html {
    let brand = BrandType::BrandCombined {
        text: AttrValue::from("Rust Club"),
        url: Some(AttrValue::from("/")),
        image_url: AttrValue::from("/rust_club_icon.png"),
        alt: AttrValue::from("Rust Club Icon"),
        dimension: Some(Dimension {
            width: String::from("80"),
            height: String::from("80"),
        }),
    };
    let navigator = use_navigator().unwrap();

    let about = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::About));
        html! {
            <NavItem text="About" {onclick} />
        }
    };

    let yew_tutorial = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::YewTutorial));
        html! {
            <NavDropdownItem text="Yew Tutorial" {onclick} />
        }
    };

    let manim_rs = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Manim));
        html! {
            <NavDropdownItem text="manim-rs" {onclick} />
        }
    };

    html! {
        <NavBar nav_id={"navbar"} class="navbar-expand-lg navbar-light bg-light" brand={brand}>
            {about}
            <NavItem text="Projects">
                {yew_tutorial}
                {manim_rs}
            </NavItem>
        </NavBar>
    }
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/projects/yew_tutorial")]
    YewTutorial,
    #[at("/projects/manim-rs")]
    Manim,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::YewTutorial => html! { <YewTutorial /> },
        Route::Manim => html! { <Manim /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        {include_inline()}
        <BrowserRouter>
            <MyNavBar/>
            <Switch<Route> render={switch} />
        </BrowserRouter>
        {include_cdn_js()}
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
