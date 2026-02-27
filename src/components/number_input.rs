use leptos::prelude::*;

#[component]
pub fn ConfigNumber(
    label: String,
    help: String,
    min: i64,
    max: i64,
    step: i64,
    value: Signal<String>,
    on_change: Callback<String>,
) -> impl IntoView {
    view! {
        <div class="config-control config-number">
            <label class="control-label">
                {label}
                <span class="help-icon" data-tooltip=help>"?"</span>
            </label>
            <input
                type="number"
                class="control-input"
                min=min
                max=max
                step=step
                prop:value=value
                on:change=move |ev| {
                    let val = event_target_value(&ev);
                    on_change.run(val);
                }
            />
        </div>
    }
}
