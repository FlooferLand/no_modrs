use no_modrs::folder_module;

// TODO: Write better tests! They are AWFUL. \
//       If anyone is willing to write better tests, please do!

folder_module!(use has_module);
folder_module!(use standalone);

fn main() {
    has_module::one::one();
    has_module::two::two();
    standalone::one::one();
    standalone::two::two();
}
