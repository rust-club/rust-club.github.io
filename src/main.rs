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
    html! {
        <NavBar nav_id={"navbar"} class="navbar-expand-lg navbar-light bg-light" brand={brand}>
            <NavItem text="About" url={AttrValue::from("/about")} />
            <NavItem text="Projects">
                <NavDropdownItem text="Yew Tutorial" url={AttrValue::from("/projects/yew_tutorial")} />
                <NavDropdownItem text="manim-rs" url={AttrValue::from("/projects/manim-rs")} />
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
        <MyNavBar/>
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
        {include_cdn_js()}
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
