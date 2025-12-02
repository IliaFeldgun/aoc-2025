use crate::mat_lib::loader::load_file;

pub fn echo_day(number: i32, part: i32) {
    println!("Day {number}/{part}");
}

pub fn echo_day_example(number: i32, part: i32) {
    match get_day_example(number, part) {
        Err(e) => eprintln!("{e}"),
        Ok(example) => println!("{example}"),
    }
}

pub fn get_day_example(number: i32, _part: i32) -> Result<String, String> {
    let path = format!("./inputs/{number}/1/example");
    load_file(&path)
}

pub fn get_my_day(number: i32, _part: i32) -> Result<String, String> {
    let path = format!("./inputs/{number}/1/my");
    load_file(&path)
}

pub fn get_day_example_result(number: i32, part: i32) -> Result<i128, String> {
    let path = format!("./inputs/{number}/{part}/example.output.expect");
    get_result(path)
}

pub fn get_my_result(number: i32, part: i32) -> Result<i128, String> {
    let path = format!("./inputs/{number}/{part}/my.output.expect");
    get_result(path)
}
fn get_result(path: String) -> Result<i128, String> {
    match load_file(&path) {
        Err(e) => Err(e),
        Ok(result) => result
            .trim()
            .parse::<i128>()
            .map_err(|e| format!("Failed to parse {result}: {e}")),
    }
}
