use eframe::egui::{self, Color32, Vec2};

#[derive(Clone)]
#[allow(dead_code)]
pub struct RobloxGame {
    pub name: &'static str,
    pub place_id: &'static str,
    pub universe_id: Option<u64>,
    pub color: Color32,
    pub icon_letter: char,
    pub category: GameCategory,
}

#[derive(Clone, Copy, PartialEq)]
pub enum GameCategory {
    Fighting,
    Roleplay,
    Simulator,
    Horror,
    Obby,
    Classic,
    Sports,
}

impl GameCategory {
    #[allow(dead_code)]
    pub fn label(&self) -> &'static str {
        match self {
            GameCategory::Fighting => "Fighting",
            GameCategory::Roleplay => "Roleplay",
            GameCategory::Simulator => "Simulator",
            GameCategory::Horror => "Horror",
            GameCategory::Obby => "Obby",
            GameCategory::Classic => "Classic",
            GameCategory::Sports => "Sports",
        }
    }
}

pub const POPULAR_GAMES: &[RobloxGame] = &[
    RobloxGame {
        name: "Rivals",
        place_id: "17625359962",
        universe_id: Some(5765098040),
        color: Color32::from_rgb(220, 50, 50),
        icon_letter: 'R',
        category: GameCategory::Fighting,
    },
    RobloxGame {
        name: "Arsenal",
        place_id: "286090429",
        universe_id: Some(115797352),
        color: Color32::from_rgb(255, 140, 0),
        icon_letter: 'A',
        category: GameCategory::Fighting,
    },
    RobloxGame {
        name: "Da Hood",
        place_id: "2788229376",
        universe_id: Some(982192655),
        color: Color32::from_rgb(80, 80, 80),
        icon_letter: 'D',
        category: GameCategory::Fighting,
    },
    RobloxGame {
        name: "Murder Mystery 2",
        place_id: "142823291",
        universe_id: Some(66654135),
        color: Color32::from_rgb(180, 0, 0),
        icon_letter: 'M',
        category: GameCategory::Fighting,
    },
    RobloxGame {
        name: "The Strongest Battlegrounds",
        place_id: "10449761463",
        universe_id: Some(4322956471),
        color: Color32::from_rgb(255, 215, 0),
        icon_letter: 'T',
        category: GameCategory::Fighting,
    },
    RobloxGame {
        name: "Blade Ball",
        place_id: "13772394625",
        universe_id: Some(4768823801),
        color: Color32::from_rgb(0, 200, 255),
        icon_letter: 'B',
        category: GameCategory::Fighting,
    },
    
    // Sports
    RobloxGame {
        name: "PUGG 2.0 (Football)",
        place_id: "6872274481",
        universe_id: Some(2561338969),
        color: Color32::from_rgb(0, 180, 80),
        icon_letter: 'P',
        category: GameCategory::Sports,
    },
    RobloxGame {
        name: "Super League Soccer",
        place_id: "3638751429",
        universe_id: Some(1285044498),
        color: Color32::from_rgb(50, 150, 50),
        icon_letter: 'S',
        category: GameCategory::Sports,
    },
    
    // Roleplay
    RobloxGame {
        name: "Brookhaven RP",
        place_id: "4924922222",
        universe_id: Some(1685831367),
        color: Color32::from_rgb(100, 180, 255),
        icon_letter: 'B',
        category: GameCategory::Roleplay,
    },
    RobloxGame {
        name: "Adopt Me!",
        place_id: "920587237",
        universe_id: Some(347725483),
        color: Color32::from_rgb(255, 100, 180),
        icon_letter: 'A',
        category: GameCategory::Roleplay,
    },
    RobloxGame {
        name: "Bloxburg",
        place_id: "185655149",
        universe_id: Some(78633063),
        color: Color32::from_rgb(50, 200, 100),
        icon_letter: 'B',
        category: GameCategory::Roleplay,
    },
    RobloxGame {
        name: "MeepCity",
        place_id: "370731277",
        universe_id: Some(134371802),
        color: Color32::from_rgb(255, 200, 50),
        icon_letter: 'M',
        category: GameCategory::Roleplay,
    },
    RobloxGame {
        name: "Royal High",
        place_id: "735030788",
        universe_id: Some(288776020),
        color: Color32::from_rgb(200, 100, 255),
        icon_letter: 'R',
        category: GameCategory::Roleplay,
    },
    RobloxGame {
        name: "Jailbreak",
        place_id: "606849621",
        universe_id: Some(234658639),
        color: Color32::from_rgb(255, 180, 0),
        icon_letter: 'J',
        category: GameCategory::Roleplay,
    },
    
    // Simulators
    RobloxGame {
        name: "Blox Fruits",
        place_id: "2753915549",
        universe_id: Some(947803620),
        color: Color32::from_rgb(100, 200, 255),
        icon_letter: 'B',
        category: GameCategory::Simulator,
    },
    RobloxGame {
        name: "Pet Simulator 99",
        place_id: "8737899170",
        universe_id: Some(3455815032),
        color: Color32::from_rgb(255, 220, 100),
        icon_letter: 'P',
        category: GameCategory::Simulator,
    },
    RobloxGame {
        name: "Bee Swarm Simulator",
        place_id: "1537690962",
        universe_id: Some(573073214),
        color: Color32::from_rgb(255, 200, 0),
        icon_letter: 'B',
        category: GameCategory::Simulator,
    },
    RobloxGame {
        name: "Anime Defenders",
        place_id: "16433827671",
        universe_id: Some(5749823839),
        color: Color32::from_rgb(255, 100, 100),
        icon_letter: 'A',
        category: GameCategory::Simulator,
    },
    
    // Horror
    RobloxGame {
        name: "Piggy",
        place_id: "4623386862",
        universe_id: Some(1599193454),
        color: Color32::from_rgb(255, 150, 180),
        icon_letter: 'P',
        category: GameCategory::Horror,
    },
    RobloxGame {
        name: "Doors",
        place_id: "6516141723",
        universe_id: Some(2446866959),
        color: Color32::from_rgb(60, 60, 80),
        icon_letter: 'D',
        category: GameCategory::Horror,
    },
    RobloxGame {
        name: "Apeirophobia",
        place_id: "9508087919",
        universe_id: Some(3742710978),
        color: Color32::from_rgb(200, 200, 100),
        icon_letter: 'A',
        category: GameCategory::Horror,
    },
    
    // Obby
    RobloxGame {
        name: "Tower of Hell",
        place_id: "1962086868",
        universe_id: Some(703124385),
        color: Color32::from_rgb(255, 50, 50),
        icon_letter: 'T',
        category: GameCategory::Obby,
    },
    RobloxGame {
        name: "Obby Creator",
        place_id: "2352458588",
        universe_id: Some(815606015),
        color: Color32::from_rgb(255, 100, 0),
        icon_letter: 'O',
        category: GameCategory::Obby,
    },
    
    // Classic
    RobloxGame {
        name: "Natural Disaster Survival",
        place_id: "189707",
        universe_id: Some(101296363),
        color: Color32::from_rgb(100, 150, 100),
        icon_letter: 'N',
        category: GameCategory::Classic,
    },
    RobloxGame {
        name: "Work at a Pizza Place",
        place_id: "192800",
        universe_id: Some(97445377),
        color: Color32::from_rgb(200, 100, 50),
        icon_letter: 'W',
        category: GameCategory::Classic,
    },
    RobloxGame {
        name: "Classic Baseplate",
        place_id: "95206881",
        universe_id: None,
        color: Color32::from_rgb(100, 100, 100),
        icon_letter: 'C',
        category: GameCategory::Classic,
    },
    RobloxGame {
        name: "Crossroads",
        place_id: "1818",
        universe_id: Some(127167484),
        color: Color32::from_rgb(80, 120, 80),
        icon_letter: 'X',
        category: GameCategory::Classic,
    },
];

#[allow(dead_code)]
pub fn draw_game_icon(ui: &mut egui::Ui, game: &RobloxGame, size: f32) {
    let (rect, _response) = ui.allocate_exact_size(Vec2::splat(size), egui::Sense::hover());
    
    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        
        painter.rect_filled(
            rect,
            egui::Rounding::same(4.0),
            game.color,
        );
        
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            game.icon_letter,
            egui::FontId::proportional(size * 0.6),
            Color32::WHITE,
        );
    }
}
