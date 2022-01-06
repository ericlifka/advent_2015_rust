use crypto::{md5::Md5,digest::Digest};

pub fn run() {
    let mut hasher = Md5::new();
    let input = b"ckczppom";
    let mut i: u64 = 0;
    let mut hash = [0; 16];
    let mut part1 = 0;

    loop {
        i += 1;
        let seed = i.to_string();

        hasher.input(input);
        hasher.input(seed.as_bytes());
        hasher.result(&mut hash);
        
        if hash[0] == 0 && hash[1] == 0 {
            if part1 == 0 && hash[2] <= 0x0f {
                part1 = i;
                println!(" Part 1: {}", part1);
            }
            if hash[2] == 0 {
                println!(" Part 2: {}", i);
                break;
            }
        }

        hasher.reset();
    }
}
