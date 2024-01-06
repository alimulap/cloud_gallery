use leptos::*;
use stylers::style_str;

#[component]
pub fn HomePage() -> impl IntoView {
    let (class_name, style_str) = style_str! {
        #home {
            display: flex;
            justify-content: center;
            align-items: center;
            font-size: "1.2em";
            color: white;
            height: 100%;
            background-color: rgba(0, 0, 0, 0);
            z-index: 1;
        }
    };
    view! {
        <div id="home">
            <h1>"Home Page"</h1>
        </div>
    }
}
