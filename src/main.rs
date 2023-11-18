use hidapi::DeviceInfo;


fn main() {
    let api = hidapi::HidApi::new().unwrap();

    let gamepad_devices = find_gamepad_devices(&api);

    if gamepad_devices.is_empty() {
        println!("There was no gamepads found");
        return;
    }

    println!("Gamepads found: {:?}", gamepad_devices);

    let first_gamepad = gamepad_devices[0];

    // Connect to device using its VID and PID (Ruyi controller)
    let vid = first_gamepad.vendor_id();
    let pid = first_gamepad.product_id();
    let device = api.open(vid, pid).unwrap();
    let product = device.get_product_string().unwrap();
    println!("Opened device {}", product.unwrap_or("".to_string()));

    println!("Reading gamepad input...");
    let mut old_buf = [0u8; 8];
    loop {
        // Read data from device
        let mut buf = [0u8; 8];
        match device.read(&mut buf[..]) {
            Ok(_) => {
                if buf != old_buf {
                    old_buf = buf;
                    // println!("Read: {:?}", &buf[..]);
                    if is_bit_activated(buf[5], 4) {
                        println!("Pressed button 1");
                    }
                    if is_bit_activated(buf[5], 5) {
                        println!("Pressed button 2");
                    }
                    if is_bit_activated(buf[5], 6) {
                        println!("Pressed button 3");
                    }
                    if is_bit_activated(buf[5], 7) {
                        println!("Pressed button 4");
                    }
                    if is_bit_activated(buf[6], 0) {
                        println!("Pressed button 5");
                    }
                    if is_bit_activated(buf[6], 1) {
                        println!("Pressed button 6");
                    }
                    if is_bit_activated(buf[6], 2) {
                        println!("Pressed button 7");
                    }
                    if is_bit_activated(buf[6], 3) {
                        println!("Pressed button 8");
                    }
                    if is_bit_activated(buf[6], 4) {
                        println!("Pressed button 9");
                    }
                    if is_bit_activated(buf[6], 5) {
                        println!("Pressed button 10");
                    }
                    if is_bit_activated(buf[3], 7) {
                        println!("Pressed right arrow");
                    }
                    if is_bit_activated(buf[4], 7) {
                        println!("Pressed down arrow");
                    }
                    if !is_bit_activated(buf[3], 0) {
                        println!("Pressed left arrow");
                    }
                    if !is_bit_activated(buf[4], 0) {
                        println!("Pressed up arrow");
                    }
                }
            }
            Err(e) => println!("Error: {}", e.to_string()),
        };
    }
}

fn find_gamepad_devices(api: &hidapi::HidApi) -> Vec<&DeviceInfo> {
    let mut gamepads_devices = Vec::new();

    // Print out information about all connected devices
    for device in api.device_list() {
        let product_string_val = match device.product_string() {
            Some(ps) => ps,
            None => continue,
        };
        if product_string_val.contains("USB") && product_string_val.contains("Gamepad") {
            gamepads_devices.push(device);
        }
    }

    gamepads_devices
}

fn is_bit_activated(the_byte: u8, bit_number: u32) -> bool {
    (the_byte >> bit_number) & 1 == 1
}
