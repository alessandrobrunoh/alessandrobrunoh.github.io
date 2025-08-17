use leptos::{html, prelude::*};
use leptos_styles::styles;

const INITIAL_VALUE: f64 = 0.0;

#[styles("slider.scss")]
#[component]
pub fn SliderView() -> impl IntoView {
    let (value, set_value) = signal(INITIAL_VALUE);
    let range_ref = NodeRef::<html::Div>::new();
    let input_ref = NodeRef::<html::Input>::new();
    let left_icon_ref = NodeRef::<html::I>::new();
    let right_icon_ref = NodeRef::<html::I>::new();

    let update_fill = move |val: f64, trans: bool| {
        if let Some(range_element) = range_ref.get() {
            let fill_width = val;
            let style = format!("--fill-width: {}%;", fill_width);
            if trans {
                let transition = "transition: width 0.3s ease;";
                range_element
                    .set_attribute("style", &format!("{}{}", &style, &transition))
                    .unwrap_or(());
            } else {
                range_element.set_attribute("style", &style).unwrap_or(());
            }
        }
    };

    let update_icon_animation = move |val: f64| {
        // Rimuovi animazioni precedenti
        if let Some(left_icon) = left_icon_ref.get() {
            let _ = left_icon.class_list().remove_1("ring-animation");
        }
        if let Some(right_icon) = right_icon_ref.get() {
            let _ = right_icon.class_list().remove_1("ring-animation");
        }

        // Aggiungi animazione appropriata
        if val == 0.0 {
            if let Some(left_icon) = left_icon_ref.get() {
                let _ = left_icon.class_list().add_1("ring-animation");
            }
        } else if val == 100.0 {
            if let Some(right_icon) = right_icon_ref.get() {
                let _ = right_icon.class_list().add_1("ring-animation");
            }
        }
    };

    Effect::new(move |_| {
        let val = value.get();
        update_fill(val, false);
        update_icon_animation(val);
    });

    let on_mousedown = move |_| {
        if let Some(input_element) = input_ref.get() {
            let _ = input_element.class_list().remove_1("unfocused");
        }
    };

    let on_mousemove = move |ev| {
        let new_value = event_target_value(&ev).parse::<f64>().unwrap_or(50.0);
        set_value.set(new_value);
    };

    let on_mouseleave = move |_| {
        if let Some(input_element) = input_ref.get() {
            let _ = input_element.class_list().add_1("unfocused");
        }
    };

    let on_mouseup = move |_| {
        if let Some(input_element) = input_ref.get() {
            let _ = input_element.class_list().add_1("unfocused");
        }
    };

    let on_input = move |ev| {
        let new_value = event_target_value(&ev).parse::<f64>().unwrap_or(50.0);
        set_value.set(new_value);
        update_fill(new_value, true);
    };

    view! {
        <div class="slider">
            <i class="fa-regular fa-bell" node_ref=left_icon_ref></i>
            <i class="fa-regular fa-bell" node_ref=right_icon_ref></i>
            <div class="range" node_ref=range_ref style="--fill-width: 50%">
                <input
                    type="range"
                    min="0"
                    max="100"
                    value=move || value.get().to_string()
                    on:mouseup=on_mouseup
                    on:mousedown=on_mousedown
                    on:mousemove=on_mousemove
                    on:mouseleave=on_mouseleave
                    on:input=on_input
                    node_ref=input_ref
                />
            </div>
        </div>
    }
}
