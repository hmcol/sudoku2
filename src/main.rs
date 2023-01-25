use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    let selected_video = use_state(|| None);

    let videos = init_video_list();

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
        <div class={classes!("app")}>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                <VideosList
                    videos={videos}
                    on_click={on_video_select}
                />
            </div>
            { for details }
        </div>
    }
}

fn init_video_list() -> Vec<Video> {
    vec![
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
    ]
}

#[derive(Clone, PartialEq)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

fn build_callback<IN: Clone + 'static, OUT>(f: &Callback<IN>, arg: &IN) -> Callback<OUT> {
    let f = f.clone();
    let arg = arg.clone();

    Callback::from(move |_| f.emit(arg.clone()))
}

#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
    on_click: Callback<Video>,
}

#[function_component(VideosList)]
fn videos_list(props: &VideosListProps) -> Html {
    let videos = &props.videos;
    let on_click = &props.on_click;

    videos
        .iter()
        .map(|video| {
            let on_video_click = build_callback(on_click, video);

            html! {
                <p
                    key={video.id}
                    onclick={on_video_click}
                >
                    { format!("{}: {}", video.speaker, video.title) }
                </p>
            }
        })
        .collect()
}

#[derive(Properties, PartialEq)]
struct VideosDetailsProps {
    video: Video,
}

#[function_component(VideoDetails)]
fn video_details(props: &VideosDetailsProps) -> Html {
    let title = &props.video.title;

    html! {
        <div>
            <h3>{ title }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}
