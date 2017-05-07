extern crate bcmp;
#[macro_use]
extern crate clap;

use std::char;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{Read, Write, stderr};
use std::usize;

use bcmp::{AlgoSpec, longest_common_substrings, unique_strings, patch_set};

fn hex_dump(data: &[u8], start: usize, end: usize) {
    let mut i = start - (start % 16);
    while i < end {
        let mut j = 0;
        // offset
        print!("\t{:08x}: ", i);
        // hex display
        while i + j < start {
            print!("   ");
            j += 1;
        }
        while j < 16 && i + j < end {
            print!("{:02x} ", data[i + j]);
            j += 1;
        }
        while j < 16 {
            print!("   ");
            j += 1;
        }
        j = 0;
        // asci display
        while i + j < start {
            print!(" ");
            j += 1;
        }
        while j < 16 && i + j < end {
            if data[i + j] >= b' ' && data[i + j] <= b'~' {
                print!("{}", char::from_u32(data[i + j] as u32).unwrap());
            }
            else {
                print!(".");
            }
            j += 1;
        }
        while j < 16 {
            print!(" ");
            j += 1;
        }
        println!("");
        i += 16;
    }
}

fn load_file(path: &str) -> Vec<u8> {
    let mut buf = Vec::<u8>::new();
    match File::open(path) {
        Ok(mut f) => match f.read_to_end(&mut buf) {
            Ok(_) => buf,
            Err(e) => {
                writeln!(stderr(), "Failed to load file \"{}\" content: {}", path, e).unwrap();
                panic!();
            }
        },
        Err(e) => {
            writeln!(stderr(), "Failed to open file \"{}\": {}", path, e).unwrap();
            panic!();
        }
    }
}

fn str_to_algospec(s: &str) -> AlgoSpec {
    if s.len() == 0 {
        writeln!(stderr(), "Empty AlgoSpec").unwrap();
        panic!();
    }
    let mut chars = s.chars();
    match chars.next().unwrap() {
        'h' => {
            match chars.as_str().parse::<usize>() {
                Ok(n) => AlgoSpec::HashMatch(n),
                Err(_) => {
                    writeln!(stderr(), "Invalid AlgoSpec \"{}\": a number should follow h", s).unwrap();
                    panic!();
                }
            }
        }
        't' => {
            match chars.as_str().parse::<usize>() {
                Ok(n) => AlgoSpec::TreeMatch(n),
                Err(_) => {
                    writeln!(stderr(), "Invalid AlgoSpec \"{}\": a number should follow t", s).unwrap();
                    panic!();
                }
            }
        }
        _ => {
            writeln!(stderr(), "Invalid AlgoSpec \"{}\": either h or t followed by a number", s).unwrap();
            panic!();
        }
    }
}

fn print_lcs(first: &[u8], second: &[u8], algo_spec: AlgoSpec, number: usize, hexdump: bool) {
    let top = longest_common_substrings(first, second, algo_spec, number);
    for i in 0..top.len() {
        print!("{}: Length: {}, ", i + 1, top[i].length);
        print!("first file 0x{:x} to 0x{:x}, ", top[i].first_pos, top[i].first_end());
        println!("second file 0x{:x} to 0x{:x}", top[i].second_pos, top[i].second_end());
        if hexdump {
            hex_dump(first, top[i].first_pos, top[i].first_end());
            println!("");
        }
    }
}

fn print_us(first: &[u8], second: &[u8], algo_spec: AlgoSpec, number: usize, hexdump: bool) {
    let mut uniques = unique_strings(first, second, algo_spec);
    uniques.sort_by(|a, b|
        if (a.1 - a.0) < (b.1 - b.0) {
            Ordering::Greater
        }
        else if (a.1 - a.0) > (b.1 - b.0) {
            Ordering::Less
        }
        else {
            Ordering::Equal
        }
    );
    uniques.truncate(number);
    for i in 0..uniques.len() {
        print!("{}: Length: {}, ", i + 1, uniques[i].1 - uniques[i].0);
        println!("second file 0x{:x} to 0x{:x}", uniques[i].0, uniques[i].1);
        if hexdump {
            hex_dump(second, uniques[i].0, uniques[i].1);
            println!("");
        }
    }
}

fn print_ps(first: &[u8], second: &[u8], algo_spec: AlgoSpec, hexdump: bool) {
    let patch_set = patch_set(first, second, algo_spec);
    for patch in patch_set {
        print!("[0x{:x}, 0x{:x}]", patch.second_pos, patch.second_end());
        println!(" << [0x{:x}, 0x{:x}]", patch.first_pos, patch.first_end());
        if hexdump {
            hex_dump(first, patch.first_pos, patch.first_end());
            println!("");
        }
    }
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();
    
    if let Some(sub_matches) = matches.subcommand_matches("lcs") {
        let first  = load_file(sub_matches.value_of("first_file").unwrap());
        let second = load_file(sub_matches.value_of("second_file").unwrap());
        let hexdump = sub_matches.is_present("hexdump");
        let number = match sub_matches.value_of("number") {
            Some(s) => match s.parse::<usize>() {
                Ok(n) => n,
                Err(_) => {
                    writeln!(stderr(), "Invalid number of substrings \"{}\"", s).unwrap();
                    panic!();
                }
            },
            None => usize::MAX
        };
        let algo_spec = match sub_matches.value_of("algorithm") {
            Some(s) => str_to_algospec(s),
            None => AlgoSpec::HashMatch(4)
        };
        print_lcs(&first, &second, algo_spec, number, hexdump);
    }
    else if let Some(sub_matches) = matches.subcommand_matches("us") {
        let first  = load_file(sub_matches.value_of("first_file").unwrap());
        let second = load_file(sub_matches.value_of("second_file").unwrap());
        let hexdump = sub_matches.is_present("hexdump");
        let number = match sub_matches.value_of("number") {
            Some(s) => match s.parse::<usize>() {
                Ok(n) => n,
                Err(_) => {
                    writeln!(stderr(), "Invalid number of substrings \"{}\"", s).unwrap();
                    panic!();
                }
            },
            None => usize::MAX
        };
        let algo_spec = match sub_matches.value_of("algorithm") {
            Some(s) => str_to_algospec(s),
            None => AlgoSpec::HashMatch(4)
        };
        print_us(&first, &second, algo_spec, number, hexdump);
    }
    else if let Some(sub_matches) = matches.subcommand_matches("ps") {
        let first  = load_file(sub_matches.value_of("first_file").unwrap());
        let second = load_file(sub_matches.value_of("second_file").unwrap());
        let hexdump = sub_matches.is_present("hexdump");
        let algo_spec = match sub_matches.value_of("algorithm") {
            Some(s) => str_to_algospec(s),
            None => AlgoSpec::HashMatch(4)
        };
        print_ps(&first, &second, algo_spec, hexdump);
    }
}
