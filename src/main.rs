fn main() {
    let mut table = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut player = 0;  
    let players = ["X", "O"];

    loop {
        println!("Player {}'s turn", players[player]);
        println!("Enter a number from 1 to 9 to place your mark");

        print_table(&table);

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input: usize = match input.trim().parse() {
            Ok(num) if num >= 1 && num <= 9 => num,
            _ => {
                println!("Invalid input. Please enter a number from 1 to 9.");
                continue;
            }
        };

        if table[input - 1] != "X" && table[input - 1] != "O" {
            table[input - 1] = players[player];
        } else {
            println!("This position is already taken");
            continue;
        }

        if check_winner(&table, players[player]) {
            print_table(&table);
            println!("Player {} wins!", players[player]);
            break;
        }

        if table.iter().all(|&x| x == "X" || x == "O") {
            print_table(&table);
            println!("It's a draw!");
            break;
        }

        player = 1 - player;
    }
}

fn print_table(table: &Vec<&str>) {
    println!(" {} | {} | {}", table[0], table[1], table[2]);
    println!("---|---|---");
    println!(" {} | {} | {}", table[3], table[4], table[5]);
    println!("---|---|---");
    println!(" {} | {} | {}", table[6], table[7], table[8]);
}

fn check_winner(table: &Vec<&str>, player: &str) -> bool {
    let winning_combinations = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];

    for combination in winning_combinations.iter() {
        if combination.iter().all(|&x| table[x] == player) {
            return true;
        }
    }

    false
}
