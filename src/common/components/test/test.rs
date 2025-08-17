use leptos::prelude::*;
use leptos_styles::styles;

#[styles("test.scss")]
#[component]
pub fn TestView() -> impl IntoView {
    view! {
        <div class="glass">
            <div class="glass-filter"></div>
            //<div class="glass-overlay"></div>
            <div class="glass-specular"></div>
            <div class="glass-content">
                "Ciaoo"
            </div>
        </div>

        //<!-- SVG FILTER DEFINITION -->
        <svg style="display: none">
            <filter id="lg-dist" x="0%" y="0%" width="100%" height="100%">
            <feTurbulence type="fractalNoise" baseFrequency="0.008 0.008" numOctaves="2" seed="92" result="noise" />
            <feGaussianBlur in="noise" stdDeviation="2" result="blurred" />
            <feDisplacementMap in="SourceGraphic" in2="blurred" scale="70" xChannelSelector="R" yChannelSelector="G" />
            </filter>
        </svg>
    }
}
