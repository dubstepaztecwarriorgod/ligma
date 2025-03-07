use raylib::color::{self, Color};


#[derive(Debug, Clone, Copy)]
pub enum LigmaColor {
    Red,
    Green,
    Blue
}

type LigmaShape = Vec<Vec<LigmaColor>>;

impl From<&str> for LigmaColor {
    fn from(value: &str) -> Self {
        match value {
            "red" | "rojo" | "rouge" | "rot" | "rosso" | "czerwony" | "красный" | "赤" | "أحمر" | "kırmızı" |
            "červená" | "vermelho" | "чырвоны" | "raudona" | "punane" | "piros" | "červený" | "rdeča" | "crvena" |
            "кызыл" | "червоний" | "röt" | "мөрөн" | "красен" | "טָדוּם" | "червен" | "แดง" | "tsuga" |
            "červeno" | "merah" | "pula" => Self::Red,
            "green" | "verde" | "vert" | "grün" | "zielony" | "зелёный" | "緑" | "أخضر" | "yeşil" |
            "zelená" | "žalia" | "roheline" | "zöld" | "зелений" | "grønn" | "მწვანე" | "зеленo" | "יָרוֹק" |
            "зелено" | "เขียว" | "tsaga" | "zelen" | "hijau" | "bului" => Self::Green,
            "blue" | "azul" | "bleu" | "blau" | "blu" | "niebieski" | "синий" | "青" | "أزرق" | "mavi" |
            "modrá" | "сіні" | "mėlyna" | "sinine" | "kék" | "синій" | "modro" | "plava" |
            "көк" | "блакитний" | "blå" | "ლურჯი" | "синo" | "כָּחוֹל" | "синьо" | "น้ำเงิน" | "koke" |
            "sinez" | "biru" | "ula" => Self::Blue,
            _ => panic!("Not a real colkor bro"),
        }
    }
}

impl From<LigmaColor> for color::Color {
    fn from(value: LigmaColor) -> Self {
        match value {
            LigmaColor::Red => Self::RED,
            LigmaColor::Green => Self::GREEN,
            LigmaColor::Blue => Self::BLUE,
            _ => unreachable!()
        }
    }
}

pub fn parse(input: String) -> LigmaShape {
    let ligma: LigmaShape = input
        .lines()
        .map(|line| { 
            line
                .split_whitespace()
                .map(|lig| LigmaColor::from(lig))
                .collect() 
        })
        .collect();

    if is_valid_square(&ligma) { ligma } else { panic!("Not a square bruh") }
}

pub fn is_valid_square(ligma_shit: &LigmaShape) -> bool {
    ligma_shit
        .iter()
        .skip(1)
        .all(|row| row.len() == ligma_shit[0].len())
}

// Memory leaks in rust are mandotory
pub fn leak_memory() {
    Box::leak(Box::new([0u8; 0x400]));
}