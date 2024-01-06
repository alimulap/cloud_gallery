use leptos::*;
use leptos::leptos_dom::logging::console_log;
use leptos_router::*;
use stylers::style_str;
use web_sys::MouseEvent;

use crate::welcome::Welcome;
use crate::home::HomePage;

#[component]
pub fn AppRouter() -> impl IntoView {
    let (class_name, style_str) = style_str! {
    };
    let on_click = move |event: MouseEvent| {
        let target = event.target().unwrap();
        console_log(target.to_string().as_string().as_ref().unwrap());
    };
    view! { class=class_name,
        <Router>
            <style> { style_str } </style>
            <style> "
                a {
                    color: #ffffff;
                    text-decoration: none;
                    padding: 10px;
                    border-radius: 5px;
                    transition: all 0.2s ease-in-out;
                }

                a:hover {
                    background-color: #ffffff;
                    color: #203620;
                }

                nav {
                    height: 50px;
                    background-color: #203620;
                    color: #ffffff;
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    padding: 10px 20px;
                    box-sizing: border-box;
                    z-index: 100;
                }

                .routes {
                    position: relative;
                    width: 100%;
                    height: 100%;
                    overflow: hidden;
                    transition: all 0.5s ease-in-out;
                }

                .start {
                    transition: none;
                    transform: translateX(-100%);
                    background-color: #ff0000;
                }

                .move-right {
                    transition: all 0.5s ease-in-out;
                    transform: translateX(100%);
                    background-color: #00ff00;
                }
                .fadeIn {
                  animation: 0.5s fadeIn forwards;
                }

                .fadeOut {
                  animation: 0.5s fadeOut forwards;
                }

                @keyframes fadeIn {
                  from {
                    opacity: 0;
                  }

                  to {
                    opacity: 1;
                  }
                }

                @keyframes fadeOut {
                  from {
                    opacity: 1;
                  }

                  to {
                    opacity: 0;
                  }
                }

                .slideIn {
                  animation: 0.25s slideIn forwards;
                }

                .slideOut {
                  animation: 0.25s slideOut forwards;
                }

                @keyframes slideIn {
                  from {
                    transform: translate(100vw, 0);
                  }
                  to {
                    transform: translate(0px, 0px);
                  }
                }

                @keyframes slideOut {
                  from {
                    transform: translate(0px, 0px);
                  }
                  to {
                    transform: translate(-100vw, 0);
                  }
                }

                .slideInBack {
                  animation: 0.25s slideInBack forwards;
                }

                .slideOutBack {
                  animation: 0.25s slideOutBack forwards;
                }

                @keyframes slideInBack {
                  from {
                    transform: translate(-100vw, 0);
                  }

                  to {
                    transform: translate(0px, 0px);
                  }
                }

                @keyframes slideOutBack {
                  from {
                    transform: translate(0px, 0px);
                  }

                  to {
                    transform: translate(100vw, 0);
                  }
                }
            " </style>
            <nav>
                <div><a href="/">"CLOUD GALLERY"</a></div>
                <div>
                    <a on:click=on_click href="/app">"Home"</a>
                    <a on:click=on_click href="/about">"About"</a>
                </div>
                <div><a href="#"></a></div>
            </nav>
            <AnimatedRoutes
                class="routes"
                outro="slideOut"
                intro="slideIn"
                outro_back="slideOutBack"
                intro_back="slideInBack"
            >
                <Route path="/" view=Welcome />
                <Route path="/app" view=HomePage />
                <Route path="/about" view=|| view! { <p>"About!" </p> } />
            </AnimatedRoutes>
        </Router>
    }
}
