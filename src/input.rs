use std::fs::read_to_string;

pub fn drop_meta_info(text: String) -> String {
    let rest = text
        .lines()
        .skip_while(|line| !line.starts_with(";"))
        .skip(1);
    rest.collect::<Vec<&str>>().join("\n")
}

pub fn read_input(in_path: String) -> String {
    let text = read_to_string(in_path).unwrap();
    drop_meta_info(text)
}
