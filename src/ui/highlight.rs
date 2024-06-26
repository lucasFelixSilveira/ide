mod markdown;
mod toml;
mod rust;
mod c;

pub fn language(line: String, file_name: String) -> String {
  let sufix: &str = file_name.rsplit_once('.').unwrap().1;

  match sufix {
    "toml" | "lock" => toml::parse(line),
    "md" => markdown::parse(line),
    "c" | "h" => c::parse(line),
    "rs" => rust::parse(line),
    _ => line
  }
}