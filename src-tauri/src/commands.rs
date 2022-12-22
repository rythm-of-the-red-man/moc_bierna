use crate::measurement::ASMeasurement;


#[tauri::command]
pub fn convert (path_in:&str, path_out:&str) {
    let measurement = ASMeasurement::new(path_in);
    measurement.to_csv(path_out)
}