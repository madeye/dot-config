use leptos::prelude::*;

#[component]
pub fn ConfigDropdown(
    label: String,
    help: String,
    choices: Vec<(String, String)>,
    value: Signal<String>,
    on_change: Callback<String>,
) -> impl IntoView {
    view! {
        <div class="config-control config-dropdown">
            <label class="control-label">
                {label}
                <span class="help-icon" data-tooltip=help>"?"</span>
            </label>
            <select
                class="control-input"
                on:change=move |ev| {
                    let val = event_target_value(&ev);
                    on_change.run(val);
                }
                prop:value=value
            >
                {choices.into_iter().map(|(val, display)| {
                    let val_clone = val.clone();
                    view! {
                        <option value=val selected=move || value.get() == val_clone>
                            {display}
                        </option>
                    }
                }).collect_view()}
            </select>
        </div>
    }
}
