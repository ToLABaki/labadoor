pub trait ToConfig<T> {
    fn to_config(&self) -> T;
}

#[cfg(feature = "telegram")]
impl ToConfig<labadoor_telegram::TelegramArgs> for crate::cli::Telegram {
    fn to_config(&self) -> labadoor_telegram::TelegramArgs {
        labadoor_telegram::TelegramArgs {
            trigger: self.trigger.clone().unwrap(),
            token: self.token.clone().unwrap(),
        }
    }
}

#[cfg(feature = "matrix")]
impl ToConfig<labadoor_matrix::MatrixArgs> for crate::cli::Matrix {
    fn to_config(&self) -> labadoor_matrix::MatrixArgs {
        labadoor_matrix::MatrixArgs {
            trigger: self.trigger.clone().unwrap(),
            username: self.username.clone().unwrap(),
            password: self.password.clone().unwrap(),
            device_id: self.device_id.clone(),
        }
    }
}

#[cfg(feature = "gpio")]
impl ToConfig<labadoor_gpio::GPIOArgs> for crate::cli::GPIO {
    fn to_config(&self) -> labadoor_gpio::GPIOArgs {
        labadoor_gpio::GPIOArgs {
            device: self.device.clone().unwrap(),
            pin: self.pin.unwrap(),
            active_low: self.active_low.unwrap(),
            active_time: self.active_time.unwrap(),
        }
    }
}

#[cfg(feature = "open")]
impl ToConfig<labadoor_open::OpenArgs> for crate::cli::Open {
    fn to_config(&self) -> labadoor_open::OpenArgs {
        labadoor_open::OpenArgs {
            auth: self.auth.clone(),
            hardware: self.hardware.clone(),
            method: self.method.clone(),
            identifier: self.identifier.clone(),
            resource_shortcut: self.resource_shortcut,
        }
    }
}
