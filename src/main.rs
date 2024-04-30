mod ide;
mod structs;
mod utils;
mod assemblers;
mod keyboard;
mod coop;
mod keybinds;

fn main() -> std::io::Result<()> {

    let coop_thread = std::thread::spawn(move || {
        coop::start()
    });

    _ = std::thread::spawn(|| {
        ide::run(
            std::env::current_exe()
                .unwrap()
                .display()
                .to_string()
                .rsplit_once(std::path::MAIN_SEPARATOR)
                .unwrap().0
                .to_string()
        )
    });

    match coop_thread.join().unwrap() {
        Ok(_) => {},
        Err(_) => {
            println!("Someone on your local network is using port 6932.");
            std::thread::sleep(std::time::Duration::new(1, 0));
        }
    }
    
    Ok(())
}
