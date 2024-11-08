#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BoardPiece {
    None,
    Red,
    Black,
}

pub fn make_blank_board() -> [[BoardPiece; 5]; 5] {
    [[BoardPiece::None; 5]; 5]
}

pub struct GameState {
    pub board: [[BoardPiece; 5]; 5],
    pub current_player: BoardPiece,
    pub pieces_droped: [i32; 2],
    history: Vec<PieceDropCommand>,
    history_pos: usize,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            board: make_blank_board(),
            current_player: BoardPiece::Red,
            pieces_droped: [0, 0],
            history: Vec::new(),
            history_pos: 0,
        }
    }

    pub fn jumble_board(&mut self) {
        self.board[1][0] = BoardPiece::Red;
        self.board[2][0] = BoardPiece::Black;
    }

    pub fn print_board(&self) {
        let mut label: String;

        for row in 0..5 {
            for col in 0..5 {
                if self.board[row][col] == BoardPiece::None {
                    label = "-".to_string();
                } else if self.board[row][col] == BoardPiece::Red {
                    label = "R".to_string();
                } else {
                    label = "B".to_string();
                }

                print!("{}", label)
            }

            println!()
        }

        println!()
    }

    pub fn handle_click(&mut self, row: usize, col: usize) {
        let command = PieceDropCommand {
            row: row,
            col: col,
            player: self.current_player,
        };

        if !command.is_valid(self) {
            return;
        }

        if self.history.len() > 0 {
            let element_to_clear = self.history.len() - (self.history_pos + 1);

            for _ in 0..element_to_clear {
                self.history.pop();
            }
        }

        command.perform(self);
        self.history.push(command);
        self.history_pos = self.history.len() - 1;

        // if row > 4 || col > 4 {
        //     return;
        // }

        // if self.pieces_droped[self.index_of_pieces(self.current_player)] >= 4 {
        //     return;
        // }

        // if self.board[row][col] != BoardPiece::None {
        //     return;
        // }

        // self.board[row][col] = self.current_player;

        // self.next_turn();
    }

    pub fn redo_action(&mut self) {
        if (self.history_pos + 1) >= self.history.len() {
            return;
        }

        self.history_pos += 1;

        let command: PieceDropCommand = self.history[self.history_pos].copy();

        command.perform(self);
    }

    pub fn undo_action(&mut self) {
        if self.history.len() == 0 {
            return;
        }

        let command: PieceDropCommand = self.history[self.history_pos].copy();

        command.undo(self);

        if self.history_pos == 0 {
            return;
        }

        self.history_pos -= 1;
    }

    // fn next_turn(&mut self) {
    //     self.pieces_droped[self.index_of_pieces(self.current_player)] += 1;

    //     if self.current_player == BoardPiece::Red {
    //         self.current_player = BoardPiece::Black
    //     } else {
    //         self.current_player = BoardPiece::Red;
    //     }
    // }

    fn index_of_pieces(&mut self, piece: BoardPiece) -> usize {
        if piece == BoardPiece::Red {
            return 0;
        }
        return 1;
    }
}

pub struct PieceDropCommand {
    pub row: usize,
    pub col: usize,
    pub player: BoardPiece,
}

impl PieceDropCommand {
    pub fn perform(&self, game: &mut GameState) {
        game.pieces_droped[game.index_of_pieces(self.player)] += 1;
        game.board[self.row][self.col] = self.player;
        if self.player == BoardPiece::Red {
            game.current_player = BoardPiece::Black
        } else {
            game.current_player = BoardPiece::Red
        }
    }

    pub fn undo(&self, game: &mut GameState) {
        if game.pieces_droped[game.index_of_pieces(self.player)] == 0 {
            return;
        }

        game.pieces_droped[game.index_of_pieces(self.player)] -= 1;
        game.board[self.row][self.col] = BoardPiece::None;
        game.current_player = self.player;
    }

    pub fn is_valid(&self, game: &mut GameState) -> bool {
        if self.row > 4 || self.col > 4 {
            return false;
        };

        if game.pieces_droped[game.index_of_pieces(self.player)] >= 4 {
            return false;
        }

        if game.board[self.row][self.col] != BoardPiece::None {
            return false;
        }

        return true;
    }

    pub fn copy(&self) -> Self {
        Self {
            row: self.row,
            col: self.col,
            player: self.player,
        }
    }
}
