extern crate multiinput_zorua;

use multiinput_zorua::*;
fn main() {
    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::Joysticks(XInputInclude::True));
    manager.register_devices(DeviceType::Keyboards);
    manager.register_devices(DeviceType::Mice);
    manager.print_device_list();
    let devices = manager.get_device_list();
    println!("{:?}", devices);
    'outer: loop {
        if let Some(event) = manager.get_event() {
            match event {
                RawEvent::MouseMoveEvent(_, x, y) => {
                    println!("MouseMoveEvent: {} {}", x, y);

                    // break;
                }
                _ => (),
            }
            println!("{:?}", event);
        } else {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
    println!("Finishing");
}
