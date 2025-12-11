/*

BINZIP format.

 */

use super::GuiStateHandle;
use gtk::prelude::*;
use lognplot::time::TimeStamp;
use lognplot::tsdb::observations::{Observation, Sample};
use lognplot::tsdb::TsDbHandle;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use zip::write::SimpleFileOptions;
use zip::ZipArchive;

pub fn load_data_from_binzip(top_level: &gtk::Window, app_state: &GuiStateHandle) {
    info!("Loading BINZIP file");
    let dialog = gtk::FileChooserDialog::new(
        Some("Import data from ZIP file"),
        Some(top_level),
        gtk::FileChooserAction::Open,
        &[
            ("Cancel", gtk::ResponseType::Cancel),
            ("Open", gtk::ResponseType::Accept),
        ],
    );

    dialog.set_modal(true);
    dialog.present();

    dialog.connect_response(clone!(
        #[strong]
        app_state,
        move |dialog, res| {
            let file = dialog.file();
            dialog.close();

            if let (gtk::ResponseType::Accept, Some(file)) = (res, file) {
                let filename = file.path().unwrap();
                info!("Loading data from filename: {:?}", filename);
                load_zip(&filename, &app_state.borrow().db);
            }
        }
    ));
}

/// Popup a dialog to save session for later usage.
pub fn save_data_to_binzip(top_level: &gtk::Window, app_state: &GuiStateHandle) {
    info!("Save data as BINZIP");
    let dialog = gtk::FileChooserDialog::new(
        Some("Save data as BINZIP"),
        Some(top_level),
        gtk::FileChooserAction::Save,
        &[
            ("Cancel", gtk::ResponseType::Cancel),
            ("Save", gtk::ResponseType::Accept),
        ],
    );
    dialog.set_modal(true);
    dialog.present();

    dialog.connect_response(clone!(
        #[strong]
        app_state,
        move |dialog, res| {
            let file = dialog.file();
            dialog.close();

            if let (gtk::ResponseType::Accept, Some(file)) = (res, file) {
                let filename = file.path().unwrap();
                info!("Saving data to filename: {:?}", filename);
                save_binzip(&filename, &app_state.borrow().db);
                info!("Data saved!");
            }
        }
    ));
}

fn load_zip(filename: &std::path::Path, db: &TsDbHandle) {
    info!("Opening ZIP file: {:?}", filename);
    let file = File::open(filename).expect("Failed to open ZIP file");
    let mut archive = ZipArchive::new(file).expect("Failed to read ZIP archive");

    let info_json = archive
        .by_name("info.json")
        .expect("info.json not found in ZIP");
    let info: Info = serde_json::from_reader(info_json).expect("Failed to parse info.json");

    println!("info.json contents: {:?}", info);

    for probe in info.probes {
        info!("Loading probe from file: {}", probe.filename);
        load_probe(&mut archive, &probe, &db);
    }
}

fn load_probe(archive: &mut ZipArchive<File>, probe: &Probe, db: &TsDbHandle) {
    let mut bin_file = archive
        .by_name(&probe.filename)
        .expect("bin file not found in ZIP");

    // Calculate the size of each row based on variable types
    let row_data_size: usize = probe
        .variables
        .iter()
        .map(|var| match var.datatype.as_str() {
            "uint8" => 1,
            "uint16" => 2,
            "uint32" => 4,
            "float" => 4,
            "double" => 8,
            _ => panic!("Unknown datatype: {}", var.datatype),
        })
        .sum();
    let row_size = row_data_size + 8;

    // Read the entire binary file into a buffer
    let mut buf = Vec::new();
    bin_file
        .read_to_end(&mut buf)
        .expect("Failed to read data.bin");

    // Deserialize row by row
    let mut rows: Vec<(f64, Vec<f64>)> = Vec::new();
    for chunk in buf.chunks(row_size) {
        let mut row = Vec::new();
        let mut offset = 0;
        let timestamp = load_value(&mut offset, &chunk, "double");
        for var in &probe.variables {
            let value = load_value(&mut offset, &chunk, &var.datatype);
            row.push(value);
        }
        rows.push((timestamp, row));
    }

    info!("Deserialized {} rows", rows.len());

    for row in rows.iter() {
        let (timestamp2, values) = row;
        for (var, &value) in probe.variables.iter().zip(values.iter()) {
            let timestamp = TimeStamp::new(*timestamp2);
            let sample = Sample::new(value);
            let observation = Observation::new(timestamp, sample);
            db.add_value(&var.name, observation);
        }
    }
}

fn load_value(offset: &mut usize, chunk: &[u8], datatype: &str) -> f64 {
    match datatype {
        "uint8" => {
            let v = chunk[*offset];
            *offset += 1;
            v as f64
        }
        "uint16" => {
            let v = u16::from_le_bytes([chunk[*offset], chunk[*offset + 1]]);
            *offset += 2;
            v as f64
        }
        "uint32" => {
            let v = u32::from_le_bytes([
                chunk[*offset],
                chunk[*offset + 1],
                chunk[*offset + 2],
                chunk[*offset + 3],
            ]);
            *offset += 4;
            v as f64
        }
        "float" => {
            let v = f32::from_le_bytes([
                chunk[*offset],
                chunk[*offset + 1],
                chunk[*offset + 2],
                chunk[*offset + 3],
            ]);
            *offset += 4;
            v as f64
        }
        "double" => {
            let v = f64::from_le_bytes([
                chunk[*offset],
                chunk[*offset + 1],
                chunk[*offset + 2],
                chunk[*offset + 3],
                chunk[*offset + 4],
                chunk[*offset + 5],
                chunk[*offset + 6],
                chunk[*offset + 7],
            ]);
            *offset += 8;
            v
        }
        other => panic!("Unknown datatype: {}", other),
    }
}

fn save_binzip(filename: &std::path::Path, db: &TsDbHandle) {
    // Implementation for saving data to ZIP file goes here

    let zip_file = File::create(filename).expect("Failed to create ZIP file");
    let mut archive = zip::ZipWriter::new(zip_file);

    let mut probes: Vec<Probe> = vec![];

    let signal_names = db.get_signal_names();
    for (index, signal_name) in signal_names.into_iter().enumerate() {
        if let Some(data) = db.get_raw_samples(&signal_name) {
            let mut variables: Vec<Variable> = vec![];

            debug!(
                "Saving signal {} with {} data points",
                signal_name,
                data.len()
            );
            // Create f64 data:
            let data: Vec<[f64; 2]> = data
                .iter()
                .map(|o| [o.timestamp.amount, o.value.value])
                .collect();

            // Start a new file in the ZIP archive for this signal
            let bin_filename = format!("data_{}.bin", index);
            let options =
                SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
            archive
                .start_file(&bin_filename, options)
                .expect("Failed to create data file in ZIP");

            // Write each [f64; 2] as binary (little-endian)
            for pair in &data {
                for &val in pair {
                    archive
                        .write_all(&val.to_le_bytes())
                        .expect("Failed to write f64 to ZIP");
                }
            }

            variables.push(Variable {
                name: signal_name.clone(),
                datatype: "double".to_string(),
            });

            probes.push(Probe {
                filename: bin_filename,
                variables,
            });
        }
    }

    // Write info.json
    let info = Info { probes };
    let info_filename = "info.json";
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    archive
        .start_file(info_filename, options)
        .expect("Failed to create info.json in ZIP");
    serde_json::to_writer(archive, &info).expect("Failed to write info.json");
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    pub probes: Vec<Probe>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Probe {
    #[serde(rename = "file")]
    pub filename: String,

    pub variables: Vec<Variable>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Variable {
    pub name: String,

    #[serde(rename = "type")]
    pub datatype: String,
}
