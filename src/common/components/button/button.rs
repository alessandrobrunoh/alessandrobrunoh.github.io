use leptos::component;

use leptos::prelude::*;
use leptos_styles::styles;

#[styles("button.scss")]
#[component]
pub fn ButtonView(
    children: Children,
    #[prop(default = false)] elevated: bool,
    #[prop(default = ButtonType::Default)] button_type: ButtonType,
) -> impl IntoView {
    view! {
        <button class=get_button_class(elevated, &button_type)>
              {children()}
        </button>
    }
}

pub enum ButtonType {
    Default,
    Primary,
    Secondary,
    Tertiary,
}

fn get_button_class(elevated: bool, button_type: &ButtonType) -> String {
    let mut classes = vec!["glass"];

    if elevated {
        classes.push("elevated");
    }

    match button_type {
        ButtonType::Primary => classes.push("primary"),
        ButtonType::Secondary => classes.push("secondary"),
        ButtonType::Tertiary => classes.push("tertiary"),
        ButtonType::Default => {}
    }

    classes.join(" ")
}
