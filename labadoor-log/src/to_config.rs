use crate::cli::CSV;

pub trait ToConfig<T> {
    fn to_config(&self) -> T;
}

#[cfg(feature = "csv")]
impl ToConfig<labadoor_csv::CSVArgs> for CSV {
    fn to_config(&self) -> labadoor_csv::CSVArgs {
        labadoor_csv::CSVArgs {
            path: self.path.clone(),
        }
    }
}
