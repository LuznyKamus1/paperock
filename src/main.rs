use casual;
use rand::prelude::*;

fn main() {
    let mut player: usize;
    let mut computer: usize;
    let array = ["papier", "kamien", "nozyce"];
    let mut wynik = 0;
    let mut input: String;
    loop{
       
        input = casual::prompt("wybierz: papier/kamien/nozyce: ").get();
        player = match input.as_str(){
            "papier"=>0,
            "kamien"=>1,
            "nozyce"=>2,
            _=>panic!("zla odpowiedz!")
        };
        println!("wybrales: {}", array[player]);

        computer = rand::thread_rng().gen_range(0..3);
        println!("komputer wybral: {}", array[computer]);

        let result = game(player,computer);
        match result{
            0=>(println!("wygrales"), wynik=wynik+1),
            1=>(println!("przegrales"), wynik=wynik-1),
            2=>(println!("remis"), ()),
            _=>panic!("zly wynik")
        };
        println!("twoj wynik to: {}", wynik);
        input = casual::prompt("czy chcesz zagrac ponownie - tak/nie :").get();
        match input.as_str(){
            "tak"=>println!("ok!"),
            _=>break,
        }
    }
    println!("nara")
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
