use itertools::Itertools;

/// Обчислення значення слова за мапою літер
fn word_to_number(word: &str, map: &std::collections::HashMap<char, u32>) -> u32 {
    word.chars().fold(0, |acc, c| acc * 10 + map[&c])
}

/// Основна функція пошуку рішень
fn solve() {
    let letters = ['m', 'u', 'x', 'a', 's', 'l', 'o', 'n'];
    let digits = (1..=8).collect::<Vec<u32>>();
    let mut count = 0;

    for perm in digits.iter().permutations(8).unique() {
        let map = letters.iter().copied().zip(perm.into_iter().copied()).collect::<std::collections::HashMap<_, _>>();

        let muha = word_to_number("muxa", &map);
        let a = map[&'a'];
        let slon = word_to_number("slon", &map);

        if muha * a == slon {
            println!(
                "{:>4}\n×{:>4}\n-----\n{:>5}\n",
                muha, a, slon
            );
            count += 1;
        }
    }

    println!("Кількість розв’язків: {}", count);
}

fn main() {
    solve();
}

// Задача має 2 розв’язки.