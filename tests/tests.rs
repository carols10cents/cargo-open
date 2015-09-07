use std::env;

#[test]
fn check_env_editor() {
    let editor: Option<&'static str> = option_env!("EDITOR");
    assert!(editor != None);
}

#[test]
fn check_env_cargo_editor() {
    let cargo_editor: Option<&'static str> = option_env!("CARGO_EDITOR");
    assert_eq!(cargo_editor, None);
}

#[test]
fn check_env_visual() {
    let visual: Option<&'static str> = option_env!("VISUAL");
    assert_eq!(visual, None);
}
