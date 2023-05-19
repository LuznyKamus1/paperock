use casual;
use rand::prelude::*;

fn main() {
    let player: usize;
    let computer: usize;
    let array = ["papier", "kamien", "nozyce"];

    println!("papier - 0");
    println!("kamien - 1");
    println!("nozyce - 2");
    
    player = casual::prompt(": ").get();
    println!("wybrales: {}", array[player]);

    computer = rand::thread_rng().gen_range(0..3);
    println!("komputer wybral: {}", array[computer]);

    let result = game(player,computer);
    match result{
        0=>println!("you won!"),
        1=>println!("you lost!"),
        2=>println!("draw!"),
        _=>panic!("wrong resoult")
    }
}
fn game(p: usize, c: usize) -> usize {

    if p==c {return 2;}
    else if p==0 && c==1 {return 0;}
    else if p==0 && c==2 {return 1;}
    else if p==1 && c==0 {return 1;}
    else if p==1 && c==2 {return 0;}
    else if p==2 && c==0 {return 0;}
    else {return 1;}
}
