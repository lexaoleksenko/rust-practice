fn main() {
    const TRIANGLES: usize = 6;

    let mut output = String::new();

    for t in 0..TRIANGLES {
        for i in 0..=t {
            let stars = 1 + i * 2;
            let spaces = TRIANGLES * 2 - 1 - i;
            output.extend(std::iter::repeat(' ').take(spaces));
            output.extend(std::iter::repeat('*').take(stars));
            output.push('\n');
        }
    }

    print!("{}", output);
}