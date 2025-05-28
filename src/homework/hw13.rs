use std::collections::HashSet;

/// Структура точки на площині
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

/// Структура прямокутника — задається лівою верхньою і правою нижньою точками
struct Rectangle {
    a: Point,
    b: Point,
}

/// Повертає фактично зайняту площу — рахує всі унікальні точки, зайняті прямокутниками
fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut occupied: HashSet<(i32, i32)> = HashSet::new();

    for rect in xs {
        // Нормалізуємо координати: a — ліва верхня, b — права нижня
        let x_min = rect.a.x.min(rect.b.x);
        let x_max = rect.a.x.max(rect.b.x);
        let y_min = rect.a.y.min(rect.b.y);
        let y_max = rect.a.y.max(rect.b.y);

        // Проходимо по кожній клітинці прямокутника
        for x in x_min..x_max {
            for y in y_min..y_max {
                occupied.insert((x, y));
            }
        }
    }

    occupied.len() as i32
}

/// Тестові дані згідно з прикладом (три прямокутники)
fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

/// Тест функції — перевіряє, що відповідь дорівнює 60
fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}

fn main() {
    area_occupied_test();
    println!("Test passed ✅");
}