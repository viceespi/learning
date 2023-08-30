fn main() {
    let empty_line: Vec<String> = vec![
        String::from(" 0 "),
        String::from(" 0 "),
        String::from(" 0 "),
        String::from(" 0 "),
        String::from(" 0 "),
        String::from(" 0 "),
        String::from(" 0 "),
        String::from(" 0 "),
    ];
    let white_line_1: Vec<String> = vec![
        String::from("wT1"),
        String::from("wH1"),
        String::from("wB1"),
        String::from("wQ "),
        String::from("wK "),
        String::from("wB2"),
        String::from("wH2"),
        String::from("wT2"),
    ];
    let white_line_2: Vec<String> = vec![
        String::from("wP1"),
        String::from("wP2"),
        String::from("wP3"),
        String::from("wP4"),
        String::from("wP5"),
        String::from("wP6"),
        String::from("wP7"),
        String::from("wP8"),
    ];
    let black_line_1: Vec<String> = vec![
        String::from("bT1"),
        String::from("bH1"),
        String::from("bB1"),
        String::from("bQ "),
        String::from("bK "),
        String::from("bB2"),
        String::from("bH2"),
        String::from("bT2"),
    ];
    let black_line_2: Vec<String> = vec![
        String::from("bP1"),
        String::from("bP2"),
        String::from("bP3"),
        String::from("bP4"),
        String::from("bP5"),
        String::from("bP6"),
        String::from("bP7"),
        String::from("bP8"),
    ];
    let white_team = teams(&white_line_1, &white_line_2);
    let black_team = teams(&black_line_1, &black_line_2);
    let player: bool = player_order();
    let mut board: Vec<Vec<String>> = set_initial_board(
        &player,
        &white_line_1,
        &white_line_2,
        &black_line_1,
        &black_line_2,
        &empty_line,
    );
    let mut current_player_color: String = associate_color_to_player(&player);
    let mut current_player_team =
        associate_team_to_player(&current_player_color, &black_team, &white_team);
    let mut has_king: bool = true;

    while has_king {
        println!("\nCurrent player is {}", current_player_color);
        organize_board(&board);
        println!("Choose a piece from your team\n");
        organize_team_pieces(&current_player_team);
        let chosen_piece: String = piece_chooser();
        println!("\nChoose the board tile to move (Line then Column)");
        let (mut new_line, mut new_column): (usize, usize) = new_position();
        let mut is_valid: bool = false;
        while !is_valid {
            is_valid = validate_movement(&current_player_color, &new_line, &new_column, &board);
            if !is_valid {
                println!("\nThe move is invalid");
                println!("\nChoose the board tile to move (Line then Column)");
                (new_line, new_column) = new_position();
                is_valid = validate_movement(&current_player_color, &new_line, &new_column, &board);
            }
        }
        let next_player_team: Vec<String> = edit_enemy_team(
            &current_player_color,
            &white_team,
            &black_team,
            &board,
            &new_line,
            &new_column,
        );
        current_player_team = next_player_team;
        swap_places(&mut board, &new_line, &new_column, &chosen_piece);
        organize_board(&board);
        current_player_color = change_player(current_player_color);
        has_king = check_for_king(&current_player_team);
        if !has_king {
            println!("Check mate");
        }
    }
    println!("{} is the loser..", current_player_color);
    current_player_color = change_player(current_player_color);
    println!("{} is the winner!!", current_player_color);
}

fn teams(line_1: &Vec<String>, line_2: &Vec<String>) -> Vec<String> {
    let mut team: Vec<String> = Vec::new();
    for piece in line_1 {
        team.push(piece.to_string());
    }
    for piece in line_2 {
        team.push(piece.to_string());
    }
    team
}

fn player_order() -> bool {
    let mut user_input: String = String::new();
    println!("Who will be the player 1? Black or White?\n");
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Invalid Input");
    let player_1: &str = user_input.trim();
    let mut player_1_white: bool = false;
    if player_1 == "White" {
        player_1_white = true;
    }
    player_1_white
}

fn associate_color_to_player(player: &bool) -> String {
    if player == &true {
        ("White").to_string()
    } else {
        ("Black").to_string()
    }
}

fn set_initial_board(
    player: &bool,
    white_line_1: &Vec<String>,
    white_line_2: &Vec<String>,
    black_line_1: &Vec<String>,
    black_line_2: &Vec<String>,
    empty_line: &Vec<String>,
) -> Vec<Vec<String>> {
    if player == &true {
        let starter_white_board: Vec<Vec<String>> = vec![
            white_line_1.clone(),
            white_line_2.clone(),
            empty_line.clone(),
            empty_line.clone(),
            empty_line.clone(),
            empty_line.clone(),
            black_line_2.clone(),
            black_line_1.clone(),
        ];
        starter_white_board
    } else {
        let starter_black_board: Vec<Vec<String>> = vec![
            black_line_1.clone(),
            black_line_2.clone(),
            empty_line.clone(),
            empty_line.clone(),
            empty_line.clone(),
            empty_line.clone(),
            white_line_2.clone(),
            white_line_1.clone(),
        ];
        starter_black_board
    }
}

fn organize_board(board: &Vec<Vec<String>>) {
    println!();
    for intern_vector in board {
        for element in intern_vector {
            print!("{}\t", element);
        }
        println!("\n\n");
    }
}

fn change_player(mut player_color: String) -> String {
    if player_color == "White" {
        player_color = "Black".to_string();
        player_color
    } else {
        player_color = "White".to_string();
        player_color
    }
}

fn associate_team_to_player(
    player_color: &String,
    black_team: &Vec<String>,
    white_team: &Vec<String>,
) -> Vec<String> {
    if player_color == "White" {
        (white_team).to_vec()
    } else {
        (black_team).to_vec()
    }
}

fn organize_team_pieces(team: &Vec<String>) {
    let mut i = 0;
    while i < team.len() {
        let piece: &String = &team[i];
        if (i + 1) == (team.len() / 2) || i == (team.len() - 1) {
            println!("{}", piece);
            i += 1;
        } else {
            print!("{}\t", piece);
            i += 1;
        }
    }
}

fn piece_chooser() -> String {
    let mut input: String = String::new();
    println!("\nType the piece that you'll move:");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Invalid input");
    let chosen_piece: String = (input.trim()).to_string();
    chosen_piece
}

fn current_piece_location(chosen_piece: &String, board: &Vec<Vec<String>>) -> (usize, usize) {
    let mut line: usize = 0;
    let mut column: usize = 0;
    while line < 8 {
        while column < 8 {
            let piece: &String = &board[line][column];
            if piece == chosen_piece {
                let current_line: usize = line;
                let current_column: usize = column;
                return (current_line, current_column);
            } else {
                column += 1;
            }
        }
        column = 0;
        line += 1;
    }
    panic!("Piece not found in the Board!");
}

fn new_position() -> (usize, usize) {
    let mut line_input: String = String::new();
    println!("\nChose the Line");
    std::io::stdin()
        .read_line(&mut line_input)
        .expect("\nInvalid Input");
    let new_line_string: String = (line_input.trim()).to_string();
    let new_line_index: usize = new_line_string.parse().expect("Invalid Parse");
    let mut column_input: String = String::new();
    println!("\nChose the Column");
    std::io::stdin()
        .read_line(&mut column_input)
        .expect("\nInvalid Input");
    let new_column_string: String = (column_input.trim()).to_string();
    let new_column_index: usize = new_column_string.parse().expect("Invalid Parse");
    (new_line_index, new_column_index)
}

fn validate_movement(
    player_color: &String,
    new_line: &usize,
    new_column: &usize,
    board: &Vec<Vec<String>>,
) -> bool {
    let mut valid_movement: bool = true;
    if player_color == "White" && board[*new_line][*new_column].starts_with('w')
        || new_line > &7
        || new_column > &7
    {
        valid_movement = false;
    }
    if player_color == "Black" && board[*new_line][*new_column].starts_with('b')
        || new_line > &7
        || new_column > &7
    {
        valid_movement = false;
    }
    valid_movement
}

fn swap_places(
    board: &mut Vec<Vec<String>>,
    new_line: &usize,
    new_column: &usize,
    chosen_piece: &String,
) {
    let (current_line, current_column) = current_piece_location(chosen_piece, board);
    board[*new_line][*new_column] = chosen_piece.to_string();
    board[current_line][current_column] = " 0 ".to_string();
}

fn edit_enemy_team(
    player_color: &String,
    white_team: &Vec<String>,
    black_team: &Vec<String>,
    board: &Vec<Vec<String>>,
    new_line: &usize,
    new_column: &usize,
) -> Vec<String> {
    let eliminated_piece: String = (board[*new_line][*new_column]).to_string();
    if player_color == "White" {
        let mut new_black_team: Vec<String> = vec![];
        for piece in black_team {
            if piece == &eliminated_piece {
                new_black_team.push("X".to_string());
            } else {
                new_black_team.push(piece.to_string());
            }
        }
        new_black_team
    } else {
        let mut new_white_team: Vec<String> = vec![];
        for piece in white_team {
            if piece == &eliminated_piece {
                new_white_team.push(" X ".to_string());
            } else {
                new_white_team.push(piece.to_string());
            }
        }
        new_white_team
    }
}

fn check_for_king(current_player_team: &Vec<String>) -> bool {
    let mut has_king: bool = false;
    let mut index: usize = 0;
    while index < current_player_team.len() {
        let piece: &str = &current_player_team[index];
        if let Some('K') = piece.chars().nth(1) {
            has_king = true;
        } else {
            index += 1;
        }
    }
    has_king
}
