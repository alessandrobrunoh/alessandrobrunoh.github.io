use leptos::prelude::*;
use leptos_styles::styles;

#[styles("navbar.scss")]
#[component]
pub fn NavbarItemView(
    #[prop(default = false)] elevated: bool,
    #[prop(default = false)] frozen: bool,
) -> impl IntoView {
    view! {
        <li>
            "CIao"
        </li>
    }
}
