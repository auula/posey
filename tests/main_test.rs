use typikon::{cli::Command, utils::Logger};

#[test]
fn test_handle_init_command() {
    // Mock args and log

    let mut log = Logger::console_log();

    // Test handle_init_command
    for cmd in &[
        "build".to_string(),
        "init".to_string(),
        "theme".to_string(),
        "help".to_string(),
    ] {
        match Command::from_str(&cmd) {
            Command::Build => typikon::cli::handle_build_command(&["build".to_string()], &mut log),
            Command::Theme => {
                typikon::cli::handle_build_command(&["--get=xxxxxxx.git".to_string()], &mut log)
            }
            Command::Init => {
                typikon::cli::handle_build_command(&["--path=/xxxxx".to_string()], &mut log)
            }
            Command::Help => typikon::cli::handle_build_command(&["theme".to_string()], &mut log),
            Command::Unknown(_) => typikon::cli::out_banner_string(),
        }
    }
}
