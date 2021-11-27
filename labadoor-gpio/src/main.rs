use gpio_cdev::{Chip, LineRequestFlags};

fn main() {
    let mut chip = Chip::new("/dev/gpiochip0").unwrap();
    let handle = chip
        .get_line(6)
        .unwrap()
        .request(LineRequestFlags::OUTPUT, 1, "labadoor-gpio")
        .unwrap();
    handle.set_value(1).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(2000));
    handle.set_value(0).unwrap();
}
