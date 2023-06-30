use gpio_cdev::{Chip, LineRequestFlags};

pub fn gpio(device: String, pin: u8, active_low: bool, active_time: u32) {
    let mut chip = Chip::new(device).unwrap();
    let handle = chip
        .get_line(pin as u32)
        .unwrap()
        .request(LineRequestFlags::OUTPUT, 1, "labadoor-gpio")
        .unwrap();
    handle.set_value(!active_low as u8).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(active_time as u64));
    handle.set_value(active_low as u8).unwrap();
}
