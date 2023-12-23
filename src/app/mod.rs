use leptos::*;
// use stylers::style_str;

pub mod router;
use router::AppRouter;

pub mod welcome;
pub mod home;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <style> { "
            html, body {
                margin: 0;
                padding: 0;
            }
            #app {
                position: fixed;
                top: 0;
                left: 0;
                width: 100%;
                height: 100%;
            }
        " } </style>
        <div id="app">
            <AppRouter />
        </div>
    }
}
