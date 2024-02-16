use rfind::{execute, get_args};

fn main() {
    let input = get_args();

    // println!("{:?}", input);
    for path in &input.paths {
        execute(path, &input)
    }
}
