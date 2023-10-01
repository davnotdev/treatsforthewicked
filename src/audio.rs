use super::*;
use kira::{
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
};
use std::io::Cursor;

#[derive(SingleResource)]
pub struct Audio(AudioManager, Vec<StaticSoundData>);

impl Audio {
    pub fn new(sounds: &[&[u8]]) -> Self {
        let manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();

        let sounds = sounds
            .iter()
            .map(|sound| {
                let sound = sound.to_vec();
                StaticSoundData::from_cursor(Cursor::new(sound), StaticSoundSettings::default())
                    .unwrap()
            })
            .collect::<Vec<_>>();

        Self(manager, sounds)
    }

    pub fn play(&mut self, idx: usize) {
        self.0.play(self.1[idx].clone()).unwrap();
    }
}
