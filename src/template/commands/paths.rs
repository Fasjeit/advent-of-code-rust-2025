#[macro_export]
macro_rules! day_path {
    () => {
        "data/day_{0}"
    };
}

#[macro_export]
macro_rules! input_path {
    () => {
        "data/day_{0}/input.txt"
    };
}

#[macro_export]
macro_rules! examples_path {
    () => {
        "data/day_{0}/examples"
    };
}

#[macro_export]
macro_rules! example_path {
    () => {
        "data/day_{0}/examples/example_1.txt"
    };
}

#[macro_export]
macro_rules! module_path {
    () => {
        "src/bin/{0}.rs"
    };
}

#[macro_export]
macro_rules! puzzle_path {
    () => {
        "data/day_{0}/puzzle.md"
    };
}
