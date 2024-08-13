use doryen_rs::{Color, DoryenApi, ScanCode};

// menu controls from keyboard
pub fn menu_control(& self, api: &mut dyn DoryenApi) {
    let input = api.input();
    let mut mov = (0, 0);

    mov.0 = match (input.key(ScanCode::Left) || input.key(ScanCode::A), 
                   input.key(ScanCode::Right) || input.key(ScanCode::D)) {
        (true, _,) => -1,
        (_, true) => 1,
        _ => 0,
    };

    mov.0 = match (input.key(ScanCode::Left) || input.key(ScanCode::A), 
                   input.key(ScanCode::Right) || input.key(ScanCode::D)) {
        (true, _,) => -1,
        (_, true) => 1,
        _ => 0,
    };

    mov.1 = match (input.key(ScanCode::Up) || input.key(ScanCode::W), 
                   input.key(ScanCode::Down) || input.key(ScanCode::S)) {
        (true, _,) => -1,
        (_, true) => 1,
        _ => 0,
    };
    mov
}