// // osu replay data unpacker

// use std::io::{Read, Cursor};
// use byteorder::{LittleEndian, ReadBytesExt};
// use crate::{util::{LifeBar, ReplayEvent, Mods, GameMode, KeysMania}, parser::Replay};

// struct Unpacker {
//     offset: usize,
//     data: Vec<u8>,
// }

// impl Unpacker {
//     fn new(data: Vec<u8>) -> Unpacker {
//         Unpacker {
//             offset: 0,
//             data: data,
//         }
//     }

//     fn read_int(&mut self) -> u32 {

//     }

//     fn read_life_bar(&mut self) -> LifeBar {
//         let time = self.read_int();
//         let percentage = self.read_flo();
//         LifeBar {
//             time: time,
//             hp: percentage,
//         }
//     }

//     fn read_replay_event(&mut self, mode: GameMode) -> ReplayEvent {
//         let time = self.read_int();
//         let x = self.read_short();
//         let y = self.read_short();
//         let key_state = self.read_int();
        
//         let mut keys_pressed;
//         if mode == GameMode::MANIA {
//             keys_pressed = KeysMania::from_u32(key_state);
//         }

//         ReplayEvent {
//             time: time,
//             keys_pressed: keys_pressed,
//         }
//     }
// }