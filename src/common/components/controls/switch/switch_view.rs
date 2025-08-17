use leptos::prelude::*;
use leptos_styles::styles;

#[styles("switch.scss")]
#[component]
pub fn SwitchView(#[prop(default = false)] active: bool, children: Children) -> impl IntoView {
    let (is_active, set_active) = signal(active);
    let on_toggle = Some(true);

    let handle_click = move |_| {
        let new_state = !is_active.get();
        set_active.set(new_state);
    };

    view! {
        <div
            class=move || format!("switch{}", if is_active.get() { " active" } else { "" })
            on:click=handle_click
        >
            {children()}
        </div>
    }
}
