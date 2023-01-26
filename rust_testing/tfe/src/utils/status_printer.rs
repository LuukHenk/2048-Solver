pub fn print_status(message: &str, current_status: usize, goal: usize) {
    print!("                                        \r");
    print!("{}{:#?}/{:#?}", message, current_status, goal);
}