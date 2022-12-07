mod kaarten;

use crate::kaarten::kaarten::Kaart;
use kaarten::kaarten::StokKaarten;
use std::io;
use std::sync::{Arc, Mutex, MutexGuard};

fn main() {
    let mut index: u32 = 0;
    let mut speler_hand: Arc<Mutex<Vec<Kaart>>> = Arc::new(Mutex::new(Vec::new()));
    let mut dealer_hand: Arc<Mutex<Vec<Kaart>>> = Arc::new(Mutex::new(Vec::new()));
    let mut stok: Arc<Mutex<Vec<StokKaarten>>> = Arc::new(Mutex::new(Vec::new()));
    let mut _saldo = 1000;
    let mut einde = false;
    let mut waarde = 0;
    let mut dealer_waarde = 0;
    stok.lock().unwrap().push(StokKaarten::new());
    stok.lock().unwrap().push(StokKaarten::new());

    start(
        speler_hand.lock().unwrap(),
        dealer_hand.lock().unwrap(),
        &mut stok,
    );
    while !einde {
        waarde = 0;
        dealer_waarde = 0;
        println!("speler hand:\n");
        for kaart in speler_hand.lock().unwrap().iter() {
            println!("{}", kaart);
        }
        let kaart1 = Kaart::get_waarde(&speler_hand.lock().unwrap()[0]);
        let kaart2 = Kaart::get_waarde(&speler_hand.lock().unwrap()[1]);

        for kaart in speler_hand.lock().unwrap().iter() {
            waarde += Kaart::get_waarde(kaart);
        }
        println!("kaart waarde: {}\n", &waarde);
        if waarde > 21 {
            einde = true;
            break;
        }

        println!("dealers hand:\n");
        println!("{}", dealer_hand.lock().unwrap()[0]);

        einde = choose(
            kaart1,
            kaart2,
            stok.lock().unwrap(),
            index,
            speler_hand.lock().unwrap(),
            dealer_hand.lock().unwrap(),
            dealer_waarde,
        );
    }
    for kaart in dealer_hand.lock().unwrap().iter() {
        dealer_waarde += Kaart::get_waarde(kaart);
    }

    println!("{}  {}", &waarde, &dealer_waarde);

    if waarde > dealer_waarde && waarde < 22 {
        println!("proficiat u heeft gewonnen");
    } else if waarde.eq(&dealer_waarde) {
        println!("gelijk spel")
    } else if dealer_waarde > 21 {
        println!("proficiat u heeft gewonnen");
    } else {
        println!("jammer maar helaas, de dealer heeft gewonnen")
    }
}

fn start(
    mut speler: MutexGuard<Vec<Kaart>>,
    mut dealer: MutexGuard<Vec<Kaart>>,
    mut stok: &mut Arc<Mutex<Vec<StokKaarten>>>,
) {
    *speler = deel(stok.lock().unwrap());
    *dealer = deel(stok.lock().unwrap());
}

fn deel(mut stok: MutexGuard<Vec<StokKaarten>>) -> Vec<Kaart> {
    let mut kaarten: Vec<Kaart> = Vec::new();
    kaarten.push(stok[0].trek_kaart());
    kaarten.push(stok[1].trek_kaart());

    kaarten
}

fn choose(
    k1: u8,
    k2: u8,
    mut stok: MutexGuard<Vec<StokKaarten>>,
    mut index: u32,
    mut speler: MutexGuard<Vec<Kaart>>,
    mut dealer_hand: MutexGuard<Vec<Kaart>>,
    mut dealer_waarde: u8,
) -> bool {
    let split_check = k1 == k2;
    match split_check {
        true => {
            println!("stand, split or hit? <0.1.2>");
            let mut input = String::new();
            io::stdin().read_line(&mut input);
            match input.trim() {
                "0" => einde(dealer_hand, dealer_waarde, stok),
                "1" => {
                    split();
                    false
                }
                "2" => {
                    speler.push(hit(stok, index));
                    false
                }
                _ => einde(dealer_hand, dealer_waarde, stok),
            }
        }
        false => {
            println!("standor  hit? <0.1>");
            let mut input = String::new();
            io::stdin().read_line(&mut input);
            match input.trim() {
                "0" => einde(dealer_hand, dealer_waarde, stok),
                "1" => {
                    speler.push(hit(stok, index));
                    false
                }
                _ => einde(dealer_hand, dealer_waarde, stok),
            }
        }
    }
}

fn hit(mut stok: MutexGuard<Vec<StokKaarten>>, mut index: u32) -> Kaart {
    index = match index {
        0 => 1,
        _ => 0,
    };
    let kaart = stok[0].trek_kaart();
    println!("u heeft {} getrokken", kaart);
    kaart
}

fn einde(
    mut dealer_hand: MutexGuard<Vec<Kaart>>,
    mut dealer_waarde: u8,
    mut stok: MutexGuard<Vec<StokKaarten>>,
) -> bool {
    let mut waarde = 0;

    loop {
        waarde = 0;
        println!("dealer hand:\n");
        for kaart in dealer_hand.iter() {
            println!("{}", kaart);
            waarde += Kaart::get_waarde(kaart);
        }

        if waarde < 17 {
            dealer_hand.push(stok[0].trek_kaart());
        } else {
            break;
        }
    }
    dealer_waarde = waarde;

    true
}

fn split() {}
