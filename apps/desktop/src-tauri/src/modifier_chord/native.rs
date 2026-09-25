use super::{ChordEdge, ChordReducer, Modifier, PlatformChord};
use rdev::{EventType, Key};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
    mpsc::SyncSender,
};

pub struct ModifierListener {
    enabled: Arc<AtomicBool>,
    started: AtomicBool,
    reducer: Arc<Mutex<ChordReducer>>,
    sender: SyncSender<ChordEdge>,
}

impl ModifierListener {
    pub fn start(sender: SyncSender<ChordEdge>) -> Result<Self, &'static str> {
        let platform = if cfg!(target_os = "macos") {
            PlatformChord::MacOs
        } else {
            PlatformChord::Windows
        };
        let enabled = Arc::new(AtomicBool::new(false));
        let reducer = Arc::new(Mutex::new(ChordReducer::new(platform)));
        Ok(Self {
            enabled,
            started: AtomicBool::new(false),
            reducer,
            sender,
        })
    }

    pub fn set_enabled(&self, enabled: bool) -> Result<(), &'static str> {
        if enabled
            && self
                .started
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
        {
            let callback_enabled = self.enabled.clone();
            let callback_reducer = self.reducer.clone();
            let callback_sender = self.sender.clone();
            if std::thread::Builder::new()
                .name("tro-passive-modifier-listener".into())
                .spawn(move || {
                    let _ = rdev::listen(move |event| {
                        if !callback_enabled.load(Ordering::Acquire) {
                            return;
                        }
                        let (key, pressed) = match event.event_type {
                            EventType::KeyPress(key) => (key, true),
                            EventType::KeyRelease(key) => (key, false),
                            _ => return,
                        };
                        let Some(modifier) = map_key(key) else {
                            return;
                        };
                        if let Ok(mut reducer) = callback_reducer.lock()
                            && let Some(edge) = reducer.update(modifier, pressed)
                        {
                            let _ = callback_sender.try_send(edge);
                        }
                    });
                })
                .is_err()
            {
                self.started.store(false, Ordering::Release);
                return Err("Global modifier listener unavailable.");
            }
        }
        self.enabled.store(enabled, Ordering::Release);
        if !enabled
            && let Ok(mut reducer) = self.reducer.lock()
            && let Some(edge) = reducer.reset()
        {
            let _ = self.sender.try_send(edge);
        }
        Ok(())
    }
}

fn map_key(key: Key) -> Option<Modifier> {
    Some(match key {
        Key::MetaLeft => Modifier::LeftCommand,
        Key::MetaRight => Modifier::RightCommand,
        Key::ControlLeft => Modifier::LeftControl,
        Key::ControlRight => Modifier::RightControl,
        Key::Alt => Modifier::LeftAlt,
        Key::AltGr => Modifier::RightAlt,
        Key::ShiftLeft => Modifier::LeftShift,
        Key::ShiftRight => Modifier::RightShift,
        Key::CapsLock => Modifier::CapsLock,
        _ => return None,
    })
}
