use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use std::str::FromStr;
use std::{env, f64};
use regex::Regex;
use string_calculator::eval_f64;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 7 {
        println!("Не хватает аргументов.");
        help();
        return
    }

    if args.len() > 8 {
        println!("Слишком много аргументов. Лишние аргументы проигнорированы.");
        help()
    }

    if args[1] == "-h" || args[1] == "--help" {
        help();
        return
    }

    let full_equation = &args[1];

    // equation.0 - left
    // equation.1 - right
    let equation = split_equation(full_equation);

    let min_variable = Decimal::from_str(&args[2]).unwrap();

    let max_variable = Decimal::from_str(&args[3]).unwrap();

    let step = Decimal::from_str(&args[4]).unwrap();

    if step <= Decimal::try_from(0.0).unwrap() {
        println!("Точность = {step}. Точность не может быть равна 0 или меньше.");
        help();
        return
    }

    let try_find_near_answer = bool::from_str(&args[5]).unwrap();

    let silent = bool::from_str(&args[6]).unwrap();
    // end arguments

    let binding = find_variable(full_equation).unwrap();
    let variable: char = binding.as_str().chars().next().unwrap();

    println!("Буду подбирать {variable} в: {full_equation}, \
              мин. {variable} = {min_variable}, макс. {variable} = {max_variable}, шаг = {step}");

    let mut i: u64 = 0;

    let mut answer: Decimal = min_variable;

    let mut answers: Vec<f64> = Vec::new();

    let mut founded_answer: bool = false;

    let mut tmp_e: (String, String) = (String::new(), String::new());

    let mut left = 0.0;
    let mut right = 0.0;

    let mut all_differences: Vec<f64> = Vec::new();
    let mut all_answers: Vec<f64> = Vec::new();

    while answer < max_variable
    {
        i += 1;

        answer += step; // округляем до двух знаков после запятой

        tmp_e.0 = equation.0.replace(variable, format!("({})", answer.to_string()).as_str());
        tmp_e.1 = equation.1.replace(variable, format!("({})", answer.to_string()).as_str());

        left = eval_f64(tmp_e.0, 0.0).unwrap();
        right = eval_f64(tmp_e.1, 0.0).unwrap();

        if left == right {
            founded_answer = true;
            break;
        }

        if try_find_near_answer {
            all_differences.push(left - right);
            all_answers.push(answer.to_f64().unwrap());
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

fn find_variable(equation: &str) -> Option<String> {
    // Регулярное выражение для поиска переменных (букв) в уравнении
    let re = Regex::new(r"[a-zA-Z]").unwrap();

    // Найти переменную
    for cap in re.captures_iter(equation) {
        if let Some(var) = cap.get(0) {
            return Some(var.as_str().to_string());
        }
    }

    None
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
             Если silent = true, то программа работает быстрее так как \
             вывод в stdout медленный и зависит от приложения терминала.");
}
