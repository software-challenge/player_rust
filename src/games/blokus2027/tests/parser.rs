use super::Blokus2027;

use crate::connection::parser::parser_strategy::{MementoReturn, ParserStrategy};
use crate::games::blokus2027::r#move::SetMove;
use crate::games::blokus2027::piece::ALL_PIECE_TYPES;
use crate::games::blokus2027::piece::PieceType::PentoW;
use crate::games::blokus2027::rotation::Rotation::None;
use crate::games::blokus2027::{
    board::Board,
    color::Color,
    gamestate::GameState,
    piece::PieceType,
    r#move::Move,
    rotation::Rotation,
};

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

    let gamestate = Blokus2027::parse_memento(core::str::from_utf8(xml).unwrap()).unwrap();

    println!("{:?}", gamestate);

    let gamestate_target = MementoReturn::GameState(Box::new(GameState::new(PieceType::PentoW, true, Board::new(), 0, 1, Color::Blue, ALL_PIECE_TYPES.to_vec(), ALL_PIECE_TYPES.to_vec(), ALL_PIECE_TYPES.to_vec(), ALL_PIECE_TYPES.to_vec())));

    println!("{:?}", gamestate_target);

    assert!(gamestate == gamestate_target, "Gamestates are not the same")
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

    let last_move = Blokus2027::parse_memento(core::str::from_utf8(xml).unwrap()).unwrap();

    println!("{:?}", last_move);

    let last_movetarget = MementoReturn::LastMove(1, Move::Set(SetMove::new(Color::Blue, PentoW, 0, 16, true, None)));

    println!("{:?}", last_movetarget);

    assert!(last_move == last_movetarget, "Last moves are not the same")
}

#[test]
fn parse_memento_skip_move() {
    let xml = r#"
        <state turn="2">
            <lastMove class="sc.plugin2027.SkipMove">
            <color>GREEN</color>
            </lastMove>
        </state>"#;

    let message = Blokus2027::parse_memento(xml).expect("skip move should parse");
    let expected = MementoReturn::LastMove(
        2,
        Move::Skip(Color::Green,),
    );

    assert_eq!(message, expected);
}

#[test]
fn parse_memento_set_move_with_other_values() {
    let xml = r#"
        <state turn="3">
            <lastMove class="sc.plugin2027.SetMove">
            <piece color="YELLOW" kind="TRIO_L" rotation="RIGHT" isFlipped="false">
                <position x="3" y="5"/>
            </piece>
            </lastMove>
        </state>"#;

    let message = Blokus2027::parse_memento(xml).expect("set move should parse");
    let expected = MementoReturn::LastMove(
        3,
        Move::Set(SetMove::new(
            Color::Yellow,
            PieceType::TrioL,
            3,
            5,
            false,
            Rotation::Right,
        )),
    );

    assert_eq!(message, expected);
}

#[test]
fn parse_memento_rejects_xml_without_state() {
    let error = Blokus2027::parse_memento("<room/>")
        .expect_err("XML without a state should be rejected");

    assert!(error.to_string().contains("No valid memento"));
}

#[test]
fn parse_memento_rejects_initial_state_without_starting_piece() {
    let xml = r#"<state startTeam="ONE" turn="0" round="1"></state>"#;

    let error = Blokus2027::parse_memento(xml)
        .expect_err("initial state without a starting piece should be rejected");

    assert!(error.to_string().contains("No starting piece"));
}

#[test]
fn parse_memento_rejects_last_move_without_move_data() {
    let xml = r#"<state turn="1"></state>"#;

    let error = Blokus2027::parse_memento(xml)
        .expect_err("last move without move data should be rejected");

    assert!(error.to_string().contains("No lastMove received"));
}

#[test]
fn parse_memento_rejects_set_move_with_invalid_rotation() {
    let xml = r#"
        <state turn="1">
            <lastMove class="sc.plugin2027.SetMove">
            <piece color="BLUE" kind="PENTO_W" rotation="INVALID" isFlipped="true">
                <position x="0" y="16"/>
            </piece>
            </lastMove>
        </state>"#;

    assert!(Blokus2027::parse_memento(xml).is_err());
}
