use std::fs::File;

use backend::AuthStore;
use daemonize::Daemonize;

mod backend;

slint::include_modules!();
fn main() {
    // Token Store
    let etst = AuthStore::new("public/LoginInfo".into());

    #[cfg(not(debug_assertions))]
    {
        #[cfg(unix)]
        {
            let stdout = File::create("/tmp/record.out").unwrap();
            let stderr = File::create("/tmp/record.err").unwrap();

            let daemonize = Daemonize::new()
                .pid_file("/tmp/record.pid")
                .stdout(stdout)
                .stderr(stderr);

            match daemonize.start() {
                Ok(_) => println!("Daemon started"),
                Err(e) => eprintln!("Error, {}", e),
            }
        }
    }

    let ui = MainWindow::new().unwrap();

    let form = ui.global::<SignInGlobal>();
    form.on_tokenSubmit(|test| {
        println!("{:#?}", test);
    });

    ui.set_page(Page::Main);

    ui.run().unwrap();
}
