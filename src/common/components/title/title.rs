use leptos::prelude::*;
use leptos_styles::styles;

#[styles("title.scss")]
#[component]
pub fn TitleView() -> impl IntoView {
    view! {
        <h1>"Alessandro Bruno"</h1>
        <p>"A paragraph were i have to write some text,"<br/>" and this is the end of the paragraph."</p>
    }
}
