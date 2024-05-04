use yew::{function_component, html, use_state, Callback, Html};

use crate::components::{
    head_bar::HeadBar,
    preferences::{
        clips_per_page_config::ClipsPerPageConfig,
        clips_search_per_batch::SearchClipPerBatchConfig, dark_mode_switch::DarkModeSwitch,
        export_button::ExportButton, language_config::LanguagesConfig,
        log_level_filter_config::LogLevelFilterConfig, max_clip_len_config::MaxClipLenConfig,
        set_auto_delete_duplications::AutoDeleteDuplications,
    },
};

#[function_component(Preferences)]
pub fn preferences() -> Html {
    let show_advanced = use_state(|| false);
    let show_advanced_clone = show_advanced.clone();

    html! {
        <div class="flex min-h-screen flex-col">
            <HeadBar></HeadBar>
            <h1 class="text-center text-6xl m-0">{ t!("preferences.title") }</h1>
            <div class="mx-5 my-2">
                <ClipsPerPageConfig></ClipsPerPageConfig>
                <br />
                <MaxClipLenConfig></MaxClipLenConfig>
                <br />
                <DarkModeSwitch></DarkModeSwitch>
                <br />
                <AutoDeleteDuplications></AutoDeleteDuplications>
                <br />
                <LanguagesConfig></LanguagesConfig>
            </div>
            <a class="cursor-pointer text-center text-gray-300 block" 
                onclick={Callback::from(move |_| show_advanced_clone.set(!*show_advanced_clone))}>
                { t!("preferences.show_advanced") }</a>            
            {
                if *show_advanced {
                    html! {
                        <div class="mx-5 my-2">
                            <ExportButton></ExportButton>
                            <br />
                            <LogLevelFilterConfig></LogLevelFilterConfig>
                            <br />
                            <SearchClipPerBatchConfig></SearchClipPerBatchConfig>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
