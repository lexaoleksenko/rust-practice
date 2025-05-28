use rand::Rng;

/// Функція генерує вектор із `n` кораблів, які в сумі мають однаковий вантаж.
/// Кожен корабель початково має `target`, але ми розкидаємо трохи ваги випадково.
fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let target = rng.gen_range(10..100);
    let mut result = vec![target; n];

    for _ in 0..(n / 2) {
        let i = rng.gen_range(0..n);
        let j = rng.gen_range(0..n);
        let delta = rng.gen_range(1..=target.min(5));
        if result[i] >= delta {
            result[i] -= delta;
            result[j] += delta;
        }
    }

    result
}

/// Функція рахує мінімальну кількість рухів, щоб всі кораблі мали однакову вагу.
/// Один рух — це перенесення 1 одиниці вантажу з одного корабля на інший.
/// Якщо неможливо зрівняти (сума не ділиться), повертає -1.
fn count_permutation(shipments: &Vec<u32>) -> isize {
    let n = shipments.len();
    let total: u32 = shipments.iter().sum();

    if total as usize % n != 0 {
        return -1; // Нерівномірний розподіл неможливий
    }

    let avg = total as usize / n;
    let mut moves = 0;

    for &ship in shipments {
        if ship as usize > avg {
            moves += ship as usize - avg; // тільки ті, хто перевищує середнє, будуть передавати
        }
    }

    moves as isize
}

/// Приклад запуску програми
fn main() {
    // Тест №1: 
    let s1 = vec![1, 1, 1, 1, 6];
    println!("Shipments: {:?}\nMoves required: {}\n", s1, count_permutation(&s1)); // => 4

    // Тест №2: 
    let s2 = vec![9, 3, 7, 2, 9];
    println!("Shipments: {:?}\nMoves required: {}\n", s2, count_permutation(&s2)); // => 7

    // Тест №3: неможливо розподілити порівну
    let s3 = vec![1, 2, 3];
    println!("Shipments: {:?}\nMoves required: {}\n", s3, count_permutation(&s3)); // => -1

    // Тест №4: згенерований вектор з правильним балансом
    let s4 = gen_shipments(10);
    println!("Generated shipments: {:?}\nMoves required: {}\n", s4, count_permutation(&s4));
}