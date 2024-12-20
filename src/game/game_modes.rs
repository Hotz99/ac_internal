pub enum GameModes {
    GmodeBotTeamdeathMatch = 7,
    BotDeathMatch = 8,
    BotOneShotOneKill = 12,
    GmodeBotPistolFrenzy = 18,
    Botlss = 19,
    BotSurvivor = 20,
    GmodeBotTeamOneShotOneKill = 21,
}

impl GameModes {
    pub fn from_i32(int: i32) -> GameModes {
        match int {
            7 => GameModes::GmodeBotTeamdeathMatch,
            8 => GameModes::BotDeathMatch,
            12 => GameModes::BotOneShotOneKill,
            18 => GameModes::GmodeBotPistolFrenzy,
            19 => GameModes::Botlss,
            20 => GameModes::BotSurvivor,
            21 => GameModes::GmodeBotTeamOneShotOneKill,
            _ => panic!("unsupported game mode"),
        }
    }
}
