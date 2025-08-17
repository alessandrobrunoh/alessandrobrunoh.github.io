use leptos::prelude::*;
use leptos_styles::styles;

use crate::common::components::navbar::navbar_item::NavbarItemView;

#[styles("navbar.scss")]
#[component]
pub fn NavbarView(
    #[prop(default = false)] elevated: bool,
    #[prop(default = false)] frozen: bool,
) -> impl IntoView {
    view! {
        <nav class="glass">
            <div class="glass-filter"></div>

            <Show when={move || frozen} fallback=move || ().into_view()>
                <div class="glass-overlay"></div>
            </Show>

            <div class="glass-specular"></div>
            <div class="glass-content">
                <ul>
                    <NavbarItemView />
                    <NavbarItemView />
                </ul>
            </div>
        </nav>
    }
}
