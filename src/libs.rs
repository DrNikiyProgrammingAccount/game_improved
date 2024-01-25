use std::error::Error;
use std::{fs, io};
use log::error;

pub struct Character {
    pub class: String,
    pub pos: (usize, usize),
    pub lives: usize,
    pub coins: usize,
    pub map: Map,
}

pub struct Map {
    pub field: Vec<Vec<char>>,
    pub coins: usize,
    pub bombs: usize,
    pub start_pos: (usize, usize)
}

impl Character {
    pub fn move_to (&mut self, dir: &(isize, isize)) {
        if self.check_borders(&dir) {
            match self.map.field[(dir.1 + self.pos.1 as isize) as usize][(dir.0 + self.pos.0 as isize) as usize] {
                '_' => { self.change_pos(dir); println!() } ,
                '0' => println!("Тут стена, сюда нельзя! Попробуйте ещё раз"),
                '*' => { self.change_pos(dir); self.lives -= 1; println!() },
                '$' => { self.change_pos(dir); self.coins += 1; self.map.coins -= 1; println!()},
                _ => panic!("Проблема с картой")
            }
        }
        else {
            println!("Вы улетели за край карты, осторожнее! Попробуйте ещё раз");
        }
    }
    fn change_pos (&mut self, dir: &(isize, isize)) {
        self.map.field[self.pos.1][self.pos.0] = '_';
        self.map.field[(dir.1 + self.pos.1 as isize) as usize][(dir.0 + self.pos.0 as isize) as usize] = '&';
        self.pos.1 = (dir.1 + self.pos.1 as isize) as usize; self.pos.0 = (dir.0 + self.pos.0 as isize) as usize;
    }
    fn check_borders (&self, dir: &(isize, isize)) -> bool {
        self.pos.0 as isize + dir.0 >= 0
            && self.pos.1 as isize + dir.1 >= 0
            && self.pos.0 as isize + dir.0 < self.map.field[0].len() as isize
            && self.pos.1 as isize + dir.1 < self.map.field.len() as isize
    }
}

impl Map {
    pub fn new (filename: &str) -> Result<Map, Box<dyn Error>> {
        let mut field = vec![];
        let mut coins = 0;
        let mut bombs = 0;
        let mut content = fs::read_to_string(filename).expect("");
        let mut s = content.lines();
        let mut start_pos: (usize, usize) = (0,0);
        for i in s.enumerate() {
            println!("{:?}", i);
            let mut line = Vec::new();
            for j in i.1.chars().enumerate() {
                match j.1 {
                    '$' => {coins += 1; line.push(j.1)}
                    '*' => {bombs += 1; line.push(j.1)}
                    '_' => {line.push(j.1)}
                    '0' => {line.push(j.1)}
                    '&' => {line.push(j.1); start_pos = (j.0, i.0)}
                    _ => ()
                }
            }
            field.push(line);
        }
        Ok(Map { field, coins, bombs, start_pos })
    }
}

pub fn run(character: &mut Character) {
    loop {
        let mut input = String::new();
        break match io::stdin().read_line(&mut input) {
            Ok(_x) => {
                match input.trim().to_ascii_uppercase().as_str() {
                "W" => { print!("{}c", 27 as char); character.move_to(&(0, -1)) },
                "S" => { print!("{}c", 27 as char); character.move_to(&(0, 1)) },
                "A" => { print!("{}c", 27 as char); character.move_to(&(-1, 0)) },
                "D" => { print!("{}c", 27 as char); character.move_to(&(1, 0)) },
                "Q" => { print!("{}c", 27 as char); if character.class == "Jumper".to_string() {character.move_to(&(-2, 0))}
                    else {println!("Вы не можете использовать особенности этого класса")}},
                "E" => { print!("{}c", 27 as char); if character.class == "Jumper".to_string() {character.move_to(&(2, 0))}
                    else {println!("Вы не можете использовать особенности этого класса")}},
                "T" => { print!("{}c", 27 as char); if character.class == "Teleporter".to_string()
                {character.move_to(&(character.map.field[0].len() as isize - 2 * character.pos.0 as isize - 1, character.map.field.len() as isize - 2 * character.pos.1 as isize - 1))}
                    else {println!("Вы не можете использовать особенности этого класса")}},
                "P" => std::process::exit(0),
                _ => {println!("{}cВведите корректный символ, а не {}!", 27 as char, input.to_ascii_uppercase().as_str().trim())}
            }},
            Err(x) => { println!("Ошибка: {}. Попробуйте повторить ввод", x); continue }
        }
    }
}