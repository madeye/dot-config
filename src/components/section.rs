use leptos::prelude::*;

#[component]
pub fn ConfigSection(
    title: String,
    children: Children,
) -> impl IntoView {
    let open = RwSignal::new(true);

    view! {
        <div class="config-section">
            <button
                class="section-header"
                on:click=move |_| open.update(|o| *o = !*o)
            >
                <span class="section-arrow" class:open=open>{"\u{25b6}"}</span>
                <h2 class="section-title">{title}</h2>
            </button>
            <div class="section-body" style:display=move || if open.get() { "" } else { "none" }>
                {children()}
            </div>
        </div>
    }
}
