use std::str::FromStr;
use std::{env, f64};
use string_calculator::eval_f64;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args[1] == "-h" || args[1] == "--help" {
        help();
        return
    }

    if args.len() < 7 {
        println!("Не хватает аргументов.");
        help();
        return
    }

    if args.len() > 8 {
        println!("Слишком много аргументов. Лишние аргументы проигнорированы.");
        help()
    }

    let full_equation = &args[1];

    // equation.0 - left
    // equation.1 - true
    let equation = split_equation(full_equation);

    let min_x = f64::from_str(&args[2]).unwrap();

    let max_x = f64::from_str(&args[3]).unwrap();

    let step = f64::from_str(&args[4]).unwrap();

    if step <= 0.0 {
        println!("Точность = {step}. Точность не может быть отрицательной или быть 0.");
        help();
        return
    }

    let try_find_near_answer = bool::from_str(&args[5]).unwrap();

    let silent = bool::from_str(&args[6]).unwrap();

    println!("Буду подбирать неизвестную переменную в: {full_equation}, мин. x = {min_x}, макс. x = {max_x}, шаг = {step}");
    // end arguments


    let mut tmp: f64 = min_x;

    let mut i = 0;

    let mut answer: f64 = 0.0;

    let mut founded_answer: bool = false;

    let mut tmp_e: (String, String) = (String::new(), String::new());

    let mut left = 0.0;
    let mut right = 0.0;

    let mut all_differences: Vec<f64> = Vec::new();
    let mut all_answers: Vec<f64> = Vec::new();

    while tmp < max_x
    {
        i += 1;
        tmp += step;

        answer = (tmp * (step * 1000000.0)).round() / (step * 1000000.0); // округляем до двух знаков после запятой
        tmp_e.0 = equation.0.replace("x", format!("({})", answer.to_string()).as_str());
        tmp_e.1 = equation.1.replace("x", format!("({})", answer.to_string()).as_str());

        left = eval_f64(tmp_e.0, 0.0).unwrap();
        right = eval_f64(tmp_e.1, 0.0).unwrap();

        if left == right {
            founded_answer = true;
            break;
        }

        if try_find_near_answer {
            all_differences.push(left - right);
            all_answers.push(answer);
        }

        if !silent {
            println!("{answer}: неправильно");
        }
    }

    if founded_answer {
        println!("Подобран ответ: {answer}, за {i} итераций")
    }
    else {
        println!("Не получилось подобрать ответ за {i} итераций и с точностью {step}.");

        if try_find_near_answer {
            println!("Поиск ближайшего ответа...");
            let x = find_near_answer_and_difference(&mut all_differences, &all_answers);
            println!("Найден ближайший ответ: {}, разница: {}", x.0, x.1)
        }

        if !try_find_near_answer {
            println!("Поиск ближайшего ответа отключен. Что бы его включить задайте пятый аргумент как true или 1.")
        }
    }
}

fn find_near_answer_and_difference(all_differences: &mut Vec<f64>, all_answers: &Vec<f64>) -> (f64, f64) {
    let mut near_answer: f64 = 0.0;

    // Преобразуем все числа в векторе в положительные
    for difference in all_differences.iter_mut() {
        *difference = difference.abs();
    }

    // Находим индекс и значение самого маленького числа в векторе
    let (min_index, min_value) = all_differences
        .iter()
        .enumerate()
        .fold((0, f64::INFINITY), |(min_index, min_value), (index, &value)| {
            if value < min_value {
                (index, value)
            } else {
                (min_index, min_value)
            }
        });

    near_answer = all_answers[min_index];

    (near_answer, min_value)
}

// Функция, которая возвращает часть до и после знака равенства
fn split_equation(equation: &str) -> (String, String) {
    // Разделяем строку на части
    let parts: Vec<&str> = equation.split('=').collect();

    let left_side = parts[0].trim().to_string();  // Часть до равно
    let right_side = parts[1].trim().to_string(); // Часть после равно

    (left_side, right_side)
}

fn help() {
    println!("Использование: <уравнение> <минимальный X> <максимальный X> <точность>(больше 0) \
             <Искать ли ближайший ответ>(true или false) <silent>(true или false) \n\
             Внимание: поиск ближайшего ответа занимает оперативную память. \
             Если silent = true, то программа работает немного быстрее.");
}
