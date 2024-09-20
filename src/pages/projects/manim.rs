use yew::prelude::*;
use yew_bootstrap::component::*;
use yew_bootstrap::util::*;

#[function_component(Manim)]
pub fn manim() -> Html {
    html! {
        <>
        <h1>{ "manim-rs" }</h1>
        <p>{
            "A port of the Manim Python library, for creating mathematical animations,
            into Rust."
        }</p>
        <br />
        <Button style={Color::Primary} url={"https://github.com/rust-club/manim-rs/"}>{
            "Github for manim-rs"
        }</Button>
        <Button style={Color::Secondary} url={"https://www.manim.community/"}>{
            "Manim Community Website" }
        </Button>
        </>
    }
}
