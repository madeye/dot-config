use leptos::prelude::*;

#[component]
pub fn ConfigToggle(
    label: String,
    help: String,
    value: Signal<String>,
    on_change: Callback<String>,
) -> impl IntoView {
    let checked = Signal::derive(move || value.get() == "true");

    view! {
        <div class="config-control config-toggle">
            <label class="toggle-label">
                <input
                    type="checkbox"
                    class="toggle-input"
                    prop:checked=checked
                    on:change=move |ev| {
                        let is_checked = event_target_checked(&ev);
                        on_change.run(if is_checked { "true".into() } else { "false".into() });
                    }
                />
                <span class="toggle-switch"></span>
                <span class="toggle-text">
                    {label}
                    <span class="help-icon" data-tooltip=help>"?"</span>
                </span>
            </label>
        </div>
    }
}
