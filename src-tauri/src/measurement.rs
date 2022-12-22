use std::{fs::File, io::Read};

use chrono::NaiveDateTime;

pub struct ASMeasurement {
    pub dates: Vec<NaiveDateTime>,
    pub first_measurement: Vec<f32>,
    pub fifth_measurement: Vec<f32>,
    pub eight_measurement: Vec<f32>,
}
impl ASMeasurement {
    pub fn format_date(date: &str) -> NaiveDateTime {
        //26.11.2022 00:00:00
        let formatted = NaiveDateTime::parse_from_str(date, "%d.%m.%Y %H:%M:%S");
        let result = match formatted {
            Ok(result) => result,
            Err(_) => {
                return NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S.%3f Z").unwrap();
            }
        };
        return result;
    }
    pub fn to_csv(&self, path: &str) {
        let mut wtr = csv::Writer::from_path(path).unwrap();
        wtr.write_record(&[
            "Data",
            "1.8/ Energia czynna pobrana",
            "5.8 / Energia indukcyjna pobrana",
            "8.8 / Energia pojemnościowa pobrana",
        ]).unwrap();
        let length = &self.dates.len();
        for i in 0..*length {
            wtr.write_record(&[
                &self.dates[i].to_string(),
                &self.first_measurement[i].to_string(),
                &self.fifth_measurement[i].to_string(),
                &self.eight_measurement[i].to_string(),
            ]).unwrap();
        }
    }
    pub fn new(path: &str) -> ASMeasurement {
        let mut file = File::open(path).unwrap();
        let mut buf = vec![];
        file.read_to_end(&mut buf).unwrap();
        let content = String::from_utf8_lossy(&buf);
        // let content = fs::read_to_string(path).unwrap();
        let mut replaced = content.replace("Kasowanie mocy / zaniki fazy (1-2 fazy)", "");
        replaced = replaced.replace(",", ".");
        // replaced = replaced.replace(";", "");
        let mut lines: Vec<&str> = replaced.split("\r\n").collect();
        lines.drain(0..3);
        let mut dates: Vec<NaiveDateTime> = vec![];
        let mut first_measurement: Vec<f32> = vec![];
        let mut fifth_measurement: Vec<f32> = vec![];
        let mut eight_measurement: Vec<f32> = vec![];
        for line in lines {
            let row: Vec<&str> = line.split(";  ").collect();
            if row.len() > 1 {
                dates.push(ASMeasurement::format_date(row[0]));
                first_measurement.push(row[1].parse::<f32>().unwrap());
                fifth_measurement.push(row[3].parse::<f32>().unwrap());
                eight_measurement.push(row[6].replace(";", "").parse::<f32>().unwrap());
            }
        }
        ASMeasurement {
            dates,
            first_measurement,
            fifth_measurement,
            eight_measurement,
        }
    }
}
