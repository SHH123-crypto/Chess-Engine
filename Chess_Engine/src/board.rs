use std::mem;
use std::io;
use std::f64::consts;
use bitvec::prelude::*;
use std::thread;
use std::math;

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
let turn: bool = 1;  //1 == white, 0 == black

mod PIECES {

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
	
	fn index_converter(index: i16) -> (i16, i16) {
		col = (index) % 8;
		row = (index) / 8;
		return (row, col);
	
	fn piece_finder(BitVec<Lsb0, u64> piece) -> Vec<(i16, i16)> {
		let mut piece_list: Vec<(i16, i16)> = Vec::new();
		for(i, p) in piece.iter().enumerate() {
			if p == 1 {
				let conv_index = index_converter(i);
				piece_list.push(conv_index);
			}
		}
		return piece_list;
				
			
	
    let mut squares_attacked: Vec<(i16, i16)> = Vec::new();
	
	fn king_attack(b_king: B_KING, w_king: W_KING, b_rook: B_ROOK, w_rook: W_ROOK, b_knight: B_KNIGHT, w_knight: W_KNIGHT, w_bishop: W_BISHOP, b_bishop: B_BISHOP) {
		let b_k: BitVec<Lsb0, u64> = b_king.POSITIONS;
		
		let b_king_loc = piece_finder(b_k)[0];
		if b_king_loc.0 + 1 != 8 {
			squares_attacked.push(b_king_loc.0 + 1, b_king_loc.1);
		}
		if b_king_loc.0 + 1 != 8 && b_king_loc.1 + 1 != 8 {
			squares_attacked.push(b_king_loc.0 + 1, b_king_loc.1 + 1);
		}
		if b_king_loc.1 + 1 != 8 {
			squares_attacked.push(b_king_loc.0, b_king_loc.1 + 1);
		}
		if !b_king_loc.0 - 1 < 0 && b_king_loc.1 + 1 != 8 {
			squares_attacked.push(b_king_loc.0 - 1, b_king_loc.1 + 1);
		}
		if !b_king_loc.0 - 1 < 0 {
			squares_attacked.push(b_king_loc.0 - 1, b_king_loc.1);
		}
		if !b_king_loc.0 - 1 < 0 && !b_king_loc.1 - 1 < 0 {
			squares_attacked.push(b_king_loc.0 - 1, b_king_loc.1 - 1);
		}
		if !b_king_loc.1 - 1 < 0 {
			squares_attacked.push(b_king_loc.0, b_king_loc.1 - 1);
		}
		if !b_king_loc.0 + 1 != 8 && !b_king_loc.1 - 1 < 0 {
			squares_attacked.push(b_king_loc.0 + 1, b_king_loc.1 - 1);
		}
		
		//a thread for each piece that checks if any movement of the king isn't possible. 
		
		thread::spawn(|| {
			w_rook_l = piece_finder(w_rook.POSITIONS);
			for (i, k) in squares_attacked.iter().enumerate() {
				for r in w_rook_l {
					if r.0 == k.0 || r.1 == k.1 {
						squares_attacked.remove(i);
					}
				}
			}
		});
		
		thread::spawn(|| {
			w_knight_l = piece_finder(w_knight.POSITIONS);
			for (i, k) in squares_attacked.iter().enumerate() {
				for b in w_knight_l {
					if !b.0 - 2 < 0 && !b.1 - 1 < 0 {
						if (b.0 - 2, b.1 - 1) == k {
							squares_attacked.remove(i);
						}
					}
					if !b.0 - 2 < 0 && !b.1 + 1 > 7 {
						if (b.0 - 2, b.1 + 1) == k {
							squares_attacked.remove(i);
						}
					}
					
					if !b.1 + 2 > 7 && !b.0 - 1 < 0 {
						if (b.1 + 2, b.0 - 1) == k {
							squares_attacked.remove(i);
						}
					}
					if !b.1 + 2 > 7 && !b.0 + 1 > 7 {
						if (b.1 + 2, b.0 + 1) == k {
							squares_attacked.remove(i);
						}
					}
					
					if !b.0 + 2 > 7 && !b.1 - 1 < 0 {
						if (b.0 + 2, b.1 - 1) == k {
							squares_attacked.remove(i);
						}
					}
					if !b.0 + 2 > 7 && !b.1 + 1 > 7 {
						if (b.0 + 2, b.1 + 1) == k {
							squares_attacked.remove(i);
						}
					}
					
					if !b.1 - 2 < 0 && !b.0 - 1 < 0 {
						if (b.1 - 2, b.0 - 1) == k {
							squares_attacked.remove(i);
						}
					}
					if !b.1 - 2 < 0 && !b.0 + 1 > 7 {
						if (b.1 - 2, b.0 + 1) == k {
							squares_attacked.remove(i);
						}
					}	
				}
			}
		});
		//For diagonal pieces get the slope between the two points and check if it is a slope of 1
		//For pawns and the bitboards and check if the two diagonals in front of the king are 1s or not
		thread::spawn( || {
			w_bish_l = piece_finder(w_bishop.POSITIONS);
			for (i, k) in squares_attacked.iter().enumerate() {
				for b in w_bish_l {
					let slope = ((b.1 - squares_attacked[i].1) / (b.0 - squares_attacked[i].0)).abs();
					if slope == 1 {
						squares_attacked.remove(i);
					}
				}
			}
		});
		
		thread::spawn ( || {
			w_queen_l = piece_finder(w_queen.POSITIONS);
			for (i, k) in squares_attacked.iter().enumerate() {
				for q in w_queen_l {
					let slope = ((q.1 - squares_attacked[i].1) / (q.0 - squares_attacked[i].0)).abs();
					if slope == 1 || (q.0 == k.0 || q.1 == k.1) {
						w_queen_l.remove(i);
					}
				}
			}
		});
		
		thread::spawn( || {
			w_king_p = piece_finder(w_king.POSITIONS)[0];
			for (i, k) in squares_attacked.iter().enumerate() {
				let distance = (( (float) w_king_p.1 - (float) k.1).pow(2) + ( (float) w_king_p.1 - (float) k.1).pow(2)).sqrt();
				if distance == 1 || distance == 2.sqrt() {
					squares_attacked.remove(i);
				}
			}
		});
		
		
			
				
		
		return squares_attacked;
				
		

	
}


 
fn main() {
    println!("Hello World");
}
