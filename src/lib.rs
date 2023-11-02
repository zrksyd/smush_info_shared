//#![feature(const_mut_refs)]
use std::sync::atomic::{AtomicU32, AtomicBool, Ordering};
use serde::{Serialize, Deserialize};
use std::fmt;

mod atomic_f32;
pub use atomic_f32::AtomicF32;

mod atomic_arena_id;
pub use atomic_arena_id::AtomicArenaId;

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    pub arena_id: AtomicArenaId,
    pub remaining_frames: AtomicU32,
    pub is_match: AtomicBool,
    pub stage: AtomicU32,
    pub players: [Player; 8]
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub character: AtomicU32,
    pub stocks: AtomicU32,
    pub damage: AtomicF32,
    pub is_cpu: AtomicBool,
    pub skin: AtomicU32
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum Character {
    None = 0,
    Bayonetta,
    Brave,
    Buddy,
    Captain,
    Chrom,
    Cloud,
    Daisy,
    Dedede,
    Demon,
    Diddy,
    Dolly,
    Donkey,
    Duckhunt,
    Edge,
    Eflame,
    Element,
    Elight,
    Falco,
    Fox,
    Fushigisou,
    Gamewatch,
    Ganon,
    Gaogaen,
    Gekkouga,
    Ike,
    Inkling,
    Jack,
    Kamui,
    Ken,
    Kirby,
    Koopa,
    Koopag,
    Koopajr,
    Krool,
    Link,
    Littlemac,
    Lizardon,
    Lucario,
    Lucas,
    Lucina,
    Luigi,
    Mario,
    Mariod,
    Marth,
    Master,
    Metaknight,
    Mewtwo,
    Miienemyf,
    Miienemyg,
    Miienemys,
    Miifighter,
    Miigunner,
    Miiswordsman,
    Murabito,
    Nana,
    Ness,
    Packun,
    Pacman,
    Palutena,
    Peach,
    Pfushigisou,
    Pichu,
    Pickel,
    Pikachu,
    Pikmin,
    Pit,
    Pitb,
    Plizardon,
    Popo,
    Ptrainer,
    Purin,
    Pzenigame,
    Reflet,
    Richter,
    Ridley,
    Robot,
    Rockman,
    Rosetta,
    Roy,
    Ryu,
    Samus,
    Samusd,
    Sheik,
    Shizue,
    Shulk,
    Simon,
    Snake,
    Sonic,
    Szerosuit,
    Tantan,
    Toonlink,
    Trail,
    Wario,
    Wiifit,
    Wolf,
    Yoshi,
    Younglink,
    Zelda,
    Zenigame,
    Max,
}

// see `Character` for how this should be used
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum Stage {
    None = 0,
    _75m,
    Animal_City,
    Animal_Island,
    Animal_Village,
    BalloonFight,
    Battle_75m,
    Battle_Animal_City,
    Battle_Animal_Island,
    Battle_Animal_Village,
    Battle_BalloonFight,
    Battle_Bayo_Clock,
    Battle_Brave_Altar,
    Battle_Buddy_Spiral,
    Battle_Demon_Dojo,
    Battle_DK_Jungle,
    Battle_DK_Lodge,
    Battle_DK_WaterFall,
    Battle_Dolly_Stadium,
    Battle_Dracula_Castle,
    Battle_DuckHunt,
    Battle_End,
    Battle_FE_Arena,
    Battle_FE_Colloseum,
    Battle_FE_Shrine,
    Battle_FE_Siege,
    Battle_FF_Cave,
    Battle_FF_Midgar,
    Battle_FlatZoneX,
    Battle_Fox_Corneria,
    Battle_Fox_LylatCruise,
    Battle_Fox_Venom,
    Battle_Fzero_Bigblue,
    Battle_Fzero_Mutecity3DS,
    Battle_Fzero_Porttown,
    Battle_Icarus_Angeland,
    Battle_Icarus_SkyWorld,
    Battle_Icarus_Uprising,
    Battle_Ice_Top,
    Battle_Jack_Mementoes,
    Battle_Kart_CircuitFor,
    Battle_Kart_CircuitX,
    Battle_Kirby_Cave,
    Battle_Kirby_Fountain,
    Battle_Kirby_Gameboy,
    Battle_Kirby_Greens,
    Battle_Kirby_Halberd,
    Battle_Kirby_Pupupu64,
    Battle_LuigiMansion,
    Battle_Mario_3DLand,
    Battle_Mario_Castle64,
    Battle_Mario_CastleDx,
    Battle_Mario_Dolpic,
    Battle_Mario_Galaxy,
    Battle_Mario_Maker,
    Battle_Mario_NewBros2,
    Battle_Mario_Odyssey,
    Battle_Mario_Paper,
    Battle_Mario_Past64,
    Battle_Mario_PastUsa,
    Battle_Mario_PastX,
    Battle_Mario_Rainbow,
    Battle_Mario_Uworld,
    Battle_MarioBros,
    Battle_Metroid_Kraid,
    Battle_Metroid_Norfair,
    Battle_Metroid_Orpheon,
    Battle_Metroid_ZebesDx,
    Battle_MG_Shadowmoses,
    Battle_Mother_Fourside,
    Battle_Mother_Magicant,
    Battle_Mother_Newpork,
    Battle_Mother_Onett,
    Battle_Nintendogs,
    Battle_Pac_Land,
    Battle_Pickel_World,
    Battle_Pictochat2,
    Battle_Pikmin_Garden,
    Battle_Pikmin_Planet,
    Battle_Pilotwings,
    Battle_Plankton,
    Battle_Poke_Kalos,
    Battle_Poke_Stadium,
    Battle_Poke_Stadium2,
    Battle_Poke_Tengam,
    Battle_Poke_Tower,
    Battle_Poke_Unova,
    Battle_Poke_Yamabuki,
    Battle_PunchOutSB,
    Battle_PunchOutW,
    Battle_Rock_Wily,
    Battle_SF_Suzaku,
    Battle_Sonic_Greenhill,
    Battle_Sonic_Windyhill,
    Battle_Spla_Parking,
    Battle_StreetPass,
    Battle_Tantan_Spring,
    Battle_Tomodachi,
    Battle_Trail_Castle,
    Battle_Wario_Gamer,
    Battle_Wario_Madein,
    Battle_WiiFit,
    Battle_WreckingCrew,
    Battle_WufuIsland,
    Battle_Xeno_Alst,
    Battle_Xeno_Gaur,
    Battle_Yoshi_CartBoard,
    Battle_Yoshi_Island,
    Battle_Yoshi_Story,
    Battle_Yoshi_Yoster,
    Battle_Zelda_Gerudo,
    Battle_Zelda_Greatbay,
    Battle_Zelda_Hyrule,
    Battle_Zelda_Oldin,
    Battle_Zelda_Pirates,
    Battle_Zelda_Skyward,
    Battle_Zelda_Temple,
    Battle_Zelda_Tower,
    Battle_Zelda_Train,
    BattleField_L,
    BattleField_S,
    BattleField,
    Bayo_Clock,
    BonusGame,
    BossStage_Dracula,
    BossStage_Final1,
    BossStage_Final2,
    BossStage_Final3,
    BossStage_Galleom,
    BossStage_GanonBoss,
    BossStage_Marx,
    BossStage_Rathalos,
    Brave_Altar,
    Buddy_Spiral,
    CampaignMap,
    Demon_Dojo,
    DK_Jungle,
    DK_Lodge,
    DK_WaterFall,
    Dolly_Stadium,
    Dracula_Castle,
    DuckHunt,
    End_75m,
    End_Animal_City,
    End_Animal_Island,
    End_Animal_Village,
    End_BalloonFight,
    End_BattleField,
    End_Bayo_Clock,
    End_Brave_Altar,
    End_Buddy_Spiral,
    End_Demon_Dojo,
    End_DK_Jungle,
    End_DK_Lodge,
    End_DK_WaterFall,
    End_Dolly_Stadium,
    End_Dracula_Castle,
    End_DuckHunt,
    End_FE_Arena,
    End_FE_Colloseum,
    End_FE_Shrine,
    End_FE_Siege,
    End_FF_Cave,
    End_FF_Midgar,
    End_FlatZoneX,
    End_Fox_Corneria,
    End_Fox_LylatCruise,
    End_Fox_Venom,
    End_Fzero_Bigblue,
    End_Fzero_Mutecity3DS,
    End_Fzero_Porttown,
    End_Icarus_Angeland,
    End_Icarus_SkyWorld,
    End_Icarus_Uprising,
    End_Ice_Top,
    End_Jack_Mementoes,
    End_Kart_CircuitFor,
    End_Kart_CircuitX,
    End_Kirby_Cave,
    End_Kirby_Fountain,
    End_Kirby_Gameboy,
    End_Kirby_Greens,
    End_Kirby_Halberd,
    End_Kirby_Pupupu64,
    End_LuigiMansion,
    End_Mario_3DLand,
    End_Mario_Castle64,
    End_Mario_CastleDx,
    End_Mario_Dolpic,
    End_Mario_Galaxy,
    End_Mario_Maker,
    End_Mario_NewBros2,
    End_Mario_Odyssey,
    End_Mario_Paper,
    End_Mario_Past64,
    End_Mario_PastUsa,
    End_Mario_PastX,
    End_Mario_Rainbow,
    End_Mario_Uworld,
    End_MarioBros,
    End_Metroid_Kraid,
    End_Metroid_Norfair,
    End_Metroid_Orpheon,
    End_Metroid_ZebesDx,
    End_MG_Shadowmoses,
    End_Mother_Fourside,
    End_Mother_Magicant,
    End_Mother_Newpork,
    End_Mother_Onett,
    End_Nintendogs,
    End_Pac_Land,
    End_Pickel_World,
    End_Pictochat2,
    End_Pikmin_Garden,
    End_Pikmin_Planet,
    End_Pilotwings,
    End_Plankton,
    End_Poke_Kalos,
    End_Poke_Stadium,
    End_Poke_Stadium2,
    End_Poke_Tengam,
    End_Poke_Tower,
    End_Poke_Unova,
    End_Poke_Yamabuki,
    End_PunchOutSB,
    End_PunchOutW,
    End_Rock_Wily,
    End_SF_Suzaku,
    End_Sonic_Greenhill,
    End_Sonic_Windyhill,
    End_Spla_Parking,
    End_StreetPass,
    End_Tantan_Spring,
    End_Tomodachi,
    End_Trail_Castle,
    End_Wario_Gamer,
    End_Wario_Madein,
    End_WiiFit,
    End_WreckingCrew,
    End_WufuIsland,
    End_Xeno_Alst,
    End_Xeno_Gaur,
    End_Yoshi_CartBoard,
    End_Yoshi_Island,
    End_Yoshi_Story,
    End_Yoshi_Yoster,
    End_Zelda_Gerudo,
    End_Zelda_Greatbay,
    End_Zelda_Hyrule,
    End_Zelda_Oldin,
    End_Zelda_Pirates,
    End_Zelda_Skyward,
    End_Zelda_Temple,
    End_Zelda_Tower,
    End_Zelda_Train,
    End,
    FE_Arena,
    FE_Colloseum,
    FE_Shrine,
    FE_Siege,
    FF_Cave,
    FF_Midgar,
    FlatZoneX,
    Fox_Corneria,
    Fox_LylatCruise,
    Fox_Venom,
    Fzero_Bigblue,
    Fzero_Mutecity3DS,
    Fzero_Porttown,
    HomerunContest,
    Icarus_Angeland,
    Icarus_SkyWorld,
    Icarus_Uprising,
    Ice_Top,
    Invalid,
    Jack_Mementoes,
    Kart_CircuitFor,
    Kart_CircuitX,
    Kirby_Cave,
    Kirby_Fountain,
    Kirby_Gameboy,
    Kirby_Greens,
    Kirby_Halberd,
    Kirby_Pupupu64,
    LuigiMansion,
    Mario_3DLand,
    Mario_Castle64,
    Mario_CastleDx,
    Mario_Dolpic,
    Mario_Galaxy,
    Mario_Maker,
    Mario_NewBros2,
    Mario_Odyssey,
    Mario_Paper,
    Mario_Past64,
    Mario_PastUsa,
    Mario_PastX,
    Mario_Rainbow,
    Mario_Uworld,
    MarioBros,
    Metroid_Kraid,
    Metroid_Norfair,
    Metroid_Orpheon,
    Metroid_ZebesDx,
    MG_Shadowmoses,
    Mother_Fourside,
    Mother_Magicant,
    Mother_Newpork,
    Mother_Onett,
    Nintendogs,
    Pac_Land,
    PhotoStage,
    Pickel_World,
    Pictochat2,
    Pikmin_Garden,
    Pikmin_Planet,
    Pilotwings,
    Plankton,
    Poke_Kalos,
    Poke_Stadium,
    Poke_Stadium2,
    Poke_Tengam,
    Poke_Tower,
    Poke_Unova,
    Poke_Yamabuki,
    PunchOutSB,
    PunchOutW,
    ResultStage_Jack,
    ResultStage,
    Rock_Wily,
    SettingStage,
    SF_Suzaku,
    ShamFight,
    Sonic_Greenhill,
    Sonic_Windyhill,
    SP_Edit,
    SpiritsRoulette,
    Spla_Parking,
    Staffroll,
    StreetPass,
    Tantan_Spring,
    TestStage,
    Tomodachi,
    Trail_Castle,
    Training,
    Wario_Gamer,
    Wario_Madein,
    WiiFit,
    WreckingCrew,
    WufuIsland,
    Xeno_Alst,
    Xeno_Gaur,
    Yoshi_CartBoard,
    Yoshi_Island,
    Yoshi_Story,
    Yoshi_Yoster,
    Zelda_Gerudo,
    Zelda_Greatbay,
    Zelda_Hyrule,
    Zelda_Oldin,
    Zelda_Pirates,
    Zelda_Skyward,
    Zelda_Temple,
    Zelda_Tower,
    Zelda_Train,
    Max
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Character::*;
        write!(f, "{}", match self {
            Bayonetta => "Bayonetta",
            Brave => "Hero",
            Buddy => "Banjo & Kazooie",
            Captain => "Capitan Falcon",
            Chrom => "Chrom",
            Cloud => "Cloud",
            Daisy => "Daisy",
            Demon => "Kazuya",
            Dedede => "King Dedede",
            Diddy => "Diddy Kong",
            Dolly => "Terry",
            Donkey => "Donkey Kong",
            Duckhunt => "Duck Hunt",
            Edge => "Sephiroth",
            Eflame => "Pyra",
            Elight => "Mythra",
            Falco => "Falco",
            Fox => "Fox",
            Fushigisou => "Ivysaur",
            Gamewatch => "Mr. Game & Watch",
            Ganon => "Ganondorf",
            Gaogaen => "Incineroar",
            Gekkouga => "Greninja",
            Ike => "Ike",
            Inkling => "Inkling",
            Jack => "Joker",
            Kamui => "Corrin",
            Ken => "Ken",
            Kirby => "Kirby",
            Koopa => "Bowser",
            Koopag => "Giga Bowser",
            Koopajr => "Bowser Jr.",
            Krool => "King K. Rool",
            Link => "Link",
            Littlemac => "Little Mac",
            Lizardon => "Charizard",
            Lucario => "Lucario",
            Lucas => "Lucas",
            Lucina => "Lucina",
            Luigi => "Luigi",
            Mario => "Mario",
            Mariod => "Dr. Mario",
            Marth => "Marth",
            Master => "Byleth",
            Metaknight => "Meta Knight",
            Mewtwo => "Mewtwo",
            Miienemyf => "Mii Brawler",
            Miienemyg => "Mii Gunner",
            Miienemys => "Mii Swordfighter",
            Miifighter => "Mii Brawler",
            Miigunner => "Mii Gunner",
            Miiswordsman => "Mii Swordfighter",
            Murabito => "Villager",
            Nana => "Ice Climbers",
            Ness => "Ness",
            Packun => "Piranha Plant",
            Pacman => "Pac-Man",
            Palutena => "Palutena",
            Peach => "Peach",
            Pfushigisou => "Ivysaur",
            Pichu => "Pichu",
            Pickel => "Steve",
            Pikachu => "Pikachu",
            Pikmin => "Olimar",
            Pit => "Pit",
            Pitb => "Dark Pit",
            Plizardon => "Charizard",
            Popo => "Ice Climbers",
            Purin => "Jigglypuff",
            Pzenigame => "Squirtle",
            Reflet => "Robin",
            Richter => "Richter",
            Ridley => "Ridley",
            Robot => "R.O.B.",
            Rockman => "Mega Man",
            Rosetta => "Rosalina & Luma",
            Roy => "Roy",
            Ryu => "Ryu",
            Samus => "Samus",
            Samusd => "Dark Samus",
            Sheik => "Sheik",
            Shizue => "Isabelle",
            Shulk => "Shulk",
            Simon => "Simon",
            Snake => "Snake",
            Sonic => "Sonic",
            Szerosuit => "Zero Suit Samus",
            Tantan => "Min Min",
            Toonlink => "Toon Link",
            Trail => "Sora",
            Wario => "Wario",
            Wiifit => "Wii Fit Trainer",
            Wolf => "Wolf",
            Yoshi => "Yoshi",
            Younglink => "Young Link",
            Zelda => "Zelda",
            Zenigame => "Squirtle",
            _ => "Invalid"
        })
    }
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Stage::*;
        write!(f, "{}", match self {
            Animal_City => "Town and City",
            Animal_Island => "Tortimer Island",
            Animal_Village => "Smashville",
            BalloonFight => "Balloon Fight",
            BattleField => "Battlefield",
            BattleField_L => "Big Battlefield",
            BattleField_S => "Small Battlefield",
            Bayo_Clock => "Umbra Clock Tower",
            BonusGame => "Bonus Game",
            BossStage_Dracula => "Boss Stage",
            BossStage_Final1 => "Boss Stage",
            BossStage_Final2 => "Boss Stage",
            BossStage_Final3 => "Boss Stage",
            BossStage_Galleom => "Boss Stage",
            BossStage_GanonBoss => "Boss Stage",
            BossStage_Marx => "Boss Stage",
            BossStage_Rathalos => "Boss Stage",
            Brave_Altar => "Yggdrasil's Altar",
            Buddy_Spiral => "Spiral Mountain",
            CampaignMap => "Unknown",
            Demon_Dojo => "Mishima Dojo",
            DK_Jungle => "Kongo Jungle",
            DK_Lodge => "Jungle Japes",
            DK_WaterFall => "Kongo Falls",
            Dolly_Stadium => "King of Fighters Stadium",
            Dracula_Castle => "Dracula’s Castle",
            DuckHunt => "Duck Hunt",
            End => "Final Destination",
            FE_Arena => "Arena Ferox",
            FE_Colloseum => "Coliseum",
            FE_Shrine => "Garreg Mach Monastery",
            FE_Siege => "Castle Siege",
            FF_Cave => "Northern Cave",
            FF_Midgar => "Midgar",
            FlatZoneX => "Flat Zone X",
            Fox_Corneria => "Corneria",
            Fox_LylatCruise => "Lylat Cruise",
            Fox_Venom => "venom",
            Fzero_Bigblue => "Big Blue",
            Fzero_Mutecity3DS => "Mute City SNES",
            Fzero_Porttown => "Port Town Aero Dive",
            HomerunContest => "Homerun Contest",
            Icarus_Angeland => "Palutena’s Temple",
            Icarus_SkyWorld => "Skyworld",
            Icarus_Uprising => "Reset Bomb Forest",
            Ice_Top => "Summit",
            Invalid => "Invalid",
            Jack_Mementoes => "Mementos",
            Kart_CircuitFor => "Mario Circuit",
            Kart_CircuitX => "Figure-8 Circuit",
            Kirby_Cave => "The Great Cave Offensive",
            Kirby_Fountain => "Fountain of Dreams",
            Kirby_Gameboy => "Dream Land GB",
            Kirby_Greens => "Green Greens",
            Kirby_Halberd => "Halberd",
            Kirby_Pupupu64 => "Dream Land",
            LuigiMansion => "Luigi’s Mansion",
            MG_Shadowmoses => "Shadow Moses Island",
            MarioBros => "Mario Bros.",
            Mario_3DLand => "3D Land",
            Mario_Castle64 => "Peach’s Castle",
            Mario_CastleDx => "Princess Peach’s Castle",
            Mario_Dolpic => "Delfino Plaza",
            Mario_Galaxy => "Mario Galaxy",
            Mario_Maker => "Super Mario Maker",
            Mario_NewBros2 => "Golden Plains",
            Mario_Odyssey => "New Donk City Hall",
            Mario_Paper => "Paper Mario",
            Mario_Past64 => "Mushroom Kingdom",
            Mario_PastUsa => "Mushroom Kingdom II",
            Mario_PastX => "Mushroomy Kingdom",
            Mario_Rainbow => "Rainbow Cruise",
            Mario_Uworld => "Mushroom Kingdom U",
            Metroid_Kraid => "Brinstar Depths",
            Metroid_Norfair => "Norfair",
            Metroid_Orpheon => "Frigate Orpheon",
            Metroid_ZebesDx => "Brinstar",
            Mother_Fourside => "Fourside",
            Mother_Magicant => "Magicant",
            Mother_Newpork => "New Pork City",
            Mother_Onett => "Onett",
            Nintendogs => "Living Room",
            Pac_Land => "PAC-LAND",
            PhotoStage => "Photo Stage",
            Pickel_World => "Minecraft World",
            Pictochat2 => "Pictochat 2",
            Pikmin_Garden => "Garden of Hope",
            Pikmin_Planet => "Distant Planet",
            Pilotwings => "Pilotwings",
            Plankton => "Hanenbow",
            Poke_Kalos => "Kalos Pokémon League",
            Poke_Stadium => "Pokémon Stadium",
            Poke_Stadium2 => "Pokémon Stadium 2",
            Poke_Tengam => "Spear Pillar",
            Poke_Tower => "Prism Tower",
            Poke_Unova => "Unova Pokémon League",
            Poke_Yamabuki => "Saffron City",
            PunchOutSB => "Boxing Ring",
            PunchOutW => "Boxing Ring",
            ResultStage => "Results",
            ResultStage_Jack => "Results",
            Rock_Wily => "Wily Castle",
            SF_Suzaku => "Suzaku Castle",
            SP_Edit => "Custom Stage",
            SettingStage => "Settings",
            ShamFight => "Unknown Stage",
            Sonic_Greenhill => "Green Hill Zone",
            Sonic_Windyhill => "Windy Hill Zone",
            SpiritsRoulette => "Spirits Roulette",
            Spla_Parking => "Moray Towers",
            Staffroll => "Staff Credits",
            StreetPass => "Find Mii",
            Tantan_Spring => "Spring Stadium",
            TestStage => "Test Stage",
            Tomodachi => "Tomodachi Life",
            Trail_Castle => "Hollow Bastion",
            Training => "Training",
            Wario_Gamer => "Gamer",
            Wario_Madein => "WarioWare, Inc.",
            WiiFit => "Wii Fit Studio",
            WreckingCrew => "Wrecking Crew",
            WufuIsland => "Wuhu Island",
            Xeno_Alst => "Cloud Sea of Alrest",
            Xeno_Gaur => "Gaur Plain",
            Yoshi_CartBoard => "Yoshi’s Story",
            Yoshi_Island => "Yoshi’s Island",
            Yoshi_Story => "Super Happy Tree",
            Yoshi_Yoster => "Yoshi’s Island (Melee)",
            Zelda_Gerudo => "Gerudo Vally",
            Zelda_Greatbay => "Great Bay",
            Zelda_Hyrule => "Hyrule Castle",
            Zelda_Oldin => "Bridge of Eldin",
            Zelda_Pirates => "Pirate Ship",
            Zelda_Skyward => "Skyloft",
            Zelda_Temple => "Temple",
            Zelda_Tower => "Great Plateau Tower",
            Zelda_Train => "Spirit Train",
            _75m => "75m",
            _ => "Unknown Stage"
        })
    }
}

impl Stage {
    pub fn into_normal(self) -> Self {
        use Stage::*;
        match self {
            Battle_75m => _75m,
            Battle_Animal_City => Animal_City,
            Battle_Animal_Island => Animal_Island,
            Battle_Animal_Village => Animal_Village,
            Battle_BalloonFight => BalloonFight,
            Battle_Bayo_Clock => Bayo_Clock,
            Battle_Brave_Altar => Brave_Altar,
            Battle_Buddy_Spiral => Buddy_Spiral,
            Battle_Demon_Dojo => Demon_Dojo,
            Battle_DK_Jungle => DK_Jungle,
            Battle_DK_Lodge => DK_Lodge,
            Battle_DK_WaterFall => DK_WaterFall,
            Battle_Dolly_Stadium => Dolly_Stadium,
            Battle_Dracula_Castle => Dracula_Castle,
            Battle_DuckHunt => DuckHunt,
            Battle_End => End,
            Battle_FE_Arena => FE_Arena,
            Battle_FE_Colloseum => FE_Colloseum,
            Battle_FE_Shrine => FE_Shrine,
            Battle_FE_Siege => FE_Siege,
            Battle_FF_Cave => FF_Cave,
            Battle_FF_Midgar => FF_Midgar,
            Battle_FlatZoneX => FlatZoneX,
            Battle_Fox_Corneria => Fox_Corneria,
            Battle_Fox_LylatCruise => Fox_LylatCruise,
            Battle_Fox_Venom => Fox_Venom,
            Battle_Fzero_Bigblue => Fzero_Bigblue,
            Battle_Fzero_Mutecity3DS => Fzero_Mutecity3DS,
            Battle_Fzero_Porttown => Fzero_Porttown,
            Battle_Icarus_Angeland => Icarus_Angeland,
            Battle_Icarus_SkyWorld => Icarus_SkyWorld,
            Battle_Icarus_Uprising => Icarus_Uprising,
            Battle_Ice_Top => Ice_Top,
            Battle_Jack_Mementoes => Jack_Mementoes,
            Battle_Kart_CircuitFor => Kart_CircuitFor,
            Battle_Kart_CircuitX => Kart_CircuitX,
            Battle_Kirby_Cave => Kirby_Cave,
            Battle_Kirby_Fountain => Kirby_Fountain,
            Battle_Kirby_Gameboy => Kirby_Gameboy,
            Battle_Kirby_Greens => Kirby_Greens,
            Battle_Kirby_Halberd => Kirby_Halberd,
            Battle_Kirby_Pupupu64 => Kirby_Pupupu64,
            Battle_LuigiMansion => LuigiMansion,
            Battle_Mario_3DLand => Mario_3DLand,
            Battle_Mario_Castle64 => Mario_Castle64,
            Battle_Mario_CastleDx => Mario_CastleDx,
            Battle_Mario_Dolpic => Mario_Dolpic,
            Battle_Mario_Galaxy => Mario_Galaxy,
            Battle_Mario_Maker => Mario_Maker,
            Battle_Mario_NewBros2 => Mario_NewBros2,
            Battle_Mario_Odyssey => Mario_Odyssey,
            Battle_Mario_Paper => Mario_Paper,
            Battle_Mario_Past64 => Mario_Past64,
            Battle_Mario_PastUsa => Mario_PastUsa,
            Battle_Mario_PastX => Mario_PastX,
            Battle_Mario_Rainbow => Mario_Rainbow,
            Battle_Mario_Uworld => Mario_Uworld,
            Battle_MarioBros => MarioBros,
            Battle_Metroid_Kraid => Metroid_Kraid,
            Battle_Metroid_Norfair => Metroid_Norfair,
            Battle_Metroid_Orpheon => Metroid_Orpheon,
            Battle_Metroid_ZebesDx => Metroid_ZebesDx,
            Battle_MG_Shadowmoses => MG_Shadowmoses,
            Battle_Mother_Fourside => Mother_Fourside,
            Battle_Mother_Magicant => Mother_Magicant,
            Battle_Mother_Newpork => Mother_Newpork,
            Battle_Mother_Onett => Mother_Onett,
            Battle_Nintendogs => Nintendogs,
            Battle_Pac_Land => Pac_Land,
            Battle_Pickel_World => Pickel_World,
            Battle_Pictochat2 => Pictochat2,
            Battle_Pikmin_Garden => Pikmin_Garden,
            Battle_Pikmin_Planet => Pikmin_Planet,
            Battle_Pilotwings => Pilotwings,
            Battle_Plankton => Plankton,
            Battle_Poke_Kalos => Poke_Kalos,
            Battle_Poke_Stadium => Poke_Stadium,
            Battle_Poke_Stadium2 => Poke_Stadium2,
            Battle_Poke_Tengam => Poke_Tengam,
            Battle_Poke_Tower => Poke_Tower,
            Battle_Poke_Unova => Poke_Unova,
            Battle_Poke_Yamabuki => Poke_Yamabuki,
            Battle_PunchOutSB => PunchOutSB,
            Battle_PunchOutW => PunchOutW,
            Battle_Rock_Wily => Rock_Wily,
            Battle_SF_Suzaku => SF_Suzaku,
            Battle_Sonic_Greenhill => Sonic_Greenhill,
            Battle_Sonic_Windyhill => Sonic_Windyhill,
            Battle_Spla_Parking => Spla_Parking,
            Battle_StreetPass => StreetPass,
            Battle_Tantan_Spring => Tantan_Spring,
            Battle_Tomodachi => Tomodachi,
            Battle_Trail_Castle => Trail_Castle,
            Battle_Wario_Gamer => Wario_Gamer,
            Battle_Wario_Madein => Wario_Madein,
            Battle_WiiFit => WiiFit,
            Battle_WreckingCrew => WreckingCrew,
            Battle_WufuIsland => WufuIsland,
            Battle_Xeno_Alst => Xeno_Alst,
            Battle_Xeno_Gaur => Xeno_Gaur,
            Battle_Yoshi_CartBoard => Yoshi_CartBoard,
            Battle_Yoshi_Island => Yoshi_Island,
            Battle_Yoshi_Story => Yoshi_Story,
            Battle_Yoshi_Yoster => Yoshi_Yoster,
            Battle_Zelda_Gerudo => Zelda_Gerudo,
            Battle_Zelda_Greatbay => Zelda_Greatbay,
            Battle_Zelda_Hyrule => Zelda_Hyrule,
            Battle_Zelda_Oldin => Zelda_Oldin,
            Battle_Zelda_Pirates => Zelda_Pirates,
            Battle_Zelda_Skyward => Zelda_Skyward,
            Battle_Zelda_Temple => Zelda_Temple,
            Battle_Zelda_Tower => Zelda_Tower,
            Battle_Zelda_Train => Zelda_Train,
            End_75m => _75m,
            End_Animal_City => Animal_City,
            End_Animal_Island => Animal_Island,
            End_Animal_Village => Animal_Village,
            End_BalloonFight => BalloonFight,
            End_BattleField => BattleField,
            End_Bayo_Clock => Bayo_Clock,
            End_Brave_Altar => Brave_Altar,
            End_Buddy_Spiral => Buddy_Spiral,
            End_Demon_Dojo => Demon_Dojo,
            End_DK_Jungle => DK_Jungle,
            End_DK_Lodge => DK_Lodge,
            End_DK_WaterFall => DK_WaterFall,
            End_Dolly_Stadium => Dolly_Stadium,
            End_Dracula_Castle => Dracula_Castle,
            End_DuckHunt => DuckHunt,
            End_FE_Arena => FE_Arena,
            End_FE_Colloseum => FE_Colloseum,
            End_FE_Shrine => FE_Shrine,
            End_FE_Siege => FE_Siege,
            End_FF_Cave => FF_Cave,
            End_FF_Midgar => FF_Midgar,
            End_FlatZoneX => FlatZoneX,
            End_Fox_Corneria => Fox_Corneria,
            End_Fox_LylatCruise => Fox_LylatCruise,
            End_Fox_Venom => Fox_Venom,
            End_Fzero_Bigblue => Fzero_Bigblue,
            End_Fzero_Mutecity3DS => Fzero_Mutecity3DS,
            End_Fzero_Porttown => Fzero_Porttown,
            End_Icarus_Angeland => Icarus_Angeland,
            End_Icarus_SkyWorld => Icarus_SkyWorld,
            End_Icarus_Uprising => Icarus_Uprising,
            End_Ice_Top => Ice_Top,
            End_Jack_Mementoes => Jack_Mementoes,
            End_Kart_CircuitFor => Kart_CircuitFor,
            End_Kart_CircuitX => Kart_CircuitX,
            End_Kirby_Cave => Kirby_Cave,
            End_Kirby_Fountain => Kirby_Fountain,
            End_Kirby_Gameboy => Kirby_Gameboy,
            End_Kirby_Greens => Kirby_Greens,
            End_Kirby_Halberd => Kirby_Halberd,
            End_Kirby_Pupupu64 => Kirby_Pupupu64,
            End_LuigiMansion => LuigiMansion,
            End_Mario_3DLand => Mario_3DLand,
            End_Mario_Castle64 => Mario_Castle64,
            End_Mario_CastleDx => Mario_CastleDx,
            End_Mario_Dolpic => Mario_Dolpic,
            End_Mario_Galaxy => Mario_Galaxy,
            End_Mario_Maker => Mario_Maker,
            End_Mario_NewBros2 => Mario_NewBros2,
            End_Mario_Odyssey => Mario_Odyssey,
            End_Mario_Paper => Mario_Paper,
            End_Mario_Past64 => Mario_Past64,
            End_Mario_PastUsa => Mario_PastUsa,
            End_Mario_PastX => Mario_PastX,
            End_Mario_Rainbow => Mario_Rainbow,
            End_Mario_Uworld => Mario_Uworld,
            End_MarioBros => MarioBros,
            End_Metroid_Kraid => Metroid_Kraid,
            End_Metroid_Norfair => Metroid_Norfair,
            End_Metroid_Orpheon => Metroid_Orpheon,
            End_Metroid_ZebesDx => Metroid_ZebesDx,
            End_MG_Shadowmoses => MG_Shadowmoses,
            End_Mother_Fourside => Mother_Fourside,
            End_Mother_Magicant => Mother_Magicant,
            End_Mother_Newpork => Mother_Newpork,
            End_Mother_Onett => Mother_Onett,
            End_Nintendogs => Nintendogs,
            End_Pac_Land => Pac_Land,
            End_Pickel_World => Pickel_World,
            End_Pictochat2 => Pictochat2,
            End_Pikmin_Garden => Pikmin_Garden,
            End_Pikmin_Planet => Pikmin_Planet,
            End_Pilotwings => Pilotwings,
            End_Plankton => Plankton,
            End_Poke_Kalos => Poke_Kalos,
            End_Poke_Stadium => Poke_Stadium,
            End_Poke_Stadium2 => Poke_Stadium2,
            End_Poke_Tengam => Poke_Tengam,
            End_Poke_Tower => Poke_Tower,
            End_Poke_Unova => Poke_Unova,
            End_Poke_Yamabuki => Poke_Yamabuki,
            End_PunchOutSB => PunchOutSB,
            End_PunchOutW => PunchOutW,
            End_Rock_Wily => Rock_Wily,
            End_SF_Suzaku => SF_Suzaku,
            End_Sonic_Greenhill => Sonic_Greenhill,
            End_Sonic_Windyhill => Sonic_Windyhill,
            End_Spla_Parking => Spla_Parking,
            End_StreetPass => StreetPass,
            End_Tantan_Spring => Tantan_Spring,
            End_Tomodachi => Tomodachi,
            End_Trail_Castle => Trail_Castle,
            End_Wario_Gamer => Wario_Gamer,
            End_Wario_Madein => Wario_Madein,
            End_WiiFit => WiiFit,
            End_WreckingCrew => WreckingCrew,
            End_WufuIsland => WufuIsland,
            End_Xeno_Alst => Xeno_Alst,
            End_Xeno_Gaur => Xeno_Gaur,
            End_Yoshi_CartBoard => Yoshi_CartBoard,
            End_Yoshi_Island => Yoshi_Island,
            End_Yoshi_Story => Yoshi_Story,
            End_Yoshi_Yoster => Yoshi_Yoster,
            End_Zelda_Gerudo => Zelda_Gerudo,
            End_Zelda_Greatbay => Zelda_Greatbay,
            End_Zelda_Hyrule => Zelda_Hyrule,
            End_Zelda_Oldin => Zelda_Oldin,
            End_Zelda_Pirates => Zelda_Pirates,
            End_Zelda_Skyward => Zelda_Skyward,
            End_Zelda_Temple => Zelda_Temple,
            End_Zelda_Tower => Zelda_Tower,
            End_Zelda_Train => Zelda_Train,
            already_normal => already_normal
        }
    }
}

impl Info {
    pub const fn new() -> Self {
        Self {
            arena_id: AtomicArenaId::new(None),
            remaining_frames: AtomicU32::new(u32::MAX),
            is_match: AtomicBool::new(false),
            stage: AtomicU32::new(Stage::None as u32),
            players: [
                Player::new(),
                Player::new(),
                Player::new(),
                Player::new(),
                Player::new(),
                Player::new(),
                Player::new(),
                Player::new()
            ]
        }
    }

    pub fn arena_id(&self) -> Option<String> {
        self.arena_id.load_string(Ordering::SeqCst)
    }

    pub fn remaining_frames(&self) -> u32 {
        self.remaining_frames.load(Ordering::SeqCst)
    }

    pub fn is_match(&self) -> bool {
        self.is_match.load(Ordering::SeqCst)
    }

    pub fn stage(&self) -> Stage {
        let s = self.stage.load(Ordering::SeqCst);
        if (0..Stage::Max as u32).contains(&s) {
            unsafe {
                core::mem::transmute(s)
            }
        } else {
            Stage::None
        }
    }
}

impl Player {
    pub const fn new() -> Self {
        Self {
            character: AtomicU32::new(Character::None as u32),
            damage: AtomicF32::new(0.),
            stocks: AtomicU32::new(0),
            is_cpu: AtomicBool::new(false),
            skin: AtomicU32::new(0)
        }
    }

    pub fn character(&self) -> Character {
        let c = self.character.load(Ordering::SeqCst);
        if (0..Character::Max as u32).contains(&c) {
            unsafe {
                core::mem::transmute(c)
            }
        } else {
            Character::None
        }
    }

    pub fn damage(&self) -> f32 {
        self.damage.load(Ordering::SeqCst)
    }

    pub fn stocks(&self) -> u32 {
        self.stocks.load(Ordering::SeqCst)
    }

    pub fn is_cpu(&self) -> bool {
        self.is_cpu.load(Ordering::SeqCst)
    }
    pub fn skin(&self) -> u32 {
        self.skin.load(Ordering::SeqCst)
    }
}

#[cfg(test)]
mod shared_tests {
    use super::*;

    #[test]
    fn character_test() {
        // Test Character out of bounds
        let player = Player {
            character: AtomicU32::new(Character::Max as u32),
            ..Player::new()
        };

        // Invalid character
        assert_eq!(player.character(), Character::None);

        let player = Player {
            character: AtomicU32::new(u32::MAX),
            ..Player::new()
        };

        // Invalid character
        assert_eq!(player.character(), Character::None);

        let player = Player {
            character: AtomicU32::new(Character::Zenigame as u32),
            ..Player::new()
        };

        // Valid character
        assert_eq!(player.character(), Character::Zenigame);
    }

    #[test]
    fn stage_test() {
        const TEST_INFO: Info = Info {
            arena_id: AtomicArenaId::new(Some([b'A', b'A', b'A', b'A', b'A'])),
            is_match: AtomicBool::new(true),
            remaining_frames: AtomicU32::new(3),
            stage: AtomicU32::new(Stage::Plankton as u32),
            players: [
                Player::new(),
                Player::new(),
                Player::new(),
                Player::new(),
                Player::new(),
                Player::new(),
                Player::new(),
                Player::new(),
            ]
        };

        assert_eq!(TEST_INFO.stage(), Stage::Plankton);
        assert_eq!(TEST_INFO.remaining_frames(), 3);
        assert!(TEST_INFO.is_match());

        let new_info = Info {
            stage: AtomicU32::new(Stage::Max as u32),
            ..TEST_INFO
        };

        // test invalid state
        assert_eq!(new_info.stage(), Stage::None);

        let new_info = Info {
            stage: AtomicU32::new(u32::MAX),
            ..TEST_INFO
        };

        // test invalid state
        assert_eq!(new_info.stage(), Stage::None);

        let new_info = Info {
            stage: AtomicU32::new(Stage::_75m as u32),
            ..TEST_INFO
        };

        assert_eq!(new_info.stage(), Stage::_75m);
    }
}
