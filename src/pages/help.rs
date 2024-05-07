use yew::{function_component, html, Html};
use yew_icons::{Icon, IconId};

use crate::components::head_bar::HeadBar;

#[function_component(Help)]
pub fn help() -> Html {
    html! {
        <div class="h-full">
            <HeadBar></HeadBar>
            <div class="flex flex-col">
                <h1 class="text-center text-6xl m-0">{ t!("help.title")}</h1>
                <br />

                <div class="flex items-center">
                    <h3 class="text-left text-2xl m-0">{ t!("help.app_icon_title")}</h3>
                    <Icon icon_id={IconId::FeatherPaperclip} class="mx-2 mt-0.5"/>
                </div>
                <div class="mx-3 my-2 text-2x">{ t!("help.app_icon")}</div>
                <br />

                <div class="flex items-center">
                    <h3 class="text-left text-2xl m-0">{ t!("help.open_menu_tray_title")}</h3>
                </div>
                <div class="mx-3 my-2 text-2x">{ t!("help.open_menu_tray")}</div>
                <br />

                <div class="flex items-center">
                    <h3 class="text-left text-2xl m-0">{ t!("help.open_window_title")}</h3>
                </div>
                <div class="mx-3 my-2 text-2x">{ t!("help.open_window")}</div>
                <br />

                <div class="flex items-center">
                    <h3 class="text-left text-2xl m-0">{ t!("help.pause_monitoring_title")}</h3>
                </div>
                <div class="mx-3 my-2 text-2x">{ t!("help.pause_monitoring")}</div>
                <br />

                <div class="flex items-center">
                    <h3 class="text-left text-2xl m-0">{ t!("help.search_title")}</h3>
                </div>
                <div class="mx-3 my-2 text-2x">{ t!("help.search")}</div>
                <br />

                <div class="flex items-center">
                    <h3 class="text-left text-2xl m-0">{ t!("help.pinned_clips_title")}</h3>
                    <Icon icon_id={IconId::BootstrapPinAngle} class="mx-2 mt-0.5"/>
                </div>
                <div class="mx-3 my-2 text-2x">{ t!("help.pinned_clips")}</div>
                <br />

                <div class="flex items-center">
                    <h3 class="text-left text-2xl m-0">{ t!("help.favourites_title")}</h3>
                    <Icon icon_id={IconId::BootstrapHeart} class="mx-2 mt-0.5"/>
                </div>
                <div class="mx-3 my-2 text-2x">{ t!("help.favourites")}</div>
                <br />

                <div class="flex items-center">
                    <h3 class="text-left text-2xl m-0">{ t!("help.auto_delete_duplications_title")}</h3>
                </div>
                <div class="mx-3 my-2 text-2x">{ t!("help.auto_delete_duplications")}</div>
                <br />

                <div class="flex items-center">
                    <h3 class="text-left text-2xl m-0">{ t!("help.clips_per_page_title")}</h3>
                </div>
                <div class="mx-3 my-2 text-2x">{ t!("help.clips_per_page")}</div>
                <br />

                <div class="flex items-center">
                    <h3 class="text-left text-2xl m-0">{ t!("help.max_clip_len_title")}</h3>
                </div>
                <div class="mx-3 my-2 text-2x">{ t!("help.max_clip_len")}</div>
                <br />

                <div class="mx-3 my-2 text-2x">
                    { t!("help.more_info") }
                    <a class="text-blue-500" href="https://github.com/ChloeWKY/CopyClip" target="_blank">{" GitHub"}</a>
                    {"."}
                </div>
            </div>
        </div>
    }
}
