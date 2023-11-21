use crate::cli::*;

pub trait ToConfig<T> {
    fn to_config(&self) -> T;
}

#[cfg(feature = "telegram")]
impl ToConfig<labadoor_telegram::TelegramArgs> for Telegram {
    fn to_config(&self) -> labadoor_telegram::TelegramArgs {
        labadoor_telegram::TelegramArgs {
            token: self.token.clone().unwrap(),
        }
    }
}

#[cfg(feature = "matrix")]
impl ToConfig<labadoor_matrix::MatrixArgs> for Matrix {
    fn to_config(&self) -> labadoor_matrix::MatrixArgs {
        labadoor_matrix::MatrixArgs {
            username: self.username.clone().unwrap(),
            password: self.password.clone().unwrap(),
            device_id: self.device_id.clone(),
        }
    }
}
