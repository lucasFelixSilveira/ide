mod rust;
mod c;

pub fn language(line: String, file_name: String) -> String {
  let sufix: &str = file_name.rsplit_once('.').unwrap().1;

  match sufix {
    "rs" => rust::parse(line),
    "c" | "h" => c::parse(line),
    _ => line
  }
}