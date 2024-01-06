use leptos::*;
use stylers::style_str;

#[component]
pub fn Welcome() -> impl IntoView {
    let (class_name, style_str) = style_str! {
        #welcome {
            position: relative;
            display: flex;
            justify-content: center;
            align-items: center;
            font-size: "1.2em";
            color: white;
            height: 100%;
            width: 100%;
            opacity: 0.0;
            //background: url("https://wallpapercave.com/wp/nmwcuAD.jpg");
            z-index: 1;
        }

        #welcome h1 {
            position: relative;
            z-index: 1;
        }

        #background {
            position: absolute;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            object-fit: cover;
            z-index: -1;
        }

        .app-name {
            color: #ffffff;
            background-color: #afafaf96;
            border-radius: 5px;
            padding: 0 5px;
        }
    };
    view! { class=class_name,
        <style> { style_str } </style>
        <img id="background" src="https://wallpapercave.com/wp/nmwcuAD.jpg" />
        <div id="welcome">
            <h1>"WELCOME TO THE "<span class="app-name">"CLOUD GALLERY"</span></h1>
        </div>
    }
}
