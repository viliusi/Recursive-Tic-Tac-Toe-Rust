use console::Term;
use std::thread;
use std::time;

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
    // 9 means that it is up to your choice, either at the start of the game, or if the send to
    // board is done
    let mut focus = 9;

    // Shows which board we are previewing, 9 means we are looking at all the boards
    let mut preview = 9;

    // Stores the inputted value up until it is confirmed
    // 9 is unchosen
    let mut held_move = 9;

    // For when you need to move pieces, if you pick up a piece, but want to unpick it up, this
    // value will be used
    let mut return_pos = 0;

    // true = player 1 X
    // false = player 2 O
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
                    if (preview == 9 as u8) {
                        held_move = 9;
                        return_pos = 9;

                        view_board(character, &mut board, &mut preview, current_players_turn, &mut held_move);
                    }
                    else {
                        // TODO Figure out why i need to try twice, before it actually works, it's
                        // as if held_move lacks behind
                        
                        try_move(character, &mut board, &mut focus, &mut preview, &mut held_move, current_players_turn);

                        // DEBUG
                        let index = input_to_index(character);

                        println!("{}", index);
                        println!("{}", held_move);

                        if (held_move != index as u8) {
                            println!("not equal");
                        } 
                        
                        thread::sleep(time::Duration::from_millis(5000));
                        // \DEBUG

                        view_board(character, &mut board, &mut preview, current_players_turn, &mut held_move);
                    }
                }
                '\n' => confirm_move(&mut board, &mut held_move, &mut focus, &mut preview, &mut current_players_turn),
                'q' => break 'game_loop,
                'h' => println!("E to view all boards, Q to quit game, 1-9 to interact, enter to confirm move."),
                _ => println!("Not a recognized input, press H for help."),
            }
        }
    }
    
}

// Preview a specific board
fn view_board(input: char, board: &mut [[u8; 9]; 9], preview: &mut u8, current_turn: bool, held_move: &mut u8) {
    print_gui(current_turn);

    let iterations = 3;

    // I don't quite understand this, but when I press 1, it gives 49 normally, 2 gives 50, so this
    // fix should work, hopefully. (I think it's because of the keycodes)
    let index = input_to_index(input);

    let mut i : u8 = 0;
    let mut i_target = 0;

    println!("+---+");
    for _num in 0..iterations {
        print!("|");

        i = i_target;

        for _num in 0..iterations {
            let piece = board[index as usize][i as usize];

            if *held_move == i {
                 match piece {
                        0 => {
                            match current_turn {
                                true => print!("X"),
                                false => print!("O"),
                            }
                        }
                        _ => {
                            (match piece {
                                1 => print!("X"),
                                2 => print!("O"),
                                _ => print!("/"),
                            });

                            // TODO get this reference to work
                            *held_move = 9;
                        }
                    }
            }
            else {
                 match piece {
                        1 => print!("X"),
                        2 => print!("O"),
                        _ => print!("/"),
                    }
            }

            i += 1;
        }

        i_target += 3;

        print!("|");
        println!();
    }
    println!("+---+");

    *preview = index;
}

fn input_to_index(input: char) -> u8 {
    let index: u8 = input as u8 - 49;
    index
}

// Check for if the chosen move is possible
fn try_move(input: char, board: &mut [[u8; 9]; 9], focus: &mut u8, preview: &mut u8, held_move: &mut u8, current_turn: bool) {
    let index = input_to_index(input);

    // switch to if
    match preview {
        index => {
            *held_move = *index as u8;
        }
        _ => {
            println!("Can't make a move on this board.");
            *held_move = 9;
        }
    }
}

// Confirm the placement, add to board, clear held_move and change the focus for next player
fn confirm_move(board: &mut [[u8;9]; 9], held_move: &mut u8, focus: &mut u8, preview: &mut u8, player_turn: &mut bool) {
    // Check the placement
    if *focus == 9 {
        // Move to any board
        println!("possible move");

        // Check if there is nothing there
    }
    else if *focus == *preview {
        // If trying to move on a possible board

    }
    else {
        println!("Not possible");
    }

    // Add to board array
    

    // Set held_move to 9
    

    // Change player turn bool


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
fn print_full_board(board: &mut [[u8; 9]; 9], preview: &mut u8) {
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
                        1 => print!("X"),
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

    *preview = 9;
}

