use serde::{Deserialize, Serialize};

/// Response body of a chess.com monthly archive endpoint: a list of games
/// played in that month.
#[derive(Serialize, Deserialize, Debug)]
pub struct ManyGames {
    pub games: Vec<Game>,
}

/// A single completed game, as returned by the chess.com public API.
#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    /// Link to the game on chess.com.
    pub url: String,
    /// Full PGN (Portable Game Notation) transcript of the game.
    pub pgn: String,
    /// Time control string, e.g. `"180+2"`.
    pub time_control: String,
    /// Unix timestamp (seconds) of when the game ended.
    pub end_time: u64,
    /// Whether the game was rated.
    pub rated: bool,
    /// Move accuracy percentages for each side, if available.
    pub accuracies: Option<Accuracies>,
    /// Compact "TCN" move encoding used internally by chess.com.
    pub tcn: String,
    /// Unique identifier for the game.
    pub uuid: String,
    /// Starting position of the game, in FEN.
    pub initial_setup: String,
    /// Final position of the game, in FEN.
    pub fen: String,
    /// Time class the game was played under (bullet, blitz, etc.).
    pub time_class: TimeClass,
    /// Game variant, e.g. `"chess"`.
    pub rules: String,
    /// Stats for the player with the white pieces.
    pub white: PlayerStats,
    /// Stats for the player with the black pieces.
    pub black: PlayerStats,
    /// ECO opening classification URL, if known.
    pub eco: Option<String>,
}

/// Per-side move accuracy percentages for a game.
#[derive(Serialize, Deserialize, Debug)]
pub struct Accuracies {
    pub black: f64,
    pub white: f64,
}

/// A player's rating and outcome for one side of a single game.
#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerStats {
    /// Player's rating at the time of the game.
    pub rating: u64,
    /// Game result from this player's perspective, e.g. `"win"`, `"resigned"`.
    pub result: String,
    /// URL identifying the player on the chess.com API.
    #[serde(rename = "@id")]
    pub id: String,
    /// The player's chess.com username.
    pub username: String,
    /// Unique identifier for the player.
    pub uuid: String,
}

/// The time class a game was played under.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TimeClass {
    Daily,
    Blitz,
    Rapid,
    Bullet,
}

/// Response body of the chess.com "list of monthly archives" endpoint:
/// URLs of each monthly archive available for a player.
#[derive(Serialize, Deserialize)]
pub struct Archives {
    pub archives: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_game_from_chess_com_json() {
        let json = r#"{
  "url": "https://www.chess.com/game/live/173038219118",
  "pgn": "[Event \"Live Chess\"]\n[Site \"Chess.com\"]\n[Date \"2026.08.15\"]\n[Round \"-\"]\n[White \"TenderLlama\"]\n[Black \"jackFLAKE\"]\n[Result \"0-1\"]\n[CurrentPosition \"8/2pN4/2P1p3/1k1p1p2/1P1PrPp1/6n1/8/1R1K4 w - - 3 41\"]\n[Timezone \"UTC\"]\n[ECO \"A10\"]\n[ECOUrl \"https://www.chess.com/openings/English-Opening-1...b6-2.Nf3-Bb7-3.g3\"]\n[UTCDate \"2026.08.15\"]\n[UTCTime \"17:25:28\"]\n[WhiteElo \"985\"]\n[BlackElo \"980\"]\n[TimeControl \"180+2\"]\n[Termination \"jackFLAKE won by resignation\"]\n[StartTime \"17:25:28\"]\n[EndDate \"2026.08.15\"]\n[EndTime \"17:31:41\"]\n[Link \"https://www.chess.com/game/live/173038219118\"]\n\n1. c4 {[%clk 0:02:59.5]} 1... b6 {[%clk 0:03:01.9]} 2. g3 {[%clk 0:02:59.9]} 2... Bb7 {[%clk 0:03:02.7]} 3. Nf3 {[%clk 0:03:01]} 3... g5 {[%clk 0:03:01.8]} 4. Bg2 {[%clk 0:03:01.3]} 4... g4 {[%clk 0:03:02.9]} 5. Nh4 {[%clk 0:02:58.3]} 5... Bxg2 {[%clk 0:03:03.5]} 6. Nxg2 {[%clk 0:02:58.9]} 6... e6 {[%clk 0:03:04]} 7. O-O {[%clk 0:02:57.1]} 7... h5 {[%clk 0:03:04.1]} 8. Nh4 {[%clk 0:02:53.8]} 8... Nc6 {[%clk 0:03:04.3]} 9. e3 {[%clk 0:02:50.3]} 9... Be7 {[%clk 0:03:04]} 10. f4 {[%clk 0:02:30.1]} 10... Bxh4 {[%clk 0:03:04]} 11. gxh4 {[%clk 0:02:30.6]} 11... Qxh4 {[%clk 0:03:04.7]} 12. d4 {[%clk 0:02:21.7]} 12... f5 {[%clk 0:03:02.3]} 13. Qe2 {[%clk 0:02:18.8]} 13... O-O-O {[%clk 0:02:59.7]} 14. Nc3 {[%clk 0:02:17.3]} 14... a6 {[%clk 0:02:59.2]} 15. c5 {[%clk 0:02:15.3]} 15... b5 {[%clk 0:03:00.2]} 16. Bd2 {[%clk 0:02:09.4]} 16... Nf6 {[%clk 0:02:57.4]} 17. Be1 {[%clk 0:02:09.1]} 17... g3 {[%clk 0:02:54.6]} 18. Bxg3 {[%clk 0:02:06.3]} 18... Qg4 {[%clk 0:02:55.4]} 19. Qxg4 {[%clk 0:01:58.9]} 19... hxg4 {[%clk 0:02:56.4]} 20. b3 {[%clk 0:01:40.8]} 20... b4 {[%clk 0:02:56.7]} 21. Na4 {[%clk 0:01:28]} 21... Ne4 {[%clk 0:02:56.7]} 22. Kg2 {[%clk 0:01:19.3]} 22... Rdg8 {[%clk 0:02:56.1]} 23. Nb2 {[%clk 0:01:17.6]} 23... Nxg3 {[%clk 0:02:48.8]} 24. hxg3 {[%clk 0:01:18.1]} 24... Ne7 {[%clk 0:02:47]} 25. Nc4 {[%clk 0:01:17.3]} 25... Nd5 {[%clk 0:02:45.8]} 26. a3 {[%clk 0:01:01.5]} 26... bxa3 {[%clk 0:02:44.5]} 27. Rxa3 {[%clk 0:01:02.3]} 27... Nf6 {[%clk 0:02:43.7]} 28. Rxa6 {[%clk 0:01:01.9]} 28... d5 {[%clk 0:02:43.8]} 29. Ne5 {[%clk 0:00:52.9]} 29... Ne4 {[%clk 0:02:37.2]} 30. Ra8+ {[%clk 0:00:45.1]} 30... Kb7 {[%clk 0:02:37.9]} 31. Rxg8 {[%clk 0:00:41.8]} 31... Rxg8 {[%clk 0:02:38.8]} 32. c6+ {[%clk 0:00:29.9]} 32... Kb6 {[%clk 0:02:37.6]} 33. Rc1 {[%clk 0:00:25.5]} 33... Rh8 {[%clk 0:02:36.7]} 34. Rh1 {[%clk 0:00:20.3]} 34... Ra8 {[%clk 0:02:34.5]} 35. Rb1 {[%clk 0:00:19.9]} 35... Ra2+ {[%clk 0:02:35]} 36. Kf1 {[%clk 0:00:20.5]} 36... Nxg3+ {[%clk 0:02:35.5]} 37. Ke1 {[%clk 0:00:21.4]} 37... Re2+ {[%clk 0:02:33.9]} 38. Kd1 {[%clk 0:00:22.3]} 38... Rxe3 {[%clk 0:02:35]} 39. b4 {[%clk 0:00:22.2]} 39... Re4 {[%clk 0:02:28.9]} 40. Nd7+ {[%clk 0:00:16.9]} 40... Kb5 {[%clk 0:02:27.3]} 0-1\n",
  "time_control": "180+2",
  "end_time": 1786815101,
  "rated": true,
  "accuracies": {
    "white": 62.15,
    "black": 68.59
  },
  "tcn": "kAXPow6Xgv2MfoMEvFXoFo0Seg3NoF5Qmu90nD0FwF7FlB1Ldm86bsWOAIPHcl!TleEwewFEmENEjrHzsyTCgo7!yjCwpwQ0jA0JiqzqaqJTqOZJAKTCO46X4!?!IQXPfc!?ch?4hb4iofCwfeimedmurzuCKZPH",
  "uuid": "4eef337e-98ce-11f1-b17d-52282501000f",
  "initial_setup": "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
  "fen": "8/2pN4/2P1p3/1k1p1p2/1P1PrPp1/6n1/8/1R1K4 w - - 3 41",
  "time_class": "blitz",
  "rules": "chess",
  "white": {
    "rating": 985,
    "result": "resigned",
    "@id": "https://api.chess.com/pub/player/tenderllama",
    "username": "TenderLlama",
    "uuid": "2b132172-3a6b-11eb-b3a1-b178c3191ff1"
  },
  "black": {
    "rating": 980,
    "result": "win",
    "@id": "https://api.chess.com/pub/player/jackflake",
    "username": "jackFLAKE",
    "uuid": "523755a0-5994-11eb-9227-c5c2c1789bf7"
  },
  "eco": "https://www.chess.com/openings/English-Opening-1...b6-2.Nf3-Bb7-3.g3"
}"#;
        let game: Game = serde_json::from_str(json).unwrap();
        assert_eq!(game.time_class, TimeClass::Blitz);
    }
}
