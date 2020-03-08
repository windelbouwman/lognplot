// Data IO. This could be moved to the lognplot crate?

use super::error_dialog::show_error;
use super::GuiStateHandle;
use gtk::prelude::*;
use lognplot::tsdb::TsDbHandle;
use std::path::Path;

/// Popup a dialog and export data as HDF5 format.
pub fn save_data_as_hdf5(top_level: &gtk::Window, app_state: &GuiStateHandle) {
    let dialog = gtk::FileChooserDialog::with_buttons(
        Some("Export data as HDF5"),
        Some(top_level),
        gtk::FileChooserAction::Save,
        &[
            ("Cancel", gtk::ResponseType::Cancel),
            ("Save", gtk::ResponseType::Accept),
        ],
    );
    let res = dialog.run();
    let filename = dialog.get_filename();
    dialog.destroy();
    if let gtk::ResponseType::Accept = res {
        if let Some(filename) = filename {
            info!("Saving data to filename: {:?}", filename);
            let res = { app_state.borrow().save(&filename) };
            if let Err(err) = res {
                let error_message = format!("Error saving data: {}", err);
                error!("{}", error_message);
                show_error(top_level, &error_message);
            } else {
                info!("Data saved success");
            }
        }
    }
}

pub fn export_data(db: TsDbHandle, filename: &Path) -> hdf5::Result<()> {
    let file = hdf5::File::create(filename)?;
    export_db(db, file)
}

fn export_db(db: TsDbHandle, file: hdf5::File) -> hdf5::Result<()> {
    let group = file.create_group("my_datorz")?;

    let signal_names = db.get_signal_names();
    for signal_name in signal_names {
        // db
        if let Some(data) = db.get_raw_samples(&signal_name) {
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

            // Construct ndarray:
            let signal = ndarray::arr2(&data);

            // Create hdf5 signal, and write it:
            let sig1 = group
                .new_dataset::<f64>()
                .create(&signal_name, (data.len(), 2))?;
            sig1.write(&signal)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::export_db;
    use lognplot::time::TimeStamp;
    use lognplot::tsdb::{Observation, Sample, TsDb};

    #[test]
    fn export_test() -> hdf5::Result<()> {
        let mut db = TsDb::default();

        let trace_name = "foo";

        // Create a trace:
        db.new_trace(trace_name);

        // Insert data:
        for x in 1..8 {
            let ts = TimeStamp::from_seconds(x);
            let sample = Sample::new(3.1415926 + x as f64);
            let observation = Observation::new(ts.clone(), sample);
            db.add_value(trace_name, observation);
        }

        let db_handle = db.into_handle();

        let file = hdf5::File::create("export_test.h5")?;
        export_db(db_handle, file)
    }
}
