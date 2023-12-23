use leptos::*;

mod app;
use app::*;

fn main() {
    mount_to_body(|| view! { <App /> })
}
