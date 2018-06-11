extern crate rand;
use rand::Rng;

const CONSONANTS: [char; 9] = ['r', 'l', 'k', 'θ', 'ç', 'n', 'ʔ', 't', 'h'];
const VOWELS: [char; 4] = ['a', 'i', 'ɒ', 'ε'];

macro_rules! seq {
    ($($e:expr),*) => ({
        let mut v = vec![$($e),*];
        Structure::Seq(v)
    })
}

macro_rules! opt {
    ($e:expr) => (Structure::Opt(Box::new($e)));
    ($e:expr, $($es:expr),+) => (opt!(seq![$e, $($es),+]));
}

// Point as in Code Point, didn't know what else to call it since it has optional characters
#[derive(Debug)]
enum Structure {
    C, V,
    Opt(Box<Structure>),
    Seq(Vec<Structure>),
}

impl Structure {
    fn gen_word(&self) -> Vec<char> {
        use Structure::*;
        match self {
            C => vec!(random_char(&CONSONANTS)),

            V => vec!(random_char(&VOWELS)),

            Opt(s) => if rand::random() { s.gen_word() } else { Vec::new() },

            Seq(ps) => ps.iter().flat_map(Structure::gen_word).collect()
        }
    }

    fn to_string(&self) -> String {
        self.gen_word().iter().collect()
    }
}

fn random_char(input: &[char]) -> char {
    let mut rng = rand::thread_rng();
    *rng.choose(input).unwrap()
}

fn print_words(structure: &Structure, count: u32, row_width: u32) {
    for i in 1 ..= count {
        let s = structure.to_string();
        print!("{:7}", s);

        if i % row_width != 0 {
            print!(", ");
        } else {
            print!("\n");
        }
    }
}

fn main() {
    use Structure::*;
    let v = seq![opt!(C), C, V, opt!(C), C, V, opt!(C)];
    print_words(&v, 200, 10);
}