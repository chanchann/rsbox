mod atoms;

use gloo::console::log;
use yew::prelude::*;
use crate::atoms::video::Video;

#[function_component(App)]
pub fn app() -> Html {
    let videos = vec![
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
    // 为了展示这些video，我们需要将Vec转为Html。首先创建一个迭代器，然后通过map将其变为html!，最后再组合成一个列表。
    let videos = videos.iter().map(|video| html! {
        <p>{format!("{}: {}", video.speaker, video.title)}</p>
    }).collect::<Html>();
    // log!("123");
    // html!和标准的HTML有些区别：
    // 1. 表达式必须被大括号包裹（"some string"也是一个表达式，需要将字符串放在引号中）
    // 2. 只能有一个根节点，如果有多个元素，可以设置一个空的根节点(<> elements </>)
    // 3. 标签需要正确的关闭
    /*
    <h1>RustConf Explorer</h1>
    <div>
        <h3>Videos to watch</h3>
        <p>John Doe: Building and breaking things</p>
        <p>Jane Smith: The development process</p>
        <p>Matt Miller: The Web 7.0</p>
        <p>Tom Jerry: Mouseless development</p>
    </div>
    <div>
        <h3>John Doe: Building and breaking things</h3>
        <img
            src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder"
            alt="video thumbnail"
        />
    </div>
     */

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"11 Videos to watch"}</h3>
                <p>{ "11 John Doe: Building and breaking things" }</p>
                <p>{ "11 Jane Smith: The development process" }</p>
                <p>{ "11 Matt Miller: The Web 7.0" }</p>
                <p>{ "11 Tom Jerry: Mouseless development" }</p>
                { videos }
            </div>
            <div>
                <h3>{ "John Doe: Building and breaking things" }</h3>
                <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
            </div>
        </>
    }
}
