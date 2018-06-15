
pub mod learn {
    use std::io;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::fs::OpenOptions;

    pub fn util_file(path: String) -> Result<(), String> {
        // Подсчет слов в строке – подсчитывает количество отдельных слов в строке.
        // Для дополнительной сложности прочитайте эти строки из текстового файла и сгенерируйте сводку
        let mut file = match OpenOptions::new()
            .read(true)
            .write(true)
            .open(path) {
            Ok(t) => t,
            Err(err) => return Err(err.to_string()),
        };
        let reader = BufReader::new(file.try_clone().unwrap());
        let lines_iter = reader.lines();
        let mut count_row: i32 = 0;
        let mut row_count = Vec::new();
        for item in lines_iter {
            let row = item.unwrap();
            if !row.is_empty() {
                count_row += 1;
                let mut temp_count_word: i32 = 1;
                for symbol in row.chars() {
                    if symbol == ' ' {
                        temp_count_word += 1;
                    }
                }
                row_count.push((row.len(), temp_count_word));
            }
        }
        let mut result = "\n\n-----------".to_string();
        result = result + "\nКоличество строк: " + &*count_row.to_string();
        let mut i = 1;
        for item in row_count {
            result = result + "\nСимволов в " + &*i.to_string() + " строке: " + &*item.0.to_string();
            result = result + " ||| Слов: " + &*item.1.to_string();
            i += 1;
        }
        //file.write(result.as_bytes()).unwrap();
        match file.write(result.as_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => return Err(err.to_string())
        }
    }

    pub fn string_reverse() {
        // Функция принимает строку (read_line) и реверсирует её
        println!("Введите строку:");
        let mut varstring = String::new();
        io::stdin().read_line(&mut varstring).expect("Не удалось прочитать строку");
        let mut pt = String::new();
        for item in varstring.chars().rev() {
            pt = pt + &item.to_string();
        }
        println!("{}", pt);
    }
}
