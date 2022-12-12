use std::io::{BufReader, BufRead};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// L'ordinateur commence
    #[arg(short, long, default_value_t = false)]
    ordinateur_en_premier: bool,

    /// Nombre de piles
    #[arg(short, long, default_value_t = 4)]
    piles: usize,
}

fn main() {
    let args: Args = Args::parse();
    let mut cols: Vec<u128> = vec![];
    for i in 0..args.piles {
        cols.push(i as u128 * 2 + 1);
    }
    let mut player = !args.ordinateur_en_premier;
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin);

    while (&cols).iter().sum::<u128>() != 0 {
        // Print game state
        println!("Les tas :");
        cols.iter().enumerate().for_each(|e| {
            println!("{}: {}", e.0 + 1, "I".repeat(*e.1 as usize));
        });

        if player {
            let col: usize;
            let amount: u128;
            loop {
                let mut input = String::new();
                println!("Quel tas ?");
                reader.read_line(&mut input).unwrap();
                if let Ok(i) = input.trim().parse::<usize>() {
                    if i > cols.len() {
                        println!("Tas inexistant, veuillez reesayer");
                    } else if cols[i - 1] == 0 {
                        println!("Tas vide, veuillez reesayer");
                    } else {
                        col = i - 1;
                        break;
                    }
                } else {
                    println!("'{}' n'est pas un nombre", input.trim());
                }
            }
            loop {
                let mut input = String::new();
                println!("Combien d'allumettes retirer ?");
                reader.read_line(&mut input).unwrap();
                if let Ok(i) = input.trim().parse::<u128>() {
                    if i > cols[col] {
                        println!("Nombre d'allumettes trop élevé, veuillez reesayer");
                    } else if i == 0 {
                        println!("Il faut au moins enlever une allumette, veuillez reesayer");
                    } else {
                        amount = i;
                        break;
                    }
                } else {
                    println!("'{}' n'est pas un nombre", input.trim());
                }
            }
            cols[col] -= amount;
        } else {
            let (index, amount) = get_best_move(&cols);
            cols[index] -= amount;
            println!("Computer removed {amount} from {}", index + 1);
        }
        if cols.iter().sum::<u128>() == 0 { println!("Vous avez gagné : {}", if player { "vrai" } else { "faux" } ); }
        player = !player;
    }
}

fn get_best_move(cols: &[u128]) -> (usize, u128) {
    let cp = cols.to_vec();

    for i in cols.iter().enumerate() {
        for j in 1..(*i.1 + 1) {
            let mut cp_local: Vec<u128> = cp.to_vec();
            cp_local[i.0] -= j;
            if calculate_cols(&cp_local).iter().all(|e| e % 2 == 0) {
                return (i.0, j)
            }
        }
    }

    let max = cols.iter().enumerate().reduce(|e, n| if e.1 > n.1 { e } else { n }).unwrap().0;
    (max, 1)
}

fn calculate_cols(cols: &[u128]) -> Vec<u128> {
    let mut bits = vec![];
    cols.iter().for_each(|next| {
        let mut bit = 0_u32;
        loop {
            if 2_u128.pow(bit) > *next {
                break;
            }
            if next & 2_u128.pow(bit) == 2_u128.pow(bit) {
                while bits.len() <= bit as usize {
                    bits.push(0);
                }
                bits[bit as usize] += 1;
            }
            bit += 1;
        }
    });

    bits
}
