use leptos::mount::mount_to_body;
use leptos::prelude::*;

use crate::common::components::button::button::{ButtonType, ButtonView};
use crate::common::components::drag::drag::DragView;
use crate::common::components::navbar::navbar::NavbarView;
use crate::common::components::project::project::ProjectView;
use crate::common::components::slider::slider::SliderView;
use crate::common::components::title::title::TitleView;
mod common;

#[component]
fn App() -> impl IntoView {
    view! {
        <div id="stripes"></div>
        <header>
            <NavbarView elevated=true />
        </header>
        <main>
            <section id="welcome">
                <TitleView />
                <div class="actions">
                    <ButtonView button_type=ButtonType::Primary elevated=true>"Primary"</ButtonView>
                    <ButtonView elevated=true>"Default"</ButtonView>
                </div>
            </section>
            <section id="projects">
                <h2>"My Projects"</h2>
                <p>"Keep up with my latest projects."</p>
                <div class="projects-list">
                    <ProjectView/>
                    <ProjectView/>
                    <ProjectView/>
                    <ProjectView/>
                </div>
            </section>
        </main>
    }
}

fn main() {
    mount_to_body(App)
}
