use {
    js_sys::Promise,
    std::collections::HashMap,
    twitch_stream_extractor::{
        hls_m3u8::{tags::VariantStream, MasterPlaylist},
        Extractor,
    },
    wasm_bindgen::prelude::*,
    wasm_bindgen_futures::future_to_promise,
};

macro_rules! playlist_to_output {
    ($playlist:expr) => {{
        $playlist
            .map(playlist_to_output)
            .map(|output| JsValue::from_serde(&output).unwrap())
            .map_err(|err| JsValue::from_str(&format!("{:?}", err)))
    }};
}

fn playlist_to_output(playlist: MasterPlaylist<'_>) -> HashMap<String, String> {
    let mut output = HashMap::new();

    for stream in playlist.video_streams() {
        if let VariantStream::ExtXStreamInf { uri, .. } = stream {
            let quality = stream.video().unwrap().to_string();
            let uri = uri.to_string();

            output.insert(quality, uri);
        }
    }

    output
}

#[wasm_bindgen]
pub fn stream_playlist(channel_name: JsValue) -> Promise {
    let channel_name = channel_name.as_string().unwrap();

    future_to_promise(async move {
        let extractor = Extractor::reqwest();

        playlist_to_output!(extractor.stream(channel_name.as_str()).await)
    })
}

#[wasm_bindgen]
pub fn vod_playlist(vod_id: JsValue) -> Promise {
    let vod_id = vod_id.as_string().unwrap();

    future_to_promise(async move {
        let extractor = Extractor::reqwest();

        playlist_to_output!(extractor.vod(vod_id.as_str()).await)
    })
}
