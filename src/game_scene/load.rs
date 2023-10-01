use super::*;

pub const SPRITE_HOUSE: usize = 1;
pub const SPRITE_CANDY: usize = 2;
pub const SPRITE_BASKET: usize = 3;
pub const SPRITE_OURKID: usize = 4;
pub const SPRITE_KID1: usize = 5;
pub const SPRITE_GRANNY: usize = 8;
pub const SPRITE_CRYING: usize = 9;
pub const SPRITE_INPINFO: usize = 10;
pub const SPRITE_TAKEONE: usize = 11;
pub const SPRITE_S0: usize = 12;
pub const SPRITE_SPLUS: usize = 22;
pub const SPRITE_INTRO: usize = 23;
pub const SPRITE_OUTRO: usize = 24;
pub const SPRITE_KIDPUNCH: usize = 25;
pub const SPRITE_SMINUS: usize = 26;

pub fn load_init(galaxy: &Galaxy) {
    let white: [u8; 4] = [255, 255, 255, 255];
    let white = TextureData::from_bytes(&white, 1, 1);

    galaxy.insert_resource(
        RendererInitLoadTextures::single_resource(),
        RendererInitLoadTextures(vec![
            white,
            TextureData::load(include_bytes!("../../assets/house.png")),
            TextureData::load(include_bytes!("../../assets/candy.jpeg")),
            TextureData::load(include_bytes!("../../assets/basket.png")),
            TextureData::load(include_bytes!("../../assets/ourkid.jpeg")),
            TextureData::load(include_bytes!("../../assets/kid1.jpeg")),
            TextureData::load(include_bytes!("../../assets/kid2.jpeg")),
            TextureData::load(include_bytes!("../../assets/kid3.jpeg")),
            TextureData::load(include_bytes!("../../assets/granny.jpeg")),
            TextureData::load(include_bytes!("../../assets/crying.jpeg")),
            TextureData::load(include_bytes!("../../assets/inpinfo.png")),
            TextureData::load(include_bytes!("../../assets/takeone.png")),
            TextureData::load(include_bytes!("../../assets/symbols/0.png")),
            TextureData::load(include_bytes!("../../assets/symbols/1.png")),
            TextureData::load(include_bytes!("../../assets/symbols/2.png")),
            TextureData::load(include_bytes!("../../assets/symbols/3.png")),
            TextureData::load(include_bytes!("../../assets/symbols/4.png")),
            TextureData::load(include_bytes!("../../assets/symbols/5.png")),
            TextureData::load(include_bytes!("../../assets/symbols/6.png")),
            TextureData::load(include_bytes!("../../assets/symbols/7.png")),
            TextureData::load(include_bytes!("../../assets/symbols/8.png")),
            TextureData::load(include_bytes!("../../assets/symbols/9.png")),
            TextureData::load(include_bytes!("../../assets/symbols/plus.png")),
            TextureData::load(include_bytes!("../../assets/intro.png")),
            TextureData::load(include_bytes!("../../assets/outro.png")),
            TextureData::load(include_bytes!("../../assets/kidpunch.jpeg")),
            TextureData::load(include_bytes!("../../assets/symbols/minus.png")),
        ]),
    );

    galaxy.insert_resource(
        Audio::single_resource(),
        Audio::new(&[
            include_bytes!("../../assets/door.mp3"),
            include_bytes!("../../assets/success.mp3"),
            include_bytes!("../../assets/punch.mp3"),
            include_bytes!("../../assets/coin.mp3"),
            include_bytes!("../../assets/no.mp3"),
        ]),
    );
}

pub fn audio_door(galaxy: &Galaxy) {
    galaxy
        .get_mut_resource::<Audio, _>(Audio::single_resource())
        .unwrap()
        .play(0);
}

pub fn audio_success(galaxy: &Galaxy) {
    galaxy
        .get_mut_resource::<Audio, _>(Audio::single_resource())
        .unwrap()
        .play(1);
}

pub fn audio_punch(galaxy: &Galaxy) {
    galaxy
        .get_mut_resource::<Audio, _>(Audio::single_resource())
        .unwrap()
        .play(2);
}

pub fn audio_coin(galaxy: &Galaxy) {
    galaxy
        .get_mut_resource::<Audio, _>(Audio::single_resource())
        .unwrap()
        .play(3);
}

pub fn audio_no(galaxy: &Galaxy) {
    galaxy
        .get_mut_resource::<Audio, _>(Audio::single_resource())
        .unwrap()
        .play(4);
}
