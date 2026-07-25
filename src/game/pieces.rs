#[derive(Copy, Clone)]
pub enum Pieces {
    Mono,
    Domino,
    TrioL,
    TrioI,
    TetroO,
    TetroT,
    TetroI,
    TetroL,
    TetroZ,
    PentoL,
    PentoT,
    PentoV,
    PentoS,
    PentoZ,
    PentoI,
    PentoP,
    PentoW,
    PentoU,
    PentoR,
    PentoX,
    PentoY,
}

impl Pieces {
    pub fn from_string(piece_string: &str) -> Self {
        match piece_string {
            "MONO" => Pieces::Mono,
            "DOMINO" => Pieces::Domino,
            "TRIO_L" => Pieces::TrioL,
            "TRIO_I" => Pieces::TrioI,
            "TETRO_O" => Pieces::TetroO,
            "TETRO_T" => Pieces::TetroT,
            "TETRO_I" => Pieces::TetroI,
            "TETRO_L" => Pieces::TetroL,
            "TETRO_Z" => Pieces::TetroZ,
            "PENTO_L" => Pieces::PentoL,
            "PENTO_T" => Pieces::PentoT,
            "PENTO_V" => Pieces::PentoV,
            "PENTO_S" => Pieces::PentoS,
            "PENTO_Z" => Pieces::PentoZ,
            "PENTO_I" => Pieces::PentoI,
            "PENTO_P" => Pieces::PentoP,
            "PENTO_W" => Pieces::PentoW,
            "PENTO_U" => Pieces::PentoU,
            "PENTO_R" => Pieces::PentoR,
            "PENTO_X" => Pieces::PentoX,
            "PENTO_Y" => Pieces::PentoY,
        
        _ => panic!("Invalid piece string"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Pieces::Mono => "MONO".to_string(),
            Pieces::Domino => "DOMINO".to_string(),
            Pieces::TrioL => "TRIO_L".to_string(),
            Pieces::TrioI => "TRIO_I".to_string(),
            Pieces::TetroO => "TETRO_O".to_string(),
            Pieces::TetroT => "TETRO_T".to_string(),
            Pieces::TetroI => "TETRO_I".to_string(),
            Pieces::TetroL => "TETRO_L".to_string(),
            Pieces::TetroZ => "TETRO_Z".to_string(),
            Pieces::PentoL => "PENTO_L".to_string(),
            Pieces::PentoT => "PENTO_T".to_string(),
            Pieces::PentoV => "PENTO_V".to_string(),
            Pieces::PentoS => "PENTO_S".to_string(),
            Pieces::PentoZ => "PENTO_Z".to_string(),
            Pieces::PentoI => "PENTO_I".to_string(),
            Pieces::PentoP => "PENTO_P".to_string(),
            Pieces::PentoW => "PENTO_W".to_string(),
            Pieces::PentoU => "PENTO_U".to_string(),
            Pieces::PentoR => "PENTO_R".to_string(),
            Pieces::PentoX => "PENTO_X".to_string(),
            Pieces::PentoY => "PENTO_Y".to_string(),
        }
    }
}