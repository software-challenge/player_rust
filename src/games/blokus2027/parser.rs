#[cfg(test)]
#[path = "tests/parser.rs"]
mod tests;

use std::{
    io, 
    str::FromStr
};

use quick_xml::{
    Reader,
    events::Event,
};

use crate::{
    common::team::Team, 
    connection::parser::parser_strategy::{MementoReturn, ParserStrategy}, 
    games::blokus2027::{
        board::Board, 
        color::Color, 
        gamestate::GameState, 
        r#move::{Move, SetMove}, 
        piece::{ALL_PIECE_TYPES, PieceType}, 
        rotation::Rotation,
    }
};

pub struct Blokus2027;

impl ParserStrategy for Blokus2027 {
    fn parse_memento(xml: &str) -> Result<MementoReturn, Box<dyn std::error::Error>> {
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
                        return Ok(MementoReturn::LastMove(turn, last_move));
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
                        #[allow(clippy::indexing_slicing)]
                        let current_turn_color = if is_statrting_team_one {
                            COLOR_ORDER_ONE[(turn % 4) as usize]
                        } else {
                           COLOR_ORDER_TWO[(turn % 4) as usize]
                        };      

                        return Ok(MementoReturn::GameState(Box::new(GameState::new(starting_piece, is_statrting_team_one, Board::new(), turn, round, current_turn_color, ALL_PIECE_TYPES.to_vec(), ALL_PIECE_TYPES.to_vec(), ALL_PIECE_TYPES.to_vec(), ALL_PIECE_TYPES.to_vec()))));
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
            Event::Text(text) if next_text_is_color => {
                let color = Color::try_from(text.as_ref()).expect(format!("Server returned invalid color: {}", text.as_ref()).as_str());
                return Ok(Move::Skip(color));
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
                        "color" => {color = Some(Color::try_from(attr.value.as_ref()).expect(format!("Server returned invalid color: {}", attr.value).as_str()))}
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
                    return Ok(Move::Set(SetMove::new(piece.color, piece.kind, x, y, piece.is_flipped, piece.rotation)));
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
                        "content" => {content = Some(Color::try_from(attr.value.as_ref()).expect(format!("Server returned invalid color: {}", attr.value).as_str()))}
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
#[allow(clippy::type_complexity)]
fn _parse_pieces(reader: &mut Reader<&[u8]>) -> Result<(Vec<PieceType>, Vec<PieceType>, Vec<PieceType>, Vec<PieceType>), Box<dyn std::error::Error>> {
    let mut pieces = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];

    let mut current_color = Color::Blue;
    let mut exit_counter = 0;
    loop {
        match reader.read_event()? {
            Event::Text(t) if !t.trim().is_empty() => {
                let piece = PieceType::from_str(t.as_ref()).map_err(|_| 
                        io::Error::new(io::ErrorKind::InvalidData,format!("Cannot create PieceType from: {:?}", t)
                ))?;
                #[allow(clippy::indexing_slicing)]
                pieces[current_color.as_index()].push(piece);
            }
            Event::Start(b) if b.name().0 == "blueShapes" => {
                current_color = Color::Blue;
            },
            Event::Start(y) if y.name().0 == "yellowShapes" => {
                current_color = Color::Yellow;
            },
             Event::Start(r) if r.name().0 == "redShapes" => {
                current_color = Color::Red;
            },
            Event::Start(g) if g.name().0 == "greenShapes" => {
                current_color = Color::Green;
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
    Ok((pieces[0].clone(), pieces[1].clone(), pieces[2].clone(), pieces[3].clone()))
}
