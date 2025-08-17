use leptos::html::Div;
use leptos::prelude::*;
use leptos_styles::styles;
use leptos_use::core::Position;
use leptos_use::{UseDraggableOptions, UseDraggableReturn, use_draggable_with_options, use_window};

#[styles("drag.scss")]
#[component]
pub fn DragView() -> impl IntoView {
    let el = NodeRef::<Div>::new();

    // `style` is a helper string "left: {x}px; top: {y}px;"
    let UseDraggableReturn { x, y, style, .. } = use_draggable_with_options(
        el,
        UseDraggableOptions::default().initial_value(Position { x: 600.0, y: 40.0 }),
    );

    view! {
        <div
            node_ref=el
            class="glass drag"
            style=move || format!("position: fixed; {}", style.get())>
                Drag me! I am at { x }, { y }
        </div>
    }
}
