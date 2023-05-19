mod show_table;
use clap::Parser;
use rand::{thread_rng, Rng};
use regex::Regex;
use show_table::{
    print_array_h, print_array_h_no_corners, printable_array_v, printable_array_v_no_corners,
    printable_matrix,
};
use std::io;
use std::io::Write;

// use std::char;

fn fill_m(mut mat: Vec<Vec<String>>, x: usize, y: usize) -> Vec<Vec<String>> {
    let mut rand = thread_rng();
    for i in 0..x {
        for j in 0..y {
            mat[i][j] = rand.gen_range(0..2).to_string();
        }
    }
    mat
}

fn fill_line_sum(
    mut line_sum: Vec<String>,
    mat: &[Vec<String>],
    x: usize,
    y: usize,
) -> Vec<String> {
    let mut ones: usize;
    for i in 0..x {
        ones = 0;
        for j in 0..y {
            if mat[i][j] == *"1" {
                ones += 1
            }
        }
        line_sum[i] = " ".to_string() + &ones.to_string()
    }
    line_sum
}

fn fill_col_sum(mut col_sum: Vec<String>, mat: &[Vec<String>], x: usize, y: usize) -> Vec<String> {
    let mut ones: usize;
    for i in 0..x {
        ones = 0;
        for j in 0..y {
            if mat[j][i] == *"1" {
                ones += 1
            }
        }
        col_sum[i] = " ".to_string() + &ones.to_string()
    }
    col_sum
}

fn get_coordinates(numbers: usize, letters: usize) -> [Vec<String>; 2] {
    let mut t_numbers = vec!["".to_string(); numbers];
    let mut t_letters = vec!["".to_string(); letters];
    for i in 0..numbers {
        t_numbers[i] = " ".to_string() + &(i + 1).to_string()
    }
    for j in 0..letters {
        let ch = (((j + 65) as u8) as char).to_string();
        t_letters[j] = " ".to_string() + &ch;
    }
    [t_numbers, t_letters]
}

fn count_str(m: Vec<Vec<String>>, ch: String) -> usize {
    let mut counter: usize = 0;
    for i in m {
        for j in i {
            if j == ch {
                counter += 1
            }
        }
    }
    counter
}

fn hidden_ones(
    hidden_m: &[Vec<String>],
    score_m: &mut [Vec<String>],
    line_sum: &[String],
    col_sum: &[String],
    x: usize,
    y: usize,
    tries: usize,
) {
    println!("> Devinez la position des chiffres 1: ");
    println!("> Les chiffres dans les deux tableaux en Haut et a Gauche indiques combien de chiffres 1 existent dans la ligne/colonne correspondente.");
    println!("> Il existe {tries} uns dans ce tableau,  par conséquant vous avez le meme nombre de tours.");
    println!("> Pour choisir une case donner les coordonnées comme dans cet exemple: A,5.");
    println!("> Les coordonées possibles sont indiquées dans les tableaux a Droite et en bas.");
    for i in 0..tries {
        let coordinates = get_coordinates(y, x);
        let numbers = &coordinates[0];
        let letters = &coordinates[1];
        let max_coor_x = &letters[x - 1].trim();
        let max_coor_y = &numbers[y - 1].trim();
        let p_line_sum = printable_array_v(line_sum.to_vec(), 0);
        let p_score_m = printable_matrix(score_m.to_vec(), 1);
        let p_letters = printable_array_v_no_corners(letters.to_vec(), 0);
        print_array_h(col_sum.to_vec(), 5, 0);
        println!();
        for j in 0..p_score_m.len() {
            println!("{}{}{}", p_line_sum[j], p_score_m[j], p_letters[j]);
        }
        print_array_h_no_corners(numbers.to_vec(), 4);
        println!("{} essais restants.", tries - i);

        // TAKE INPUT //
        print!("Case: ");
        io::stdout().flush().unwrap();
        let mut coor = String::new();
        io::stdin()
            .read_line(&mut coor)
            .expect("Failed to read line");
        coor = coor.trim().to_string();
        // --------- //

        while !verify(&coor, max_coor_x.to_string(), max_coor_y.to_string()) {
            let mut help = "Format Invalide! Ecrire '?' pour l'aide\n";
            if coor == "?" {
                help = "Exemples de Formats Valides: A,1 - B,4 - F,5"
            }
            // TAKE INPUT //
            print!("{help}\nCase: ");
            io::stdout().flush().unwrap();
            coor = String::new();
            io::stdin()
                .read_line(&mut coor)
                .expect("Failed to read line");
            coor = coor.trim().to_string();
            // --------- //
        }
        let coor_t: Vec<&str> = coor.split(',').collect();
        let coor_x = coor_t[0].chars().next().unwrap() as usize - 65;
        let coor_y = coor_t[1].parse::<usize>().unwrap() - 1;
        if hidden_m[coor_x][coor_y] == *"1" {
            score_m[coor_x][coor_y] = "1".to_string()
        } else {
            score_m[coor_x][coor_y] = "-1".to_string()
        }
    }
    let p_score_m = printable_matrix(score_m.to_vec(), 1);
    let p_hidden_m = printable_matrix(hidden_m.to_vec(), 1);
    for k in 0..p_score_m.len() {
        println!("{}   {}", p_score_m[k], p_hidden_m[k]);
    }
    println!(
        "Vous avez terminez avec {} faute(s).",
        count_str(score_m.to_vec(), "-1".to_string())
    );
}
fn verify(ch: &str, max_letter: String, max_number: String) -> bool {
    let re = Regex::new(r"^[A-Z][,][0-9]{1,2}$").unwrap();
    if !re.is_match(ch) {
        return false;
    } else {
        let coor: Vec<&str> = ch.split(',').collect();
        let max_letter = max_letter.chars().next().unwrap() as u8;
        let letter = coor[0].chars().next().unwrap() as u8;
        let max_number = max_number.parse::<i32>().unwrap();
        let number = coor[1].parse::<i32>().unwrap();
        if number > max_number || letter > max_letter {
            return false;
        }
    }
    true
}

fn run_game(lines: usize, columns: usize) {
    let mut hidden_m: Vec<Vec<String>> = vec![vec![String::new(); columns]; lines];
    let mut score_m: Vec<Vec<String>> = vec![vec!["..".to_string(); columns]; lines];
    let mut line_sum: Vec<String> = vec![String::new(); lines];
    let mut col_sum: Vec<String> = vec![String::new(); columns];
    hidden_m = fill_m(hidden_m, lines, columns);
    line_sum = fill_line_sum(line_sum, &hidden_m, lines, columns);
    col_sum = fill_col_sum(col_sum, &hidden_m, lines, columns);
    // let coords = get_coordinates(lines, columns);
    let tries = count_str(hidden_m.to_vec(), "1".to_string());
    hidden_ones(
        &hidden_m,
        &mut score_m,
        &line_sum,
        &col_sum,
        lines,
        columns,
        tries,
    )
}

#[derive(Parser)]
#[command(name = "Hidden Ones")]
#[command(version = "1.0")]
#[command(
    about = "A guessing game",
    long_about = "Try to find where the numbers '1' are hidden with he provided hints"
)]
struct Args {
    #[arg(
        long,
        short,
        value_name = "SIZE",
        help = "Number of rows and columns (Default: 4)"
    )]
    size: Option<usize>,
}
fn main() {
    let args = Args::parse();
    let lines: usize;
    let columns: usize;

    match args.size {
        Some(s) => lines = s,
        None => lines = 4,
    }

    // match args.columns {
    //     Some(c) => lines = c,
    //     None => columns = lines,
    // }

    columns = lines;
    run_game(lines, columns)
}
