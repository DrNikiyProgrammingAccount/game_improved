mod libs;
use std::io;
use crate::libs::Character;
use crate::libs::Map;

fn main() {
    let map = Map::new("map.txt").unwrap();
    let mut character = Character {
        class: "".to_string(),
        pos: map.start_pos,
        lives: map.bombs / 5 + 1,
        coins: 0,
        map,
    };
    println!("Выберите класс:");
    println!("1. Classic");
    println!("2. Jumper");
    println!("3. Teleporter");
    loop {
        let mut input = String::new();
        break match io::stdin().read_line(&mut input) {
            Ok(_x) => match input.trim().to_ascii_uppercase().as_str() {
                "1" => { print!("{}c", 27 as char); character.class = "Classic".to_string() },
                "2" => { print!("{}c", 27 as char); character.class = "Jumper".to_string() },
                "3" => { print!("{}c", 27 as char); character.class = "Teleporter".to_string() },
                "P" => std::process::exit(0),
                _ => {println!("{}cВведите корректный номер класса, а не {}!", 27 as char, input.to_ascii_uppercase().as_str().trim())}
            },
            Err(x) => { println!("Ошибка: {}. Попробуйте повторить ввод", x); continue }
        }
    }
    println!();
    loop {
        println!("Привео)))");
        println!("Передвижение: WASD, выйти на Q");
        println!("Задача: Собрать все монетки");
        println!("& - Персонаж");
        println!("* - Ловушка");
        println!("$ - Монетка");
        println!("0 - Стена");
        for i in &character.map.field {
            println!("{:?}", i);
        }
        println!("Осталось жизней: {}", character.lives);
        println!("Монет собрано: {}", character.coins);
        println!("Монет осталось: {}", character.map.coins);
        if character.lives == 0 {
            println!("Вы умерли :(");
            io::stdin().read_line(&mut "".to_string());
            break;
        }
        if character.map.coins == 0 {
            println!("Вы выиграли!");
            io::stdin().read_line(&mut "".to_string());
            break;
        }
        libs::run (&mut character);
    }
}