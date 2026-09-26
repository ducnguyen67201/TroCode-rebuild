use super::{ChordEdge, ChordReducer, Modifier, PlatformChord};
#[cfg(target_os = "macos")]
use core_graphics::event::{
    CGEventTap, CGEventTapLocation, CGEventTapOptions, CGEventTapPlacement, CGEventType,
    CallbackResult, EventField,
};
#[cfg(target_os = "windows")]
use rdev::{EventType, Key};
#[cfg(target_os = "macos")]
use std::collections::BTreeSet;
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
                    let _ = listen_for_modifiers(move |modifier, pressed| {
                        if !callback_enabled.load(Ordering::Acquire) {
                            return;
                        }
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

#[cfg(target_os = "macos")]
fn listen_for_modifiers(
    on_modifier: impl Fn(Modifier, bool) + 'static,
) -> Result<(), &'static str> {
    let down = Mutex::new(BTreeSet::new());
    CGEventTap::with_enabled(
        CGEventTapLocation::HID,
        CGEventTapPlacement::HeadInsertEventTap,
        CGEventTapOptions::ListenOnly,
        vec![CGEventType::FlagsChanged],
        move |_proxy, event_type, event| {
            if !matches!(event_type, CGEventType::FlagsChanged) {
                return CallbackResult::Keep;
            }
            let keycode = event.get_integer_value_field(EventField::KEYBOARD_EVENT_KEYCODE) as u16;
            let Some(modifier) = map_macos_keycode(keycode) else {
                return CallbackResult::Keep;
            };
            let Ok(mut down) = down.lock() else {
                return CallbackResult::Keep;
            };
            let pressed = if down.insert(keycode) {
                true
            } else {
                down.remove(&keycode);
                false
            };
            drop(down);
            on_modifier(modifier, pressed);
            CallbackResult::Keep
        },
        || unsafe { core_foundation_sys::runloop::CFRunLoopRun() },
    )
    .map_err(|_| "Global modifier listener unavailable.")
}

#[cfg(target_os = "macos")]
fn map_macos_keycode(keycode: u16) -> Option<Modifier> {
    Some(match keycode {
        0x37 => Modifier::LeftCommand,
        0x36 => Modifier::RightCommand,
        0x3b => Modifier::LeftControl,
        0x3e => Modifier::RightControl,
        0x3a => Modifier::LeftAlt,
        0x3d => Modifier::RightAlt,
        0x38 => Modifier::LeftShift,
        0x3c => Modifier::RightShift,
        0x39 => Modifier::CapsLock,
        _ => return None,
    })
}

#[cfg(target_os = "windows")]
fn listen_for_modifiers(
    on_modifier: impl Fn(Modifier, bool) + Send + 'static,
) -> Result<(), &'static str> {
    rdev::listen(move |event| {
        let (key, pressed) = match event.event_type {
            EventType::KeyPress(key) => (key, true),
            EventType::KeyRelease(key) => (key, false),
            _ => return,
        };
        if let Some(modifier) = map_windows_key(key) {
            on_modifier(modifier, pressed);
        }
    })
    .map_err(|_| "Global modifier listener unavailable.")
}

#[cfg(target_os = "windows")]
fn map_windows_key(key: Key) -> Option<Modifier> {
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

#[cfg(all(test, target_os = "macos"))]
mod tests {
    use super::*;

    #[test]
    fn macos_modifier_keycodes_are_mapped_without_text_translation() {
        assert_eq!(map_macos_keycode(0x37), Some(Modifier::LeftCommand));
        assert_eq!(map_macos_keycode(0x36), Some(Modifier::RightCommand));
        assert_eq!(map_macos_keycode(0x3b), Some(Modifier::LeftControl));
        assert_eq!(map_macos_keycode(0x3e), Some(Modifier::RightControl));
        assert_eq!(map_macos_keycode(0x3a), Some(Modifier::LeftAlt));
        assert_eq!(map_macos_keycode(0x3d), Some(Modifier::RightAlt));
        assert_eq!(map_macos_keycode(0x38), Some(Modifier::LeftShift));
        assert_eq!(map_macos_keycode(0x3c), Some(Modifier::RightShift));
        assert_eq!(map_macos_keycode(0x39), Some(Modifier::CapsLock));
        assert_eq!(map_macos_keycode(0x00), None);
    }
}
