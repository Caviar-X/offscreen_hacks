use anyhow::*;
use winapi::um::winuser::*;
pub fn keyboard_press(key: u16) -> anyhow::Result<()> {
    unsafe {
        let mut inputs: [INPUT; 2] = std::mem::zeroed();
        inputs[0].type_ = INPUT_KEYBOARD;
        inputs[0].u.ki_mut().wVk = key;
        inputs[1].type_ = INPUT_KEYBOARD;
        inputs[1].u.ki_mut().wVk = key;
        inputs[1].u.ki_mut().dwFlags = KEYEVENTF_KEYUP;
        let usent = SendInput(inputs.len() as u32, &mut inputs as *mut INPUT, 40);
        if usent != inputs.len() as u32 {
            return Err(anyhow!("Failed to press the keyboard"));
        }
    };
    Ok(())
}
/// Detect which key is pressed
/// TODO: Maybe Add OEM keys later?
pub fn keyboard_pressed() -> Option<u16> {
    for i in 8..=190 {
        if unsafe { GetAsyncKeyState(i) } & 0x001 == 1 {
            return Some(i as u16);
        }
    }
    None
}
