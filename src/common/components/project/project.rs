use leptos::prelude::*;
use leptos_styles::styles;

#[styles("project.scss")]
#[component]
pub fn ProjectView() -> impl IntoView {
    view! {
        <article class="project">
            <img src="https://zed.dev/_next/image?url=%2Fimg%2Fpost%2Fllms-cant-write-code%2Fllms-cant-write-code.webp&w=3840&q=75" alt="Project Image"></img>
            <h3>"Project Title"</h3>
            <p>"Project Description abababa"</p>
            <div class="project-tech">
                <span class="tech">"HTML"</span>
                <span class="tech">"CSS"</span>
                <span class="tech">"JavaScript"</span>
            </div>
            <div class="project-details">
                <div class="author">
                    <img src="https://zed.dev/_next/image?url=https%3A%2F%2Favatars.githubusercontent.com%2Fu%2F94272%3Fv%3D4&w=128&q=75" alt="Author Image"></img>
                    <span>"Alessandro Bruno"</span>
                </div>
                <span>"11 august, 2025"</span>
            </div>
        </article>
    }
}
