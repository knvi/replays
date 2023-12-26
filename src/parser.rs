use std::{io::{self, Cursor, Read}, fs::File};

use byteorder::ReadBytesExt;

use crate::util::{LifeBar, ReplayEvent, Mods, GameMode, KeysMania};

/// osu! Replay Parser

#[derive(Debug)]
pub struct Replay {
    pub game_mode: GameMode,
    pub version: u32,
    pub beatmap_hash: String,
    pub player_name: String,
    pub replay_hash: String,
    pub hit300: u16,
    pub hit100: u16,
    pub hit50: u16,
    pub geki: u16,
    pub katu: u16,
    pub miss: u16,
    pub score: u32,
    pub combo: u16,
    pub fc: u8,
    pub mods: Mods,
    pub hp_sequence: Vec<LifeBar>,
    pub timestamp: u64,
    pub replay_length: u32,
    pub replay_data: Vec<ReplayEvent>,
    pub online_score_id: u64,
    pub additional_mods: u32,
}

pub fn read_string(cursor: &mut io::Cursor<Vec<u8>>) -> io::Result<Option<String>> {
    let mut indicator = [0; 1];
    cursor.read_exact(&mut indicator)?;

    println!("{:?}", indicator);

    match indicator[0] {
        0x00 => Ok(None),
        0x0b => {
            let length = read_uleb128(cursor)?;
            let mut buffer = vec![0; length as usize];
            cursor.read_exact(&mut buffer)?;
            let string = String::from_utf8(buffer).unwrap();
            Ok(Some(string))
        },
        _ => panic!("Invalid string indicator"),
    }
}

pub fn read_uleb128(cursor: &mut io::Cursor<Vec<u8>>) -> io::Result<u32> {
    let mut result = 0;
    let mut shift = 0;
    loop {
        let mut byte = [0; 1];
        cursor.read_exact(&mut byte)?;
        let value = byte[0] & 0x7f;
        result |= (value as u32) << shift;
        if byte[0] & 0x80 == 0 {
            break;
        }
        shift += 7;
    }
    Ok(result)
}

pub fn read_life_bar(cursor: &mut io::Cursor<Vec<u8>>) -> io::Result<Vec<LifeBar>> {
    let mut life_graph = Vec::new();
    let mut life_graph_str = read_string(cursor)?.unwrap();

    life_graph_str = life_graph_str.trim_end_matches(',').to_string();
    let entries = life_graph_str.split(",").collect::<Vec<&str>>();
    for entry in entries {
        let life_graph_entry = entry.split("|").collect::<Vec<&str>>();
        let time = life_graph_entry[0].parse::<u32>().unwrap_or(0);
        let percentage = life_graph_entry[1].parse::<f32>().unwrap_or(0.0);

        life_graph.push(LifeBar {
            time: time,
            hp: percentage,
        });
    }

    Ok(life_graph)
}

pub fn read_replay_event(cursor: &mut io::Cursor<Vec<u8>>) -> io::Result<Vec<ReplayEvent>> {
    let mut lzma_data: Vec<u8> = Vec::new();
    lzma_rs::lzma_decompress(&mut cursor.clone(), &mut lzma_data).unwrap();

    let mut replay_events = Vec::new();
    
    let lzma_data_str = String::from_utf8(lzma_data).unwrap();
    let events: Vec<&str> = lzma_data_str.split(",").filter(|x| !x.is_empty()).collect();
    println!("{:?}", events);

    let mut rng_seed: Option<u32> = None;
    let mut event_count = 0;

    for event in events {
        let event_data: Vec<&str> = event.split("|").collect();
        println!("{:?}", event_data);
        let time = event_data[0].parse::<u32>().unwrap_or(0);
        let x = event_data[1];
        let y = event_data[2];
        let keys_pressed = event_data[3].parse::<u32>().unwrap_or(0);

        if event_count < 2 && x.parse::<f32>().unwrap() == 256.0 && y.parse::<f32>().unwrap() == -500.0 {
            continue;
        }

        if time == 0 {
            rng_seed = Some(time);
        }

        replay_events.push(ReplayEvent {
            time: time,
            keys_pressed: KeysMania::from_u32(x.parse::<f32>().unwrap() as u32),
        });

        event_count += 1;
    }

    Ok(replay_events)
}

pub fn read_from_path(path: &str) -> io::Result<Replay> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut cursor = Cursor::new(buffer);
    let game_mode = GameMode::from_u8(cursor.read_u8()?);
    let version = cursor.read_u32::<byteorder::LittleEndian>()?;
    let beatmap_hash = read_string(&mut cursor)?;
    let player_name = read_string(&mut cursor)?;
    let replay_hash = read_string(&mut cursor)?;
    let hit300 = cursor.read_u16::<byteorder::LittleEndian>()?;
    let hit100 = cursor.read_u16::<byteorder::LittleEndian>()?;
    let hit50 = cursor.read_u16::<byteorder::LittleEndian>()?;
    let geki = cursor.read_u16::<byteorder::LittleEndian>()?;
    let katu = cursor.read_u16::<byteorder::LittleEndian>()?;
    let miss = cursor.read_u16::<byteorder::LittleEndian>()?;
    let score = cursor.read_u32::<byteorder::LittleEndian>()?;
    let combo = cursor.read_u16::<byteorder::LittleEndian>()?;
    let fc = cursor.read_u8()?;
    let mods = Mods::from_u32(cursor.read_u32::<byteorder::LittleEndian>()?);
    let hp_sequence = read_life_bar(&mut cursor)?;
    println!("Finished reading life bar");
    let timestamp = cursor.read_u64::<byteorder::LittleEndian>()?;
    let replay_length = cursor.read_u32::<byteorder::LittleEndian>()?;
    let replay_data = read_replay_event(&mut cursor)?;
    let online_score_id = cursor.read_u64::<byteorder::LittleEndian>()?;
    let additional_mods = cursor.read_u32::<byteorder::LittleEndian>()?;

    Ok(Replay {
        game_mode: game_mode,
        version: version,
        beatmap_hash: beatmap_hash.unwrap(),
        player_name: player_name.unwrap(),
        replay_hash: replay_hash.unwrap(),
        hit300: hit300,
        hit100: hit100,
        hit50: hit50,
        geki: geki,
        katu: katu,
        miss: miss,
        score: score,
        combo: combo,
        fc: fc,
        mods: mods,
        hp_sequence: hp_sequence,
        timestamp: timestamp,
        replay_length: replay_length,
        replay_data: replay_data,
        online_score_id: online_score_id,
        additional_mods: additional_mods,
    })
}

impl Replay {
    pub fn new(path: &str) -> Replay {
        let replay = read_from_path(path)
            .expect("Failed to read replay file");

        replay
    }
}