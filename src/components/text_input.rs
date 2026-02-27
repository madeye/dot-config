use leptos::prelude::*;

#[component]
pub fn ConfigText(
    label: String,
    help: String,
    value: Signal<String>,
    on_change: Callback<String>,
) -> impl IntoView {
    view! {
        <div class="config-control config-text">
            <label class="control-label">
                {label}
                <span class="help-icon" data-tooltip=help.clone()>"?"</span>
            </label>
            <input
                type="text"
                class="control-input"
                placeholder=help
                prop:value=value
                on:input=move |ev| {
                    let val = event_target_value(&ev);
                    on_change.run(val);
                }
            />
        </div>
    }
}
