/// osu! Replay Parser
/// 
/// This library is used to parse osu! replays.

pub mod parser;
mod unpack;
mod util;

#[cfg(test)]
mod tests {
    use crate::parser::Replay;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_parse_replay() {
        let replay = Replay::new("tests/replays/mania.osr");
        // println!("{:?}", replay);
    }
}