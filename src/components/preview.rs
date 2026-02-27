use leptos::prelude::*;

#[component]
pub fn FilePreview(content: Memo<String>) -> impl IntoView {
    view! {
        <div class="file-preview">
            <div class="preview-header">"Preview"</div>
            <pre class="preview-content"><code>{content}</code></pre>
        </div>
    }
}
