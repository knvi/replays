#[derive(PartialEq, Eq, Debug)]
pub enum GameMode {
    STD = 0,
    TAIKO = 1,
    CTB = 2,
    MANIA = 3,
}

impl GameMode {
    pub fn from_u8(mode: u8) -> GameMode {
        match mode {
            0 => GameMode::STD,
            1 => GameMode::TAIKO,
            2 => GameMode::CTB,
            3 => GameMode::MANIA,
            _ => panic!("Invalid game mode"),
        }
    }
}

#[derive(Debug)]
pub enum Mod {
    NoFail = 1 << 0,
    Easy = 1 << 1,
    TouchDevice = 1 << 2,
    Hidden = 1 << 3,
    HardRock = 1 << 4,
    SuddenDeath = 1 << 5,
    DoubleTime = 1 << 6,
    Relax = 1 << 7,
    HalfTime = 1 << 8,
    Nightcore = 1 << 9,
    Flashlight = 1 << 10,
    Autoplay = 1 << 11,
    SpunOut = 1 << 12,
    Relax2 = 1 << 13,
    Perfect = 1 << 14,
    Key4 = 1 << 15,
    Key5 = 1 << 16,
    Key6 = 1 << 17,
    Key7 = 1 << 18,
    Key8 = 1 << 19,
    FadeIn = 1 << 20,
    Random = 1 << 21,
    Cinema = 1 << 22,
    Target = 1 << 23,
    Key9 = 1 << 24,
    KeyCoop = 1 << 25,
    Key1 = 1 << 26,
    Key3 = 1 << 27,
    Key2 = 1 << 28,
    ScoreV2 = 1 << 29,
    Mirror = 1 << 30,
}

#[derive(Debug, Default)]
pub struct Mods {
    pub mods: Vec<Mod>,
}

impl Mods {
    pub fn from_u32(mods: u32) -> Mods {
        let mut mods_vec = Vec::new();
        if mods & Mod::NoFail as u32 != 0 {
            mods_vec.push(Mod::NoFail);
        }
        if mods & Mod::Easy as u32 != 0 {
            mods_vec.push(Mod::Easy);
        }
        if mods & Mod::TouchDevice as u32 != 0 {
            mods_vec.push(Mod::TouchDevice);
        }
        if mods & Mod::Hidden as u32 != 0 {
            mods_vec.push(Mod::Hidden);
        }
        if mods & Mod::HardRock as u32 != 0 {
            mods_vec.push(Mod::HardRock);
        }
        if mods & Mod::SuddenDeath as u32 != 0 {
            mods_vec.push(Mod::SuddenDeath);
        }
        if mods & Mod::DoubleTime as u32 != 0 {
            mods_vec.push(Mod::DoubleTime);
        }
        if mods & Mod::Relax as u32 != 0 {
            mods_vec.push(Mod::Relax);
        }
        if mods & Mod::HalfTime as u32 != 0 {
            mods_vec.push(Mod::HalfTime);
        }
        if mods & Mod::Nightcore as u32 != 0 {
            mods_vec.push(Mod::Nightcore);
        }
        if mods & Mod::Flashlight as u32 != 0 {
            mods_vec.push(Mod::Flashlight);
        }
        if mods & Mod::Autoplay as u32 != 0 {
            mods_vec.push(Mod::Autoplay);
        }
        if mods & Mod::SpunOut as u32 != 0 {
            mods_vec.push(Mod::SpunOut);
        }
        if mods & Mod::Relax2 as u32 != 0 {
            mods_vec.push(Mod::Relax2);
        }
        if mods & Mod::Perfect as u32 != 0 {
            mods_vec.push(Mod::Perfect);
        }
        if mods & Mod::Key4 as u32 != 0 {
            mods_vec.push(Mod::Key4);
        }
        if mods & Mod::Key5 as u32 != 0 {
            mods_vec.push(Mod::Key5);
        }
        if mods & Mod::Key6 as u32 != 0 {
            mods_vec.push(Mod::Key6);
        }
        if mods & Mod::Key7 as u32 != 0 {
            mods_vec.push(Mod::Key7);
        }
        if mods & Mod::Key8 as u32 != 0 {
            mods_vec.push(Mod::Key8);
        }
        if mods & Mod::FadeIn as u32 != 0 {
            mods_vec.push(Mod::FadeIn);
        }
        if mods & Mod::Random as u32 != 0 {
            mods_vec.push(Mod::Random);
        }
        if mods & Mod::Cinema as u32 != 0 {
            mods_vec.push(Mod::Cinema);
        }   
        if mods & Mod::Target as u32 != 0 {
            mods_vec.push(Mod::Target);
        }
        if mods & Mod::Key9 as u32 != 0 {
            mods_vec.push(Mod::Key9);
        }
        if mods & Mod::KeyCoop as u32 != 0 {
            mods_vec.push(Mod::KeyCoop);
        }
        if mods & Mod::Key1 as u32 != 0 {
            mods_vec.push(Mod::Key1);
        }
        if mods & Mod::Key3 as u32 != 0 {
            mods_vec.push(Mod::Key3);
        }
        if mods & Mod::Key2 as u32 != 0 {
            mods_vec.push(Mod::Key2);
        }
        if mods & Mod::ScoreV2 as u32 != 0 {
            mods_vec.push(Mod::ScoreV2);
        }
        if mods & Mod::Mirror as u32 != 0 {
            mods_vec.push(Mod::Mirror);
        }
        Mods {
            mods: mods_vec,
        }
    }
}

#[derive(Debug)]
pub enum KeysMania {
    K1 = 1 << 0,
    K2 = 1 << 1,
    K3 = 1 << 2,
    K4 = 1 << 3,
    K5 = 1 << 4,
    K6 = 1 << 5,
    K7 = 1 << 6,
    K8 = 1 << 7,
    K9 = 1 << 8,
    K10 = 1 << 9,
    K11 = 1 << 10,
    K12 = 1 << 11,
    K13 = 1 << 12,
    K14 = 1 << 13,
    K15 = 1 << 14,
    K16 = 1 << 15,
    K17 = 1 << 16,
    K18 = 1 << 17,
}

impl KeysMania {
    pub fn from_u32(keys: u32) -> KeysMania {
        match keys {
            1 => KeysMania::K1,
            2 => KeysMania::K2,
            4 => KeysMania::K3,
            8 => KeysMania::K4,
            16 => KeysMania::K5,
            32 => KeysMania::K6,
            64 => KeysMania::K7,
            128 => KeysMania::K8,
            256 => KeysMania::K9,
            512 => KeysMania::K10,
            1024 => KeysMania::K11,
            2048 => KeysMania::K12,
            4096 => KeysMania::K13,
            8192 => KeysMania::K14,
            16384 => KeysMania::K15,
            32768 => KeysMania::K16,
            65536 => KeysMania::K17,
            131072 => KeysMania::K18,
            _ => panic!("Invalid keys value"),
        }
    }
}

#[derive(Debug)]
pub struct ReplayEvent {
    pub time: u32,
    pub keys_pressed: KeysMania,
}

#[derive(Debug)]
pub struct LifeBar {
    pub time: u32,
    pub hp: f32,
}