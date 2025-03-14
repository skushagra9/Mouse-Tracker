use device_query::{DeviceQuery, DeviceState, Keycode, MouseState};
use std::{fs::OpenOptions, time::{SystemTime, UNIX_EPOCH}};
use std::io::Write;
use std::sync::{Arc, Mutex};


pub fn get_mouse_movements(running: &Arc<Mutex<bool>>, task: String) {
    let device_state = DeviceState::new();
    let mut file = OpenOptions::new() // Open or create the file
        .write(true)
        .append(true)
        .create(true)
        .open(format!("mouse_movements_{}.log", task)) // Specify the file name
        .unwrap();
    while *running.lock().unwrap() {
        // Get the state of all keys
        let keys: Vec<Keycode> = device_state.get_keys();

        // Check for modifier keys
        let alt_key_down = keys.contains(&Keycode::LAlt) || keys.contains(&Keycode::RAlt);
        let ctrl_key_down = keys.contains(&Keycode::LControl) || keys.contains(&Keycode::RControl);
        let meta_key_down = keys.contains(&Keycode::LMeta) || keys.contains(&Keycode::RMeta);

        // Get mouse state
        let mouse: MouseState = device_state.get_mouse();
        let button_number = mouse.button_pressed.iter().position(|&b| b).unwrap_or(0);
        let pressed_key = keys.last().map(|k| format!("{:?}", k)).unwrap_or("None".to_string());


        let mouse_x = mouse.coords.0; 
        let mouse_y = mouse.coords.1;

        // Get timestamp
        let happened_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

            writeln!(
                file,
                "altKeyDown: {}, ctrlKeyDown: {}, metaKeyDown: {}, buttonNumber: {}, happenedAt: {}, mouse_x: {}, mouse_y: {}, pressed_key: {}",
                alt_key_down, ctrl_key_down, meta_key_down, button_number, happened_at, mouse_x, mouse_y, pressed_key
            ).unwrap(); 


        // Sleep to prevent excessive CPU usage
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}