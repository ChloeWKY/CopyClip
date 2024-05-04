use yew::{function_component, html, Html};

use crate::components::head_bar::HeadBar;

#[function_component(Help)]
pub fn help() -> Html {
    html! {
        <div class="h-full">
            <HeadBar></HeadBar>
            <div class="flex flex-col">
                <h1 class="text-center text-6xl m-0">{ t!("help.title")}</h1>
                <br />

                <h3 class="text-left text-2xl m-0">{ t!("help.app_icon_title")}</h3>
                <div class="mx-3 my-2 text-2x">{ t!("help.app_icon")}</div>
                <br />

                <h3 class="text-left text-2xl m-0">{ t!("help.open_menu_tray_title")}</h3>
                <div class="mx-3 my-2 text-2x">{ t!("help.open_menu_tray")}</div>
                <br />

                <h3 class="text-left text-2xl m-0">{ t!("help.open_window_title")}</h3>
                <div class="mx-3 my-2 text-2x">{ t!("help.open_window")}</div>
                <br />

                <h3 class="text-left text-2xl m-0">{ t!("help.pause_monitoring_title")}</h3>
                <div class="mx-3 my-2 text-2x">{ t!("help.pause_monitoring")}</div>
                <br />

                <h3 class="text-left text-2xl m-0">{ t!("help.search_title")}</h3>
                <div class="mx-3 my-2 text-2x">{ t!("help.search")}</div>
                <br />

                <h3 class="text-left text-2xl m-0">{ t!("help.pinned_clips_title")}</h3>
                <div class="mx-3 my-2 text-2x">{ t!("help.pinned_clips")}</div>
                <br />

                <h3 class="text-left text-2xl m-0">{ t!("help.favourites_title")}</h3>
                <div class="mx-3 my-2 text-2x">{ t!("help.favourites")}</div>
            </div>
        </div>
    }
}
