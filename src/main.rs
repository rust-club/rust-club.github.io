use serde::Deserialize;
use yew::prelude::*;
use yew_bootstrap::component::*;
use yew_bootstrap::util::*;
//use yew_bootstrap::component::{BrandType, NavBar, NavDropdownItem, NavItem};

#[derive(Clone, PartialEq, Deserialize)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[derive(Properties, PartialEq)]
struct VideosDetailsProps {
    video: Video,
}

#[function_component(VideoDetails)]
fn video_details(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    let mut embed_url = String::from("https://youtube.com/embed/");
    embed_url.push_str(video.url.rsplit_once('/').unwrap().1);
    html! {
        <div>
            <h3>{ video.title.clone() }</h3>
            <iframe width="640" height="360" src={embed_url} alt="video thumbnail" />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
    on_click: Callback<Video>,
}

#[function_component(VideosList)]
fn videos_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
    videos
        .iter()
        .map(|video| {
            let on_video_select = {
                let on_click = on_click.clone();
                let video = video.clone();
                Callback::from(move |_| {
                    on_click.emit(video.clone())
                })
            };
            html! {
                <p key={video.id} onclick={on_video_select}>{format!("{}: {}", video.speaker, video.title)}</p>
            }
        })
        .collect()
}

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
            <NavItem text="Home" url={AttrValue::from("/")} />
            <NavItem text="more">
                <NavDropdownItem text="Icon" url={AttrValue::from("/rust_club_icon.png")} />
            </NavItem>
        </NavBar>
    }
}

#[function_component(App)]
fn app() -> Html {
    let videos = use_state(Vec::new);
    {
        let videos = videos.clone();
        use_effect_with((), move |_| {
            let videos = videos.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_videos = vec![
                    Video {
                        id: 1,
                        title: "Building and breaking things".to_string(),
                        speaker: "John Doe".to_string(),
                        url: "https://youtu.be/PsaFVLr8t4E".to_string(),
                    },
                    Video {
                        id: 2,
                        title: "The development process".to_string(),
                        speaker: "Jane Smith".to_string(),
                        url: "https://youtu.be/PsaFVLr8t4E".to_string(),
                    },
                    Video {
                        id: 3,
                        title: "The Web 7.0".to_string(),
                        speaker: "Matt Miller".to_string(),
                        url: "https://youtu.be/PsaFVLr8t4E".to_string(),
                    },
                    Video {
                        id: 4,
                        title: "Mouseless development".to_string(),
                        speaker: "Tom Jerry".to_string(),
                        url: "https://youtu.be/PsaFVLr8t4E".to_string(),
                    },
                ];
                videos.set(fetched_videos);
            });
            || ()
        });
    }
    let selected_video = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| selected_video.set(Some(video)))
    };
    let details = selected_video.as_ref().map(|video| {
        html! {
            <VideoDetails video={video.clone()} />
        }
    });
    html! {
        <>
            {include_inline()}
            <MyNavBar/>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{ "Videos to watch" }</h3>
                <VideosList videos={(*videos).clone()} on_click={on_video_select.clone()} />
            </div>
            { for details }
            {include_cdn_js()}
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
