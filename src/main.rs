fn main() {
    // 0 = empty
    // 1 = X
    // 2 = O
    //
    // 1 2 3
    // 4 5 6
    // 7 8 9
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
    
    println!("{}", board[0][0]);


}
