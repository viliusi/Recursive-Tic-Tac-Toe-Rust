fn main() {
    // 0 = empty
    // 1 = X
    // 2 = O
    //
    // 0 1 2
    // 3 4 5
    // 6 7 8
    let mut board = [[0u8; 9]; 9];

    println!("Welcome, to Recursive Tic Tac Toe in Rust, by viliusi");

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
    
    print_boards(&mut board);
    
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
fn print_boards(board: &mut [[u8; 9]; 9]) {
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
                print!("I");

                // TODO: Color the text if it's focused
                // Sets up the 3 values in the line of one board
                for _num in 0..iterations {
                    print!("{}", board[i][j]);
                    i += 1;
                }

            }

            // Finishes of a line
            print!("I");
            j += 1;
            i_target += 3;
            println!();
        }

        // For the next field board to be printed, it needs to reset the target and set the end
        // line
        i_target = 0;
        println!("+---+---+---+");
    }
}
