use rfind::{execute, get_args};

fn main() {
    let input = get_args();

    for path in &input.paths {
        execute(path, &input)
    }
}
