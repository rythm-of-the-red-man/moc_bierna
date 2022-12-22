#[cfg(test)]
mod tests {
    use crate::measurement::{self, ASMeasurement};
    use std::env::current_dir;

    #[test]
    pub fn test_single_user_case() {
        let current_path = current_dir().unwrap();
        let path = current_path.to_str().unwrap().to_string() + "/src/tests/AS  3280797.LP";

        let measurement = ASMeasurement::new(&path);
        let control_len = measurement.dates.len();
        assert_eq!(measurement.first_measurement.len(), control_len);
        assert_eq!(measurement.fifth_measurement.len(), control_len);
        assert_eq!(measurement.eight_measurement.len(), control_len);
    }
}
