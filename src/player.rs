use std::{io::Cursor, sync::Arc, thread};

use rodio::OutputStreamBuilder;

#[derive(Debug)]
pub struct Player {
    connected_sound: Arc<[u8]>,
    disconnected_sound: Arc<[u8]>,
}

impl Player {
    pub fn new(plugged_sound: Vec<u8>, unplugged_sound: Vec<u8>) -> anyhow::Result<Self> {
        Ok(Self {
            connected_sound: Arc::from(plugged_sound),
            disconnected_sound: Arc::from(unplugged_sound),
        })
    }

    pub fn play_plugged(&self) {
        self.spawn_playback(Arc::clone(&self.connected_sound));
    }

    pub fn play_unplugged(&self) {
        self.spawn_playback(Arc::clone(&self.disconnected_sound));
    }

    fn spawn_playback(&self, data: Arc<[u8]>) {
        thread::spawn(move || {
            let stream_handle = OutputStreamBuilder::open_default_stream().unwrap();
            let input = Cursor::new(data);
            let sink = rodio::play(stream_handle.mixer(), input).unwrap();
            sink.sleep_until_end();
        });
    }
}
