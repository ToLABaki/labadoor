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
