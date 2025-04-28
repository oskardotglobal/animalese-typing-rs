use std::{sync::mpsc::channel, time::Duration};

use crate::{
    input::{Input, Modifier},
    layout::Layout,
    voice::AnimaleseVoice,
};
use awedio::{Sound, backends::CpalBackend, manager::Manager};
use device_query::{DeviceEvents, DeviceEventsHandler, Keycode};
use moka::sync::Cache;

pub struct AnimaleseTyping {
    manager: Manager,
    modifiers: Cache<Modifier, bool>,
    voice: AnimaleseVoice,
    layout: Box<dyn Layout>,

    // Keep these in the struct so that they stay in scope;
    // if they are dropped, the events are deregistered
    event_handler: DeviceEventsHandler,
    _backend: CpalBackend,
}

impl AnimaleseTyping {
    const SCREAMING_VOLUME_FACTOR: f32 = 2.0;

    const ASSET_PATH: &'static str = "assets/audio";

    pub fn new(voice: AnimaleseVoice, layout: Box<dyn Layout>) -> Self {
        let (manager, _backend) = awedio::start().expect("Could not connect to audio backend");
        let event_handler = DeviceEventsHandler::new(Duration::from_millis(10))
            .expect("Could not initialize event loop");

        let modifiers = Cache::new(100);

        Self {
            manager,
            modifiers,
            layout,
            voice,
            event_handler,
            _backend,
        }
    }

    pub fn read_key(&mut self, key: Keycode) {
        self.manager.clear();

        if let Some(input) = self.layout.parse_key_chord(&key, &self.modifiers) {
            let folder = match input {
                Input::Letter(char) => format!("animalese/{0}/{char}.aac", self.voice),
                Input::Number(char) => format!("vocals/{0}/{char}.aac", self.voice),
                Input::Special(name) => format!("sfx/{name}.aac"),
            };

            let volume = if self.modifiers.contains_key(&Modifier::CapsLock)
                ^ self.modifiers.contains_key(&Modifier::Shift)
            {
                Self::SCREAMING_VOLUME_FACTOR
            } else {
                1.0
            };

            self.manager.play(Box::new(
                awedio::sounds::open_file(format!("{0}/{folder}", Self::ASSET_PATH))
                    .expect("Assets should exist")
                    .with_adjustable_volume_of(volume),
            ));
        }
    }

    pub fn run(mut self) -> ! {
        // Cloning the cache is an inexpensive operation,
        // so clone it as often as needed so that the closure
        // does not have to take ownership of `self`

        let (tx, rx) = channel::<Keycode>();
        let modifiers = self.modifiers.clone();

        let _key_up_guard = self.event_handler.on_key_up(move |key| {
            if let Some(modifier) = Modifier::from_keycode(key) {
                match modifier {
                    Modifier::CapsLock => (),
                    _ => modifiers.invalidate(&modifier),
                }
            }
        });

        let modifiers = self.modifiers.clone();

        let _key_down_guard = self.event_handler.on_key_down(move |key| {
            if let Some(modifier) = Modifier::from_keycode(key) {
                return match modifier {
                    Modifier::CapsLock if modifiers.contains_key(&Modifier::CapsLock) => {
                        modifiers.invalidate(&modifier)
                    }
                    _ => modifiers.insert(modifier, true),
                };
            }

            tx.send(*key).unwrap();
        });

        loop {
            let key = rx.recv().unwrap();
            self.read_key(key);
        }
    }
}
