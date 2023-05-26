use anyhow::*;
use winapi::shared::windef::POINT;
use winapi::um::winuser::*;
fn get_mouse_pos() -> (i32, i32) {
    let mut point = POINT { x: 0, y: 0 };
    unsafe { GetCursorPos(&mut point) };
    (point.x, point.y)
}
fn get_screen_size() -> (i32, i32) {
    let (x, y);
    unsafe {
        x = GetSystemMetrics(0);
        y = GetSystemMetrics(1);
    };
    (x, y)
}
/// Returns true if the mouse is in the border of the screen with given width
pub fn is_in_zone(width: i32) -> bool {
    let (x, y) = get_mouse_pos();
    let (sx, sy) = get_screen_size();
    ((x >= 0 && x <= width) || (x >= sx - width && x <= sx))
        || ((y >= 0 && y <= width) || (y >= sy - width && y <= sy))
}
/// The Mouse Keys
#[derive(Debug)]
pub enum MouseKey {
    /// The Left Button
    LeftButton,
    /// The Right Button
    RightButton,
    /// Yes, you.The house of the dead : Overkill
    /// FIXME: fix the problem.FUCK YOU WINDOWS API
    MiddleButton,
}
/// Emulates the mouse press event using windows api
pub fn mouse_press(key: MouseKey) -> Result<()> {
    unsafe {
        let mut inputs: [INPUT; 2] = std::mem::zeroed();
        inputs[0].type_ = INPUT_MOUSE;
        let pos = get_mouse_pos();
        // see MSDN
        match key {
            MouseKey::LeftButton => {
                inputs[0].u.mi_mut().dwFlags = MOUSEEVENTF_LEFTDOWN;
                inputs[0].u.mi_mut().dx = pos.0;
                inputs[0].u.mi_mut().dy = pos.1;
                inputs[0].u.mi_mut().mouseData = 0;
                inputs[0].u.mi_mut().time = 0;
                inputs[0].u.mi_mut().dwExtraInfo = 0;
                inputs[1].u.mi_mut().dwFlags = MOUSEEVENTF_LEFTUP;
                inputs[1].u.mi_mut().dx = pos.0;
                inputs[1].u.mi_mut().dy = pos.1;
                inputs[1].u.mi_mut().time = 0;
                inputs[1].u.mi_mut().mouseData = 0;
                inputs[1].u.mi_mut().dwExtraInfo = 0;
            }
            MouseKey::MiddleButton => {
                /*
                inputs[0].u.mi_mut().dwFlags = MOUSEEVENTF_WHEEL;
                inputs[0].u.mi_mut().dx = pos.0;
                inputs[0].u.mi_mut().dy = pos.1;
                inputs[0].u.mi_mut().mouseData = WHEEL_DELTA as u32;
                inputs[0].u.mi_mut().time = 0;
                inputs[0].u.mi_mut().dwExtraInfo = 0;
                THIS DOESNT WORK
                WHYYYYYYYYYYYYYYYYYYYY
                */
                todo!("FUCK YOU WINDOWS API");
            }
            MouseKey::RightButton => {
                inputs[0].u.mi_mut().dwFlags = MOUSEEVENTF_RIGHTDOWN;
                inputs[0].u.mi_mut().dx = pos.0;
                inputs[0].u.mi_mut().dy = pos.1;
                inputs[0].u.mi_mut().mouseData = 0;
                inputs[0].u.mi_mut().time = 0;
                inputs[0].u.mi_mut().dwExtraInfo = 0;
                inputs[1].u.mi_mut().dwFlags = MOUSEEVENTF_RIGHTUP;
                inputs[1].u.mi_mut().dx = pos.0;
                inputs[1].u.mi_mut().dy = pos.1;
                inputs[1].u.mi_mut().mouseData = 0;
                inputs[1].u.mi_mut().time = 0;
                inputs[1].u.mi_mut().dwExtraInfo = 0;
            }
        }
        let usent = SendInput(inputs.len() as u32, &mut inputs as *mut INPUT, 40);
        if usent != inputs.len() as u32 {
            return Err(anyhow!("Failed to send mouse input"));
        }
    };
    Ok(())
}
