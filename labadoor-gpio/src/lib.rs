use gpio_cdev::{Chip, LineRequestFlags};

pub struct GPIOArgs {
    pub device: String,
    pub pin: u8,
    pub active_low: bool,
    pub active_time: u32,
}

pub fn gpio(args: GPIOArgs) {
    let mut chip = Chip::new(args.device).unwrap();
    let handle = chip
        .get_line(args.pin as u32)
        .unwrap()
        .request(LineRequestFlags::OUTPUT, 1, "labadoor-gpio")
        .unwrap();
    handle.set_value(!args.active_low as u8).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(args.active_time as u64));
    handle.set_value(args.active_low as u8).unwrap();
}
