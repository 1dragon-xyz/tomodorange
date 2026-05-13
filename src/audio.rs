use rust_embed::RustEmbed;
use rodio::{Decoder, OutputStream, Sink, source::Source};
use std::io::Cursor;
use std::sync::{Arc, Mutex};

#[derive(RustEmbed)]
#[folder = "assets/audio/"]
struct Asset;

pub struct AudioEngine {
    _stream: OutputStream,
    _handle: rodio::OutputStreamHandle,
    sink: Arc<Mutex<Sink>>,
    // Optimized pre-loaded data
    tick_data: Arc<Vec<u8>>,
    gong_data: Option<Arc<Vec<u8>>>,
    waves_data: Arc<Vec<u8>>,
}

impl AudioEngine {
    pub fn new() -> Option<Self> {
        let (stream, handle) = OutputStream::try_default().ok()?;
        let sink = Arc::new(Mutex::new(Sink::try_new(&handle).ok()?));
        
        // Pre-load all assets into memory once
        let tick_data = Arc::new(Asset::get("water_drop.wav")?.data.into_owned());
        
        // Load OGG Waves instead of MP3
        let waves_data = Arc::new(Asset::get("waves.ogg")?.data.into_owned());
        
        let gong_data = Asset::get("gong.wav").map(|a| Arc::new(a.data.into_owned()));

        Some(Self {
            _stream: stream,
            _handle: handle,
            sink,
            tick_data,
            gong_data,
            waves_data,
        })
    }

    pub fn play_tick(&self) {
        let cursor = Cursor::new(Arc::clone(&self.tick_data).to_vec());
        if let Ok(source) = Decoder::new(cursor) {
            if let Ok(sink) = Sink::try_new(&self._handle) {
                sink.set_volume(0.45);
                sink.append(source);
                sink.detach(); 
            }
        }
    }

    pub fn play_gong(&self) {
        if let Some(data) = &self.gong_data {
            let cursor = Cursor::new(Arc::clone(data).to_vec());
            if let Ok(source) = Decoder::new(cursor) {
                if let Ok(sink) = Sink::try_new(&self._handle) {
                    sink.set_volume(0.7);
                    sink.append(source);
                    sink.detach();
                }
            }
        }
    }

    pub fn start_break_loop(&self) {
        let cursor = Cursor::new(Arc::clone(&self.waves_data).to_vec());
        if let Ok(source) = Decoder::new(cursor) {
            let sink = self.sink.lock().unwrap();
            sink.stop();
            sink.set_volume(0.25);
            // OGG Vorbis is perfect for repeat_infinite()
            sink.append(source.repeat_infinite());
        }
    }

    pub fn stop_break_loop(&self) {
        let sink = self.sink.lock().unwrap();
        sink.stop();
    }
}
