use rstest::rstest;
use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    //input = input.trim_end().to_string();
    input.pop();
    match unpacking_string(input) {
        Ok(str) => println!("{}", str),
        Err(err) => println!("{}", err),
    }
}

// Функция распаковки строки
fn unpacking_string(input: String) -> Result<String, String> {
    let mut previous = '0'; // переменная хранящая предыдущий символ
    let mut flag = false; // флаг, сигнализирующий о том что символ необходимо экранировать
    let mut cycle_flag = false; // флаг, сигнализирующий о том что предыдущий символ повторяющийся
    let mut repetitions = String::new(); // переменная хранящая количество повторений
    let mut output = String::new();

    // проверка на то что первый символ в строке не является числом, в таком случае строка не корректна
    if let Some(first_char) = input.chars().nth(0) {
        if first_char as u8 >= b'1' && (first_char as u8) <= b'9' {
            return Err("incorrect string".to_string());
        }
    }

    // Итерируемся по каждому символу в строке
    for ch in input.chars() {
        // Если текущий символ - цифра
        if ch as u8 >= b'1' && (ch as u8) <= b'9' {
            // Если нет флага экранирования, добавляем его к строке количества повторений
            if !flag {
                repetitions.push(ch);
                cycle_flag = true; // устанавливаем флаг цикла, сигнализирующий о том что последний найденный символ для вывода необходимо выводить в цикле
                continue; // переходим на следующую итерацию, т.к. число повторений символа может быть многозначным
            } else {
            }
        }

        // если установлен флаг цикла, добавляем последний найденный символ в результат n-1 раз, т.к. 1 раз он был добавлен при нахождении
        if cycle_flag {
            for _ in 1..repetitions.parse().unwrap() {
                output.push(previous);
            }
            cycle_flag = false;
            repetitions.clear();
        }

        // если текущий символ - "\" то следущий необходимо экранировать
        if ch == '\\' {
            if previous != '\\' || !flag {
                // проверяем, не экранирован ли текущий символ
                flag = true; // устанавливаем флаг экранирования
                previous = ch;
                continue;
            }
        }

        output.push(ch);
        previous = ch;
        flag = false;
    }

    // если строка оканчивается на неэкранированное число, добавляем к результату оставшиеся символы
    if cycle_flag {
        for _ in 1..repetitions.parse().unwrap() {
            output.push(previous);
        }
        repetitions.clear();
    }
    // если остался поднят флаг экранирования, значит строка оканчивается на неэкранированный слэш, такая строка является некорректной
    if flag {
        return Err("incorrect string".to_string());
    }
    Ok(output)
}

// Модульные unit-тесты
#[rstest]
#[case("a4bc2d5e", Ok("aaaabccddddde".to_string()))]
#[case("abcd",  Ok("abcd".to_string()))]
#[case("45", Err("incorrect string".to_string()))]
#[case("", Ok("".to_string()))]
#[case("qwe45", Ok("qweeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee".to_string()))]
#[case("q1w2e3r4t5", Ok("qwweeerrrrttttt".to_string()))]
#[case("qwe\\4\\5", Ok("qwe45".to_string()))]
#[case("qwe\\45", Ok("qwe44444".to_string()))]
#[case("qwe\\0", Ok("qwe0".to_string()))]
#[case("qwe\\", Err("incorrect string".to_string()))]
#[case("qwe\\\\\\\\\\\\", Ok("qwe\\\\\\".to_string()))]
#[case("qwe\\45\\5", Ok("qwe444445".to_string()))]
#[case("qwe\\45\\", Err("incorrect string".to_string()))]
#[case("qwe\\45\\5\\", Err("incorrect string".to_string()))]

fn test_unpacking_string(#[case] a: String, #[case] expected: Result<String, String>) {
    assert_eq!(unpacking_string(a), expected);
}
