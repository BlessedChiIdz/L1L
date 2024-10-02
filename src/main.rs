use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

#[allow(non_snake_case)]
fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
#[allow(non_snake_case)]
#[derive(Debug)]
struct Grammar {
    VT: Vec<String>,
    VN: Vec<String>,
    P: HashMap<String, Vec<String>>,
    S: String,
}
#[allow(non_snake_case)]
fn grammar_input() -> Grammar {
    let mut VT: Vec<String> = input("Введите терминальные символы: ").split_whitespace().map(|s| s.to_string()).collect();
    let mut VN: Vec<String> = input("Введите нетерминальные символы: ").split_whitespace().map(|s| s.to_string()).collect();
    let n: usize = input("Введите количество правил: ").trim().parse().unwrap();
    let mut P: HashMap<String, Vec<String>> = HashMap::new();
    for _ in 0..n {
        let r = input("Введите терминал правила: ");
        let rs: Vec<String> = input("Введите правило: ").split_whitespace().map(|s| s.to_string()).collect();
        P.insert(r, rs);
    }
    let S = input("Введите целевой символ: ");
    VT.push("_".to_string());
    Grammar { VT, VN, P, S }
}
#[allow(non_snake_case)]
fn count_non_term_sym(grammar: &Grammar, sequence: &String) -> usize {
    let mut length = 0;
    for sym in sequence.chars() {
        if grammar.VT.contains(&sym.to_string()) {
            length += 1;
        }
    }
    length
}
#[allow(non_snake_case)]
fn main() {
    let rulesI = input("Моя грамматика - 1, ввод грамматики - 2");
    let mut data:Grammar;
    if rulesI == "1"{
        data = Grammar {
            VT: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            VN: vec!["A".to_string(), "B".to_string(), "C".to_string()],
            P: HashMap::from([
                ("A".to_string(), vec!["aBbbC".to_string()]),
                ("B".to_string(), vec!["aaBb".to_string(), "".to_string()]),
                ("C".to_string(), vec!["cC".to_string(), "".to_string()]),
            ]),
            S: "A".to_string(),
        };
    }else{
        data = grammar_input();
    }
    //let (left_border, right_border): (usize, usize) = input("Введите диапазон цепочек ОТ и ДО\n").split_whitespace().map(|s| s.parse().unwrap()).next_tuple().unwrap();
    let left_border:usize = input("OT").trim().parse().expect("asd");
    let right_border:usize = input("DO").trim().parse().expect("asd");
    let mut grammar = data;
    let mut rules: Vec<String> = vec![grammar.S.clone()];
    let mut used_sequence = HashSet::new();
    while let Some(sequence) = rules.pop() {
        if used_sequence.contains(&sequence) {
            continue;
        }
        used_sequence.insert(sequence.clone());
        let mut no_term = true;
        for (i, symbol) in sequence.chars().enumerate() {
            if let Some(index) = grammar.VN.iter().position(|x| *x == symbol.to_string()) {
                no_term = false;
                for elem in &grammar.P[&grammar.VN[index]] {
                    let temp = format!("{}{}{}", &sequence[..i], elem, &sequence[i + 1..]);
                    if count_non_term_sym(&grammar, &temp) <= right_border && !rules.contains(&temp) {
                        rules.push(temp);
                    }
                }
            } else if !grammar.VT.contains(&symbol.to_string()) {
                no_term = false;
                println!("Цепочка {} не разрешима", sequence);
                break;
            }
        }
        if no_term && left_border <= sequence.len() && sequence.len() <= right_border {
            println!("{}", sequence);
        }
    }
}