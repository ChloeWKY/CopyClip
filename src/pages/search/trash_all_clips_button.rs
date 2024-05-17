/// invoke delete_clip_from_database
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, Callback, Html, Properties};
use yew_icons::{Icon, IconId};

use crate::invoke::invoke;

use std::rc::Rc;

use super::clip::SearchRes;

#[derive(PartialEq, Properties)]
pub struct TrashAllClipsButtonProps {
    pub search_res: Rc<SearchRes>,
    pub search_res_dispatch: yewdux::prelude::Dispatch<SearchRes>,
}

#[derive(Debug, Serialize)]
struct TrashAllClipsArgs {
    pub ids: Vec<u64>,
}

#[function_component(TrashAllClipsButton)]
pub fn trash_all_clips_button(props: &TrashAllClipsButtonProps) -> Html {
    let search_res = props.search_res.clone();
    let res = search_res.res.lock().unwrap();
    let search_res_dispatch = props.search_res_dispatch.clone();
    // get a vector of all ids in res
    let ids: Vec<u64> = res.iter().map(|x| x.clip.id).collect();

    let trash_all_clip_button_on_click = Callback::from(move |_| {
        let search_res_dispatch = search_res_dispatch.clone();
        let ids = ids.clone();
        spawn_local(async move {
            let args = TrashAllClipsArgs { ids };
            let args = serde_wasm_bindgen::to_value(&args).unwrap();
            invoke("delete_all_clips_from_database", args).await;
            search_res_dispatch.reduce_mut(|state| {
                // delete all clips in the search res
                let mut result = state.res.lock().unwrap();
                result.clear();
                state.rebuild_num += 1;
            });
        });
    });

    html! {
            <button
                class="font-bold w-full"
                onclick={trash_all_clip_button_on_click}
            >
                <Icon icon_id={IconId::BootstrapTrash} class="mx-auto mt-0.5"/>
            </button>
    }
}
