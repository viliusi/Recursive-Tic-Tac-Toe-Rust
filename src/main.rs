use console::Term;

fn main() {
    // 0 = empty
    // 1 = X
    // 2 = O
    //
    // 0 1 2
    // 3 4 5
    // 6 7 8
    let mut board = [[0u8; 9]; 9];

    // Denotes which board is currently being played on
    // 0 means that it is up to your choice, either at the start of the game, or if the send to
    // board is done
    let mut focus = 0;

    // Refers to if the script is looking at a specific board, or all of them.
    // If false, then we are looking at all
    // If true, we are looking at a specific board
    let mut preview = false;

    // Stores the inputted value up until it is confirmed
    // 0 is unchosen
    let mut held_move = 0;

    // For when you need to move pieces, if you pick up a piece, but want to unpick it up, this
    // value will be used
    let mut return_pos = 0;

    // true = player 1
    // false = player 2
    let mut current_players_turn = true;

    // Start screen
    //
    // q = go back
    // nums to focus on board section
    // enter confirm
    //
    // If a specific part is not selected, press the corresponding number to select (Check for this
    // with a seperate variable)
    // When you're there, a selection will be marked by green to ask to confirm placement, every
    // time you press it will change which is green.
    // If you have 3 pieces on already, you will need to move them, by first pressing on it to mark
    // it red, then pressing an empty space, to mark it green. And then press enter to confirm your
    // move.
    
    print_gui(current_players_turn);

    print_full_board(&mut board, &mut preview);

    println!("Welcome, to Recursive Tic Tac Toe in Rust, by viliusi");

    let stdout = Term::buffered_stdout();

    'game_loop: loop {
        if let Ok(character) = stdout.read_char() {
            match character {
                'e' => {
                    print_gui(current_players_turn);
                    print_full_board(&mut board, &mut preview);
                }
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    match preview {
                        true => try_move(character, &mut board, &mut focus),
                        false => view_board(character, &mut board, &mut preview),
                    }
                }
                'q' => break 'game_loop,
                'h' => println!("E to view all boards, Q to quit game, 1-9 to interact, enter to confirm move."),
                _ => println!("Not a recognized input, press H for help."),
            }
        }
    }
    
}

// Preview a specific board
fn view_board(input: char, board: &mut [[u8; 9]; 9], preview: &mut bool) {
    clear_screen();

    let iterations = 3;

    // I don't quite understand this, but when I press 1, it gives 49 normally, 2 gives 50, so this
    // fix should work, hopefully.
    let index: usize = input as usize - 49;

    let mut i = 0;
    let mut i_target = 0;

    println!("+---+");
    for _num in 0..iterations {
        print!("|");

        for _num in 0..iterations {
            i = i_target;

            let piece = board[index][i];
                    
            match piece {
                1 => print!("x"),
                2 => print!("O"),
                _ => print!("/"),
            }

            i += 1;
        }

        i_target += 3;

        print!("|");
        println!();
    }
    println!("+---+");

    *preview = true;
}

// Check for if the chosen move is possible
fn try_move(input: char, board: &mut [[u8; 9]; 9], focus: &mut u8) {
    println!("Move tried");
}

// Confirm the placement, add to board, clear held_move and change the focus for next player
fn confirm_move(board: &mut [[u8;9]; 9], held_move: u8, focus: &mut u8) {
    
}

// Clears the console window
fn clear_screen() {
    print!("{esc}c", esc = 27 as char);
}

// Function to show which players turn it is and so on
fn print_gui(player_turn: bool) {
    clear_screen();
    
    match player_turn {
        true => println!("Player 1's turn."),
        false => println!("Player 2's turn."),
    }
}

    // Pass a reference to the board array so it can be printed
    // First three lines will have ⎸ to part the boards
    // Between 3rd and 4th line will be ---+---+---
    // And so on, should look something like this
    // ///I///I///
    // ///I///I///
    // ///I///I///
    // ---+---+---
    // ///I///I///
    // ///I///I///
    // ///I///I///
    // ---+---+---
    // ///I///I///
    // ///I///I///
    // ///I///I///
fn print_full_board(board: &mut [[u8; 9]; 9], preview: &mut bool) {
    let iterations = 3; 

    let mut i;
    let mut i_target = 0;
    let mut j = 0;

    println!("+---+---+---+");
    // This part makes sure all 3 of the fields get run
    for _num in 0..iterations {

        // This makes sure all the lines get printed
        for _num in 0..iterations {

            // Sets up a full line of the board
            for _num in 0..iterations {
                i = i_target;
                print!("|");

                // TODO: Color the text if it's focused
                // Sets up the 3 values in the line of one board
                for _num in 0..iterations {
                    let piece = board[i][j];
                    
                    match piece {
                        1 => print!("x"),
                        2 => print!("O"),
                        _ => print!("/"),
                    }

                    i += 1;
                }

            }

            // Finishes of a line
            print!("|");
            j += 1;
            i_target += 3;
            println!();
        }

        // For the next field board to be printed, it needs to reset the target and set the end
        // line
        i_target = 0;
        println!("+---+---+---+");
    }

    *preview = false;
}

