static INFO: &str = "- [INFO]: ";


fn receiver() {
    log_console("Hello there I can receive something.");
}

fn sender() {
    log_console("Hello there I can send something.");
}


fn log_console(data: &str) {
    println!("{}{}", INFO, data);
}

fn main() {
    receiver();
    sender();

    log_console("Finished executing program.");
}
