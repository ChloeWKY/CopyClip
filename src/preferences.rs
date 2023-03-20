use yew::{function_component, html, Html};

use crate::components::preferences::{
    clips_per_page_config::ClipsPerPageConfig, max_clip_len_config::MaxClipLenConfig,
};

#[function_component(Preferences)]
pub fn preferences() -> Html {
    html! {
        <div class="flex min-h-screen flex-col bg-white">
            <h1 class="text-center text-6xl m-0">{ "Preferences" }</h1>
            <div class="mx-5 my-2">
                <ClipsPerPageConfig></ClipsPerPageConfig>
                <br />
                <MaxClipLenConfig></MaxClipLenConfig>
            </div>
        </div>
    }
}
