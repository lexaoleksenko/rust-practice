fn main() {
    const W: usize = 30; 
    const H: usize = 15;

    let mut output = String::new();

    for y in 0..H {
        for x in 0..W {
            if y == 0 || y == H - 1 || x == 0 || x == W - 1 {
                // Рамка
                output.push('*');
            } else if x * H == y * W || (W - x - 1) * H == y * W {
                // Діагоналі
                output.push('*');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    print!("{}", output);
}