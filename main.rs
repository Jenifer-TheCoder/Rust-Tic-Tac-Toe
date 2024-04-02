use std::io;

// Define a structure to represent the Tic Tac Toe board
struct Board {
    cells: Vec<Option<char>>,
}

impl Board {
    // Constructor function to create a new empty board
    fn new() -> Self {
        Board {
            cells: vec![None; 9],
        }
    }

    // Function to display the current state of the board
    fn display(&self) {
        println!("-------------");
        for (i, cell) in self.cells.iter().enumerate() {
            if i % 3 == 0 {
                print!("| ");
            }
            match cell {
                Some(symbol) => print!("{} | ", symbol),
                None => print!("  | "),
            }
            if (i + 1) % 3 == 0 {
                println!("\n-------------");
            }
        }
    }

    // Function to check if the game has been won
    fn check_win(&self, symbol: char) -> bool {
        let win_patterns = vec![
            // Rows
            (0, 1, 2), (3, 4, 5), (6, 7, 8),
            // Columns
            (0, 3, 6), (1, 4, 7), (2, 5, 8),
            // Diagonals
            (0, 4, 8), (2, 4, 6),
        ];
        win_patterns.iter().any(|&(a, b, c)| {
            self.cells[a] == Some(symbol) && self.cells[b] == Some(symbol) && self.cells[c] == Some(symbol)
        })
    }

    // Function to check if the board is full
    fn is_full(&self) -> bool {
        self.cells.iter().all(|&cell| cell.is_some())
    }

    // Function to make a move on the board
    fn make_move(&mut self, position: usize, symbol: char) -> Result<(), &'static str> {
        if position >= self.cells.len() || self.cells[position].is_some() {
            return Err("Invalid move");
        }
        self.cells[position] = Some(symbol);
        Ok(())
    }
}

fn main() {
    println!("Welcome to Tic Tac Toe!");

    let mut board = Board::new();
    let mut current_player = 'X';

    loop {
        // Display the board
        board.display();

        // Prompt the current player for their move
        println!("Player {}, enter your move (1-9):", current_player);

        // Read player input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Parse input
        let position: usize = match input.trim().parse::<usize>() {
            Ok(num) => num - 1,
            Err(_) => {
                println!("Invalid input. Please enter a number between 1 and 9.");
                continue;
            }
        };

        // Make the move
        match board.make_move(position, current_player) {
            Ok(_) => {
                // Check for win
                if board.check_win(current_player) {
                    println!("Player {} wins!", current_player);
                    break;
                }
                // Check for draw
                if board.is_full() {
                    println!("It's a draw!");
                    break;
                }
                // Switch players
                current_player = if current_player == 'X' { 'O' } else { 'X' };
            }
            Err(err) => println!("{}", err),
        }
    }
}