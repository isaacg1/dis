use std::collections::{HashMap, HashSet};
type Trits = [u64; 10];

fn int_to_trits(mut n: u64) -> Trits {
    let mut trits = [0; 10];
    for i in 0..10 {
        trits[9 - i] = n % 3;
        n /= 3;
    }
    trits
}

fn roll_trit(trits: Trits) -> Trits {
    let mut new = [0; 10];
    new[0] = trits[9];
    for i in 0..9 {
        new[i + 1] = trits[i]
    }
    new
}

fn trits_to_int(trits: Trits) -> u64 {
    let mut out = 0;
    for t in trits {
        out *= 3;
        out += t;
    }
    out
}

fn tswb(t1: Trits, t2: Trits) -> Trits {
    let mut out = [0; 10];
    for i in 0..10 {
        out[i] = (3 + t1[i] - t2[i]) % 3;
    }
    out
}

fn generate(debug: bool) {
    if !debug {
        // Setup string
        print!("{}", "*^||||||||||||||||||||||||||||||||>||||{^||!");
    }
    let basic_chars = ['*', '^', '>', '|', '}', '{', '!', '_'];
    let mut map = HashMap::new();
    for c1 in basic_chars {
        for c2 in basic_chars {
            for c3 in basic_chars {
                for c4 in basic_chars {
                    for c5 in basic_chars {
                        let t1 = int_to_trits(c1 as u8 as u64);
                        let t2 = int_to_trits(c2 as u8 as u64);
                        let t3 = int_to_trits(c3 as u8 as u64);
                        let t4 = int_to_trits(c4 as u8 as u64);
                        let t5 = int_to_trits(c5 as u8 as u64);
                        let mut acc = roll_trit(t1);
                        acc = tswb(acc, t2);
                        acc = tswb(acc, t3);
                        acc = tswb(acc, t4);
                        acc = tswb(acc, t5);
                        let was_new = map.insert(
                            trits_to_int(acc) % 256,
                            format!("{}{}{}{}{}", c1, c2, c3, c4, c5),
                        );
                        if was_new.is_some() && debug {
                            println!(
                                "{}{}{}{}{}: {} {}",
                                c1,
                                c2,
                                c3,
                                c4,
                                c5,
                                trits_to_int(acc),
                                trits_to_int(acc) % 256
                            );
                        }
                    }
                }
            }
        }
    }
    if debug {
        println!("{}", map.len());
    } else {
        for i in 0..256 {
            if let Some(string) = map.get(&i) {
                print!("{}_!", string);
            }
        }
        println!();
    }
}

fn verify() {
    let output = "000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F202122232425262728292A2B2C2D2E2F303132333435363738393A3B3C3D3E3F404142434445464748494A4B4C4D4E4F505152535455565758595A5B5C5D5E5F606162636465666768696A6B6C6D6E6F707172737475767778797A7B7C7D7E7F808182838485868788898A8B8C8D8E8F909192939495969798999A9B9C9D9E9FA0A1A2A3A4A5A6A7A8A9AAABACADAEAFB0B1B2B3B4B5B6B7B8B9BABBBCBDBEBFC0C1C2C3C4C5C6C7C8C9CACBCCCDCECFD0D1D2D3D4D5D6D7D8D9DADBDCDDDEDFE0E1E2E3E4E5E6E7E8E9EAEBECEDEEEFF0F1F2F3F4F5F6F7F8F9FAFBFCFDFEFF00";
    let unique: HashSet<_> = output.as_bytes().chunks(2).collect();
    println!("{}", unique.len());
}

fn main() {
    let kind = 1;
    match kind {
        0 => generate(true),
        1 => generate(false),
        2 => verify(),
        _ => unimplemented!(),
    }
}
