import * as twitch_stream_extractor from 'twitch-stream-extractor-js';
import fetch from 'node-fetch';

// Polyfill to make reqwest work for NodeJS
global.Headers = fetch.Headers
global.Request = fetch.Request
global.Response = fetch.Response
global.Window = Object 
global.fetch = fetch

twitch_stream_extractor.stream_playlist('sleepy').then(playlist => {
    console.log(`Channel playlist:\n${JSON.stringify(playlist, null, 2)}`);
}).catch(err => {
    console.error(`An error occurred: ${err}`);
});

twitch_stream_extractor.vod_playlist('562766638').then(playlist => {
    console.log(`VoD playlist:\n${JSON.stringify(playlist, null, 2)}`);
}).catch(err => {
    console.error(`An error occurred: ${err}`);
});
