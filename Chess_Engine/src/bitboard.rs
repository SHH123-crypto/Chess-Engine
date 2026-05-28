use std::mem;
use std::io;
use std::f64::consts;
/*
struct CBoard { 

    let W_ROOK: u64;               
    let W_KNIGHT: u64;
    let W_BISHOP: u64;
    let W_KING: u64;
    let W_QUEEN: u64;
    let W_PAWN: u64;

    let B_ROOK: u64;
    let B_KNIGHT: u64;
    let B_BISHOP: u64;
    let B_KING: u64;
    let B_QUEEN: u64;
    let B_PAWN: u64;
}
*/
let INF = f64::INFINITY;
let turn: bool = 1;

mod BITBOARD {

    struct W_ROOK {
        VAL: 5,
        POSITIONS: u64,
        POS_MOVES: [u64, 14] 
        //more later
    }

    struct W_BISHOP {
        VAL: 3,
        POSITIONS: u64,
        POS_MOVES: [u64, 13] 
        //more later
    }

    struct W_KNIGHT {
        VAL: 3,
        POSITIONS: u64,
        POS_MOVES: [u64, 8]
        //more later
    }


    struct W_KING {
        VAL: INF,
        POSITIONS: u64,
        POS_MOVES: [u64, 8] 
        //more later
    }


    struct W_QUEEN {
        VAL: 9,
        POS_MOVES: [u64, 27],
        POSITIONS: u64
        //more later
    }


    struct W_PAWN {
        VAL: 1,
        POSITIONS: u64,
        POS_MOVES: [u64, 4]
        //more later
    }
	
	struct B_ROOK {
        VAL: 5,
        POSITIONS: u64,
        POS_MOVES: [u64, 14] 
        //more later
    }

    struct B_BISHOP {
        VAL: 3,
        POSITIONS: u64,
        POS_MOVES: [u64, 13] 
        //more later
    }

    struct B_KNIGHT {
        VAL: 3,
        POSITIONS: u64,
        POS_MOVES: [u64, 8]
        //more later
    }


    struct B_KING {
        VAL: INF,
        POSITIONS: u64,
        POS_MOVES: [u64, 8] 
        //more later
    }


    struct B_QUEEN {
        VAL: 9,
        POS_MOVES: [u64, 27],
        POSITIONS: u64
        //more later
    }


    struct B_PAWN {
        VAL: 1,
        POSITIONS: u64,
        POS_MOVES: [u64, 4]
        //more later
    }
	
    fn rook_vision(PIECES: [u64, 11], b_rook: B_ROOK, b_knight: B_KNIGHT, b_bishop: B_BISHOP, b_king: B_KING, b_queen: B_QUEEN, b_pawn: B_PAWN, w_rook: W_ROOK, w_knight: W_KNIGHT, w_bishop: W_BISHOP, w_king: W_KING, w_queen: W_QUEEN, w_pawn: W_PAWN) {
		
		//algo that checks each piece's location and whether it coincides with the rook
		return //Piece name and ?location?
	}
	
	fn knight_vision(PIECES: [u64, 11]) {
		//algo that checks each piece's location and whether it coincides with the rook
		return //Piece name and ?location?
	}
	
	fn bishop_vision(PIECES: [u64, 11]) {
		//algo that checks each piece's location and whether it coincides with the rook
		return //Piece name and ?location?
	}
	
	fn king_vision(PIECES: [u64, 11]) {
		//algo that checks each piece's location and whether it coincides with the rook
		return //Piece name and ?location?
	}
	 
	fn queen_vision(PIECES: [u64, 11]) {
		//algo that checks each piece's location and whether it coincides with the rook
		return //Piece name and ?location?
	}
	
	fn pawn_vision(PIECES: [u64, 11]) {
		//algo that checks each piece's location and whether it coincides with the rook
		return //Piece name and ?location?
	}
	



	
}


 
fn main() {
    println!("Hello World");
}
