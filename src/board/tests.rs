use super::*;

#[test]
fn add_tile() {
    let mut b = Board::new();

    for expected_count in (0..LENGTH+1).rev() {
        assert_eq!(b.empty_tiles().len(), expected_count);
        assert_eq!(b.score, 0);

        let result = b.add_tile();

        if expected_count > 0 {
            assert!(result.is_some());
            b = result.unwrap();
        } else {
            assert!(result.is_none());
        }
    }

}

macro_rules! tile {
    (o) => { Tile::Empty };
    (A) => { Tile::Occupied(1) };
    (B) => { Tile::Occupied(2) };
    (C) => { Tile::Occupied(3) };
    (D) => { Tile::Occupied(4) };
    (E) => { Tile::Occupied(5) };
    (F) => { Tile::Occupied(6) };
    (G) => { Tile::Occupied(7) };
    (H) => { Tile::Occupied(8) };
    (I) => { Tile::Occupied(9) };
    (J) => { Tile::Occupied(10) };
    (K) => { Tile::Occupied(11) };
}

macro_rules! board {
    ( $($t:ident), * ) => {
        board!( $($t), * | 0 )
    };
    ( $($t:ident), * | $s:expr ) => {
        Board {
            tiles: vec![ $(tile!($t),) * ],
            score: $s
        }
    };
}

macro_rules! assert_move {
    ( $d:path ; $($t:ident), * | $ts:expr => $($n:ident), * | $ns:expr ) => {
        assert_eq!(
            board!($($t), * | $ts).move_board($d),
            Some(board!($($n), * | $ns))
        );
    };
}

#[test]
fn create_new() {
    assert_eq!(Board::new().tiles.len(), LENGTH);
}

#[test]
fn board_macro() {
    assert_eq!(board!(
            A,B,C,D,
            E,F,G,H,
            I,J,K,o,
            o,o,o,o
    ).tiles[0], Tile::Occupied(1));
}

#[test]
fn tile_macro() {
    assert_eq!(tile!(A), Tile::Occupied(1));
    assert_eq!(tile!(o), Tile::Empty);
}

#[test]
fn simple_move() {
    assert_move!(Direction::Right ;
        o,o,o,o,
        A,B,C,o,
        C,A,o,B,
        A,o,o,o | 0
        =>
        o,o,o,o,
        o,A,B,C,
        o,C,A,B,
        o,o,o,A | 0
    );
}

#[test]
fn simple_merge() {
    assert_move!(Direction::Right ;
        o,o,o,o,
        A,B,C,C,
        C,A,A,B,
        A,o,o,A | 0
        =>
        o,o,o,o,
        o,A,B,D,
        o,C,B,B,
        o,o,o,B | ( 16 + 4 + 4 )
    );
}

#[test]
fn tricky_merge() {
    assert_move!(Direction::Right ;
        A,A,B,B,
        A,B,A,B,
        A,A,A,A,
        A,B,B,A | 0
        =>
        o,o,B,C,
        A,B,A,B,
        o,o,B,B,
        o,A,C,A | ( 4 + 8 + 4 + 4 + 8 )
    );
}

#[test]
fn other_direction() {
    assert_move!(Direction::Left ;
        A,A,B,B,
        A,B,A,B,
        A,A,A,A,
        A,B,B,A | 0
        =>
        B,C,o,o,
        A,B,A,B,
        B,B,o,o,
        A,C,A,o | ( 4 + 8 + 4 + 4 + 8 )
    );
}

#[test]
fn up_and_down() {
    assert_move!(Direction::Up ;
        A,A,B,B,
        A,B,A,B,
        A,A,A,A,
        A,B,B,A | 0
        =>
        B,A,B,C,
        B,B,B,B,
        o,A,B,o,
        o,B,o,o | ( 4 + 4 +  4 + 8 + 4)
    );

    assert_move!(Direction::Down ;
        A,A,B,B,
        A,B,A,B,
        A,A,A,A,
        A,B,B,A | 0
        =>
        o,A,o,o,
        o,B,B,o,
        B,A,B,C,
        B,B,B,B | ( 4 + 4 + 4 + 8 + 4)
    );
}

#[test]
fn can_move() {
    let b = board!(
        A,o,o,o,
        o,o,o,o,
        o,o,o,o,
        o,o,o,o);

    assert!(b.can_move(Direction::Down));
    assert!(b.can_move(Direction::Right));
    assert!(!b.can_move(Direction::Up));
    assert!(!b.can_move(Direction::Left));

    let b = board!(
        A,A,o,o,
        o,o,o,o,
        o,o,o,o,
        o,o,o,o);

    assert!(b.can_move(Direction::Down));
    assert!(b.can_move(Direction::Right));
    assert!(b.can_move(Direction::Left));
    assert!(!b.can_move(Direction::Up));

    let b = board!(
        A,A,B,C,
        D,E,F,G,
        H,I,J,K,
        J,K,I,H);

    assert!(b.can_move(Direction::Left));
    assert!(b.can_move(Direction::Right));
    assert!(!b.can_move(Direction::Down));
    assert!(!b.can_move(Direction::Up));
}

#[test]
fn has_possible_moves() {
    let b = board!(
        A,B,C,D,
        E,F,G,H,
        I,J,K,J,
        H,I,J,o);

    assert!(b.has_possible_moves());

    let b = board!(
        A,B,C,D,
        E,F,G,H,
        I,J,K,J,
        H,I,A,A);

    assert!(b.has_possible_moves());

    let b = board!(
        A,B,C,D,
        E,F,G,H,
        I,J,K,J,
        H,I,B,A);

    assert!(!b.has_possible_moves());
}
