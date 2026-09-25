use std::collections::BTreeSet;
use uuid::Uuid;

#[cfg(any(target_os = "macos", target_os = "windows"))]
mod native;
#[cfg(any(target_os = "macos", target_os = "windows"))]
pub use native::ModifierListener;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Modifier {
    LeftCommand,
    RightCommand,
    LeftControl,
    RightControl,
    LeftAlt,
    RightAlt,
    LeftShift,
    RightShift,
    LeftWindows,
    RightWindows,
    CapsLock,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlatformChord {
    MacOs,
    Windows,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ChordEdge {
    Pressed(Uuid),
    Released,
}

#[derive(Debug)]
pub struct ChordReducer {
    platform: PlatformChord,
    down: BTreeSet<Modifier>,
    active: bool,
}

impl ChordReducer {
    pub fn new(platform: PlatformChord) -> Self {
        Self {
            platform,
            down: BTreeSet::new(),
            active: false,
        }
    }

    pub fn update(&mut self, modifier: Modifier, pressed: bool) -> Option<ChordEdge> {
        if pressed {
            self.down.insert(modifier);
        } else {
            self.down.remove(&modifier);
        }
        let matches = self.matches_exact_chord();
        match (self.active, matches) {
            (false, true) => {
                self.active = true;
                Some(ChordEdge::Pressed(Uuid::new_v4()))
            }
            (true, false) => {
                self.active = false;
                Some(ChordEdge::Released)
            }
            _ => None,
        }
    }

    pub fn reset(&mut self) -> Option<ChordEdge> {
        self.down.clear();
        if std::mem::take(&mut self.active) {
            Some(ChordEdge::Released)
        } else {
            None
        }
    }

    fn matches_exact_chord(&self) -> bool {
        let mut non_locking = self
            .down
            .iter()
            .copied()
            .filter(|key| *key != Modifier::CapsLock);
        match self.platform {
            PlatformChord::MacOs => {
                let keys: Vec<_> = non_locking.by_ref().collect();
                keys.len() == 2
                    && keys
                        .iter()
                        .any(|key| matches!(key, Modifier::LeftCommand | Modifier::RightCommand))
                    && keys
                        .iter()
                        .any(|key| matches!(key, Modifier::LeftControl | Modifier::RightControl))
            }
            PlatformChord::Windows => non_locking.eq([Modifier::LeftControl, Modifier::LeftAlt]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mac_accepts_either_side_and_releases_once() {
        let mut reducer = ChordReducer::new(PlatformChord::MacOs);
        assert_eq!(reducer.update(Modifier::RightControl, true), None);
        assert!(matches!(
            reducer.update(Modifier::LeftCommand, true),
            Some(ChordEdge::Pressed(_))
        ));
        assert_eq!(reducer.update(Modifier::LeftCommand, true), None);
        assert_eq!(
            reducer.update(Modifier::RightControl, false),
            Some(ChordEdge::Released)
        );
        assert_eq!(reducer.update(Modifier::LeftCommand, false), None);
    }

    #[test]
    fn extra_modifiers_cancel_and_caps_lock_does_not() {
        let mut reducer = ChordReducer::new(PlatformChord::MacOs);
        reducer.update(Modifier::CapsLock, true);
        reducer.update(Modifier::LeftControl, true);
        assert!(matches!(
            reducer.update(Modifier::RightCommand, true),
            Some(ChordEdge::Pressed(_))
        ));
        assert_eq!(
            reducer.update(Modifier::LeftShift, true),
            Some(ChordEdge::Released)
        );
    }

    #[test]
    fn windows_rejects_altgr_and_right_side_variants() {
        let mut reducer = ChordReducer::new(PlatformChord::Windows);
        reducer.update(Modifier::LeftControl, true);
        assert_eq!(reducer.update(Modifier::RightAlt, true), None);
        reducer.reset();
        reducer.update(Modifier::LeftControl, true);
        assert!(matches!(
            reducer.update(Modifier::LeftAlt, true),
            Some(ChordEdge::Pressed(_))
        ));
    }

    #[test]
    fn reset_active_emits_one_release() {
        let mut reducer = ChordReducer::new(PlatformChord::Windows);
        reducer.update(Modifier::LeftControl, true);
        reducer.update(Modifier::LeftAlt, true);
        assert_eq!(reducer.reset(), Some(ChordEdge::Released));
        assert_eq!(reducer.reset(), None);
    }
}
