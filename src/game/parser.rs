use std::{
    io, 
    str::FromStr
};

use quick_xml::{
    Reader,
    events::Event,
};

use crate::{
    connection::parser::{
        message::Message, 
        parser_strategy::ParserStrategy
    }, game::{
        board::Board, color::Color, gamestate::GameState, r#move::Move, piece::{ALL_PIECE_TYPES, PieceType}, rotation::Rotation, team::Team
    }
};

pub struct Blokus2026;

impl ParserStrategy for Blokus2026 {
    fn parse_memento(xml: &str) -> Result<Box<Message>, Box<dyn std::error::Error>> {
        let mut starting_piece: Option<PieceType> = None;
        let mut start_team: Option<Team> = None;
        let mut turn: u8 = 0;
        let mut round: u8 = 0;

        let mut reader = Reader::from_str(xml);
        loop {
            match reader.read_event()? {
                Event::Start(e) if e.name().0 == "state"=> {
                    for attr in e.attributes() {
                        let attr = attr?;
                        match attr.key.0 {
                            "startTeam" => {
                                start_team = Team::from_str(attr.value.as_ref()).ok();
                            },
                            "turn" => {
                                turn = u8::from_str(attr.value.as_ref())?;
                            },
                            "startPiece" => {
                                starting_piece = PieceType::from_str(attr.value.as_ref()).ok();
                            }
                            "round" => {
                                round = u8::from_str(attr.value.as_ref())?;
                            },
                            _ => {}
                        }
                    }

                    if turn != 0 {
                        let last_move = parse_last_move(&mut reader)?; 
                        return Ok(Box::from(Message::MementoLastMove(Some(turn), Some(last_move))));
                    } else {
                        //let board = _parse_game_board(&mut reader)?;
                        //let pieces = _parse_pieces(&mut reader)?;
                        let is_statrting_team_one = Team::One == start_team.unwrap_or_else(|| {
                            eprintln!("No start team. Using ONE as fallback!");
                            Team::One
                        });
                        let starting_piece = starting_piece.ok_or(io::Error::new(
                            io::ErrorKind::UnexpectedEof,
                            "No starting piece"
                        ))?;

                        //taken from gamestate
                        const COLOR_ORDER_ONE: [Color; 4] = [Color::Blue, Color::Yellow, Color::Red, Color::Green];
                        const COLOR_ORDER_TWO: [Color; 4] = [Color::Yellow, Color::Red, Color::Green, Color::Blue];
                        let current_turn_color = if is_statrting_team_one {
                            COLOR_ORDER_ONE[(turn % 4) as usize]
                        } else {
                           COLOR_ORDER_TWO[(turn % 4) as usize]
                        };

                        return Ok(Box::from(Message::MementoInitial(Some(GameState::new(starting_piece, is_statrting_team_one, Board::new(), turn, round, current_turn_color, ALL_PIECE_TYPES.to_vec(), ALL_PIECE_TYPES.to_vec(), ALL_PIECE_TYPES.to_vec(), ALL_PIECE_TYPES.to_vec())))));
                    }
                },
                Event::Start(e) if e.name().0 == "lastMove" => {

                }
                Event::Eof => return Err(Box::from(io::Error::new(io::ErrorKind::UnexpectedEof,"No valid memento!"))),
                _=> ()
            }
        }
    }
}   

fn parse_last_move(reader: &mut Reader<&[u8]>) -> Result<Move, Box<dyn std::error::Error>> {
    loop {
        match reader.read_event()? {
            Event::Start(l) if l.name().0 == "lastMove" =>  {
                for attr in l.attributes() {
                    let attr = attr?;
                    if attr.key.0 == "class" {
                        if attr.value.as_ref() == "sc.plugin2027.SetMove" {
                            return parse_set_move(reader);
                        }
                        else if attr.value.as_ref() == "sc.plugin2027.SkipMove" {
                            return parse_skip_move(reader);
                        }
                    } 
                }
            },
            Event::Eof => {
                return Err(Box::from(io::Error::new(io::ErrorKind::UnexpectedEof,"No lastMove received")));
            },
            _ => {}
        }
    }
}

fn parse_skip_move(reader: &mut Reader<&[u8]>) -> Result<Move, Box<dyn std::error::Error>> {
    let mut next_text_is_color = false;
    loop {
        match reader.read_event()? {
            Event::Start(c) if c.name().0 == "color" =>  next_text_is_color = true,
            Event::Text(t) if next_text_is_color=> {
                let color = Color::from_string(&t);
                return Ok(Move::new(color, PieceType::Domino, 0, 0, false, Rotation::None, true));
            }
            Event::Eof => {
                return Err(Box::from(io::Error::new(io::ErrorKind::UnexpectedEof,"Turn is not 0 but no lastMove received")));
            },
            _ => {}
        }
    }
}

fn parse_set_move(reader: &mut Reader<&[u8]>) -> Result<Move, Box<dyn std::error::Error>> {
    struct PieceData {
        color: Color,
        kind: PieceType,
        rotation: Rotation,
        is_flipped: bool
    }

    let mut piece_data: Option<PieceData> = None;
    loop {
        match reader.read_event()? {
            Event::Start(p) if p.name().0 == "piece" => {
                let mut color: Option<Color> = None;
                let mut piece_type: Option<PieceType> = None;
                let mut rotation: Option<Rotation> = None;
                let mut is_flipped: Option<bool> = None;
                for attr in p.attributes() {
                    let attr = attr?;
                    match attr.key.0 {
                        "color" => {color = Some(Color::from_string(attr.value.as_ref()))}
                        "kind" => {piece_type = Some(PieceType::from_str(attr.value.as_ref()).map_err(|_| 
                        io::Error::new(io::ErrorKind::InvalidData,"Received turn data but no class"
                        ))?)}
                        "rotation" => {rotation = Some(Rotation::from_string(attr.value.as_ref())?)}
                        "isFlipped" => {is_flipped = Some(bool::from_str(attr.value.as_ref())?)}
                        _ => {}
                    }
                }
                if let (Some(c), Some(k), Some(r), Some(f)) = (color, piece_type, rotation, is_flipped) {
                    piece_data = Some(PieceData {
                        color: c,
                        kind: k,
                        rotation: r,
                        is_flipped: f, 
                    })
                } else {
                    return Err(Box::from(io::Error::new(
                        io::ErrorKind::NotFound,
                        format!("Missing piece data. Unable to form Move. Received: color: {:?}, kind: {:?}, rotation: {:?}, is_flipped: {:?}.", color, piece_type, rotation, is_flipped)
                    )));
                }
            },
            Event::Empty(p) if p.name().0 == "position" => {
                let mut x = None;
                let mut y = None;
                for attr in p.attributes() {
                    let attr = attr?;
                    match attr.key.0 {
                        "x" => {x = Some(usize::from_str(attr.value.as_ref())?)}
                        "y" => {y = Some(usize::from_str(attr.value.as_ref())?)}
                        _ => {}
                    }
                }
                if let (Some(x), Some(y)) = (x, y) {
                    let piece = piece_data.ok_or(io::Error::new(
                        io::ErrorKind::NotFound,
                        "Received no piece data. Unable to form Move"
                    ))?;
                    return Ok(Move::new(piece.color, piece.kind, x, y, piece.is_flipped, piece.rotation, false));
                } else {
                    return Err(Box::from(io::Error::new(
                        io::ErrorKind::NotFound,
                        format!("Missing position data. Unable to form Move. Received: x: {:?}, y: {:?}.", x, y)
                    )));
                }
            },
            Event::Eof => {
                return Err(Box::from(io::Error::new(io::ErrorKind::UnexpectedEof,"Turn is not 0 but no lastMove received")));
            },
            _ => {}
        }
    }
}

fn _parse_game_board(reader: &mut Reader<&[u8]>) -> Result<Board, Box<dyn std::error::Error>> {
    let mut board = Board::new();
    loop {
        match reader.read_event()? {
            Event::Start(f) if f.name().0 == "field"=> {
                let mut x: Option<usize> = None;
                let mut y: Option<usize> = None;
                let mut content: Option<Color> = None;
                for attr in f.attributes() {
                    let attr = attr?;
                    match attr.key.0 {
                        "x" => {x = Some(usize::from_str(attr.value.as_ref())?)}
                        "y" => {y = Some(usize::from_str(attr.value.as_ref())?)}
                        "content" => {content = Some(Color::from_string(attr.value.as_ref()))}
                        _ => {}
                    }
                }
                if let (Some(x), Some(y), Some(color)) = (x, y, content) {
                    let _ = board.set_cell(x, y, color);
                    } else {
                        eprintln!("Missing field data. Skipping current field. Received: x: {:?}, y: {:?}, content: {:?}.", x, y, content)
                }
            },
            Event::Empty(e) if e.name().0 == "board" => break,
            Event::End(e) if e.name().0 == "board" => break,
            Event::Eof => {
                return Err(Box::from(io::Error::new(io::ErrorKind::UnexpectedEof,"Board not received, or only partially received.")));

            },
            _ => {}
        }
    }
    Ok(board)
}

// blue, yellow, red, green
fn _parse_pieces(reader: &mut Reader<&[u8]>) -> Result<(Vec<PieceType>, Vec<PieceType>, Vec<PieceType>, Vec<PieceType>), Box<dyn std::error::Error>> {
    let mut blue_pieces =  Vec::new();
    let mut yellow_pieces =  Vec::new();
    let mut red_pieces =  Vec::new();
    let mut green_pieces =  Vec::new();

    let mut current_colore = Color::Blue;
    let mut exit_counter = 0;
    loop {
        match reader.read_event()? {
            Event::Text(t) if !t.trim().is_empty() => {
                let piece = PieceType::from_str(t.as_ref()).map_err(|_| 
                        io::Error::new(io::ErrorKind::InvalidData,format!("Cannot create PieceType from: {:?}", t)
                ))?;
                match current_colore {
                    Color::Blue => blue_pieces.push(piece),
                    Color::Yellow => yellow_pieces.push(piece),
                    Color::Red => red_pieces.push(piece),
                    Color::Green => green_pieces.push(piece),
                };
            }
            Event::Start(b) if b.name().0 == "blueShapes" => {
                current_colore = Color::Blue;
            },
            Event::Start(y) if y.name().0 == "yellowShapes" => {
                current_colore = Color::Yellow;
            },
             Event::Start(r) if r.name().0 == "redShapes" => {
                current_colore = Color::Red;
            },
            Event::Start(g) if g.name().0 == "greenShapes" => {
                current_colore = Color::Green;
            },
            Event::End(e) if e.name().0 != "shape" => {
                exit_counter += 1;
                if exit_counter >= 4 {break;}
            }
            Event::Eof => {
                return Err(Box::from(io::Error::new(io::ErrorKind::UnexpectedEof,"Pieces not received, or only partially received.")));
            },
            _ => {}
        }
    }
    Ok((blue_pieces, yellow_pieces, red_pieces, green_pieces))
}

#[cfg(test)]
mod tests {
    use crate::connection::parser::message::Message;
use crate::connection::parser::parser_strategy::ParserStrategy;
use crate::game::piece::ALL_PIECE_TYPES;
use crate::game::piece::PieceType::PentoW;
use crate::game::rotation::Rotation::None;
use crate::game::{
        board::Board,
        color::Color,
        gamestate::GameState,
        piece::PieceType,
        r#move::Move,
    };

    use super::Blokus2026;

    #[test]
    fn parse_memento_initial() {
        let xml = br#"
        PieceType<room roomId="ffcd302c-e191-4257-9104-00177dc490ca">
          <data class="memento">
            <state class="state" startTeam="ONE" turn="0" startPiece="PENTO_W" round="1">
              <board/>
              <lastMoveMono/>
              <blueShapes>
                <shape>MONO</shape>
                <shape>DOMINO</shape>
                <shape>TRIO_L</shape>
                <shape>TRIO_I</shape>
                <shape>TETRO_O</shape>
                <shape>TETRO_T</shape>
                <shape>TETRO_I</shape>
                <shape>TETRO_L</shape>
                <shape>TETRO_Z</shape>
                <shape>PENTO_L</shape>
                <shape>PENTO_T</shape>
                <shape>PENTO_V</shape>
                <shape>PENTO_S</shape>
                <shape>PENTO_Z</shape>
                <shape>PENTO_I</shape>
                <shape>PENTO_P</shape>
                <shape>PENTO_W</shape>
                <shape>PENTO_U</shape>
                <shape>PENTO_R</shape>
                <shape>PENTO_X</shape>
                <shape>PENTO_Y</shape>
              </blueShapes>
              <yellowShapes>
                <shape>MONO</shape>
                <shape>DOMINO</shape>
                <shape>TRIO_L</shape>
                <shape>TRIO_I</shape>
                <shape>TETRO_O</shape>
                <shape>TETRO_T</shape>
                <shape>TETRO_I</shape>
                <shape>TETRO_L</shape>
                <shape>TETRO_Z</shape>
                <shape>PENTO_L</shape>
                <shape>PENTO_T</shape>
                <shape>PENTO_V</shape>
                <shape>PENTO_S</shape>
                <shape>PENTO_Z</shape>
                <shape>PENTO_I</shape>
                <shape>PENTO_P</shape>
                <shape>PENTO_W</shape>
                <shape>PENTO_U</shape>
                <shape>PENTO_R</shape>
                <shape>PENTO_X</shape>
                <shape>PENTO_Y</shape>
              </yellowShapes>
              <redShapes>
                <shape>MONO</shape>
                <shape>DOMINO</shape>
                <shape>TRIO_L</shape>
                <shape>TRIO_I</shape>
                <shape>TETRO_O</shape>
                <shape>TETRO_T</shape>
                <shape>TETRO_I</shape>
                <shape>TETRO_L</shape>
                <shape>TETRO_Z</shape>
                <shape>PENTO_L</shape>
                <shape>PENTO_T</shape>
                <shape>PENTO_V</shape>
                <shape>PENTO_S</shape>
                <shape>PENTO_Z</shape>
                <shape>PENTO_I</shape>
                <shape>PENTO_P</shape>
                <shape>PENTO_W</shape>
                <shape>PENTO_U</shape>
                <shape>PENTO_R</shape>
                <shape>PENTO_X</shape>
                <shape>PENTO_Y</shape>
              </redShapes>
              <greenShapes>
                <shape>MONO</shape>
                <shape>DOMINO</shape>
                <shape>TRIO_L</shape>
                <shape>TRIO_I</shape>
                <shape>TETRO_O</shape>
                <shape>TETRO_T</shape>
                <shape>TETRO_I</shape>
                <shape>TETRO_L</shape>
                <shape>TETRO_Z</shape>
                <shape>PENTO_L</shape>
                <shape>PENTO_T</shape>
                <shape>PENTO_V</shape>
                <shape>PENTO_S</shape>
                <shape>PENTO_Z</shape>
                <shape>PENTO_I</shape>
                <shape>PENTO_P</shape>
                <shape>PENTO_W</shape>
                <shape>PENTO_U</shape>
                <shape>PENTO_R</shape>
                <shape>PENTO_X</shape>
                <shape>PENTO_Y</shape>
              </greenShapes>
              <validColors>
                <color>BLUE</color>
                <color>YELLOW</color>
                <color>RED</color>
                <color>GREEN</color>
              </validColors>
            </state>
          </data>
        </room>"#;

        let gamestate = Blokus2026::parse_memento(&String::from_utf8(xml.to_vec()).unwrap()).unwrap();

        println!("{:?}", gamestate);

        let gamestate_target = Message::MementoInitial(Some(GameState::new(PieceType::PentoW, true, Board::new(), 0, 1, Color::Blue, ALL_PIECE_TYPES.to_vec(), ALL_PIECE_TYPES.to_vec(), ALL_PIECE_TYPES.to_vec(), ALL_PIECE_TYPES.to_vec())));

        println!("{:?}", gamestate_target);

        assert!(*gamestate == gamestate_target, "Gamestates are not the same")
    }

        #[test]
    fn parse_memento_last_move() {
        let xml = br#"
            <room roomId="ffcd302c-e191-4257-9104-00177dc490ca">
              <data class="memento">
                <state class="state" startTeam="ONE" turn="1" startPiece="PENTO_W" round="1">
                  <lastMove class="sc.plugin2027.SetMove">
                    <piece color="BLUE" kind="PENTO_W" rotation="NONE" isFlipped="true">
                      <position x="0" y="16"/>
                    </piece>
                  </lastMove>
                  <board>
                    <field x="2" y="16" content="BLUE"/>
                    <field x="1" y="17" content="BLUE"/>
                    <field x="2" y="17" content="BLUE"/>
                    <field x="0" y="18" content="BLUE"/>
                    <field x="1" y="18" content="BLUE"/>
                  </board>
                  <lastMoveMono/>
                  <blueShapes>
                    <shape>MONO</shape>
                    <shape>DOMINO</shape>
                    <shape>TRIO_L</shape>
                    <shape>TRIO_I</shape>
                    <shape>TETRO_O</shape>
                    <shape>TETRO_T</shape>
                    <shape>TETRO_I</shape>
                    <shape>TETRO_L</shape>
                    <shape>TETRO_Z</shape>
                    <shape>PENTO_L</shape>
                    <shape>PENTO_T</shape>
                    <shape>PENTO_V</shape>
                    <shape>PENTO_S</shape>
                    <shape>PENTO_Z</shape>
                    <shape>PENTO_I</shape>
                    <shape>PENTO_P</shape>
                    <shape>PENTO_U</shape>
                    <shape>PENTO_R</shape>
                    <shape>PENTO_X</shape>
                    <shape>PENTO_Y</shape>
                  </blueShapes>
                  <yellowShapes>
                    <shape>MONO</shape>
                    <shape>DOMINO</shape>
                    <shape>TRIO_L</shape>
                    <shape>TRIO_I</shape>
                    <shape>TETRO_O</shape>
                    <shape>TETRO_T</shape>
                    <shape>TETRO_I</shape>
                    <shape>TETRO_L</shape>
                    <shape>TETRO_Z</shape>
                    <shape>PENTO_L</shape>
                    <shape>PENTO_T</shape>
                    <shape>PENTO_V</shape>
                    <shape>PENTO_S</shape>
                    <shape>PENTO_Z</shape>
                    <shape>PENTO_I</shape>
                    <shape>PENTO_P</shape>
                    <shape>PENTO_W</shape>
                    <shape>PENTO_U</shape>
                    <shape>PENTO_R</shape>
                    <shape>PENTO_X</shape>
                    <shape>PENTO_Y</shape>
                  </yellowShapes>
                  <redShapes>
                    <shape>MONO</shape>
                    <shape>DOMINO</shape>
                    <shape>TRIO_L</shape>
                    <shape>TRIO_I</shape>
                    <shape>TETRO_O</shape>
                    <shape>TETRO_T</shape>
                    <shape>TETRO_I</shape>
                    <shape>TETRO_L</shape>
                    <shape>TETRO_Z</shape>
                    <shape>PENTO_L</shape>
                    <shape>PENTO_T</shape>
                    <shape>PENTO_V</shape>
                    <shape>PENTO_S</shape>
                    <shape>PENTO_Z</shape>
                    <shape>PENTO_I</shape>
                    <shape>PENTO_P</shape>
                    <shape>PENTO_W</shape>
                    <shape>PENTO_U</shape>
                    <shape>PENTO_R</shape>
                    <shape>PENTO_X</shape>
                    <shape>PENTO_Y</shape>
                  </redShapes>
                  <greenShapes>
                    <shape>MONO</shape>
                    <shape>DOMINO</shape>
                    <shape>TRIO_L</shape>
                    <shape>TRIO_I</shape>
                    <shape>TETRO_O</shape>
                    <shape>TETRO_T</shape>
                    <shape>TETRO_I</shape>
                    <shape>TETRO_L</shape>
                    <shape>TETRO_Z</shape>
                    <shape>PENTO_L</shape>
                    <shape>PENTO_T</shape>
                    <shape>PENTO_V</shape>
                    <shape>PENTO_S</shape>
                    <shape>PENTO_Z</shape>
                    <shape>PENTO_I</shape>
                    <shape>PENTO_P</shape>
                    <shape>PENTO_W</shape>
                    <shape>PENTO_U</shape>
                    <shape>PENTO_R</shape>
                    <shape>PENTO_X</shape>
                    <shape>PENTO_Y</shape>
                  </greenShapes>
                  <validColors>
                    <color>BLUE</color>
                    <color>YELLOW</color>
                    <color>RED</color>
                    <color>GREEN</color>
                  </validColors>
                </state>
              </data>
            </room>"#;

        let last_move = Blokus2026::parse_memento(&String::from_utf8(xml.to_vec()).unwrap()).unwrap();

        println!("{:?}", last_move);

        let last_movetarget = Message::MementoLastMove(Some(1), Some(Move::new(Color::Blue, PentoW, 0, 16, true, None, false)));

        println!("{:?}", last_movetarget);

        assert!(*last_move == last_movetarget, "Last moves are not the same")
    }
}       