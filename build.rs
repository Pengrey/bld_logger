mod bld_logger {
    #[macro_export]
    macro_rules! info {
        ($content:expr) => {
            println!("cargo::warning=\r[{}] {}", "\x1b[34m*\x1b[0m", $content);
        };
    }

    #[macro_export]
    macro_rules! success {
        ($content:expr) => {
            println!("cargo::warning=\r[{}] {}", "\x1b[32m+\x1b[0m", $content);
        };
    }

    #[macro_export]
    macro_rules! debug {
        ($content:expr) => {
            println!("cargo::warning=\r[{}] {}", "\x1b[33m^\x1b[0m", $content);
        };
    }

    #[macro_export]
    macro_rules! error {
        ($content:expr) => {
            println!("cargo::warning=\r[{}] {}", "\x1b[31m-\x1b[0m", $content);
        };
    }
}

fn main() {
    info!("Example of a info log.");
    success!("Example of a success log.");
    debug!("Example of a debug log.");
    error!("Example of a debug log.");
}