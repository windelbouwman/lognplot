// Data IO. This could be moved to the lognplot crate?

use super::error_dialog::show_error;
use super::GuiStateHandle;
use gtk::prelude::*;
use lognplot::time::TimeStamp;
use lognplot::tsdb::{Observation, Sample, TsDbHandle};
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
    let filename = dialog.filename();
    if let gtk::ResponseType::Accept = res {
        if let Some(filename) = filename {
            info!("Saving data to filename: {:?}", filename);
            let res = { app_state.borrow().save(&filename) };
            if let Err(err) = res {
                let error_message = format!("Error saving data: {}", err);
                show_error(top_level, &error_message);
            } else {
                info!("Data saved success");
            }
        }
    }
}

pub fn load_data_from_hdf5(top_level: &gtk::Window, app_state: &GuiStateHandle) {
    let dialog = gtk::FileChooserDialog::with_buttons(
        Some("Import data from HDF5 file"),
        Some(top_level),
        gtk::FileChooserAction::Open,
        &[
            ("Cancel", gtk::ResponseType::Cancel),
            ("Open", gtk::ResponseType::Accept),
        ],
    );

    let res = dialog.run();
    let filename = dialog.filename();
    if let (gtk::ResponseType::Accept, Some(filename)) = (res, filename) {
        info!("Loading data from filename: {:?}", filename);
        let res = { app_state.borrow().load(&filename) };
        if let Err(err) = res {
            let error_message = format!("Error loading data from {:?}: {}", filename, err);
            show_error(top_level, &error_message);
        } else {
            info!("Data loaded!");
        }
    }
}

pub fn export_data(db: TsDbHandle, filename: &Path) -> hdf5::Result<()> {
    let file = hdf5::File::create(filename)?;
    export_db(db, &file)
}

/// Export all signals from the database into a HDF5 file.
fn export_db(db: TsDbHandle, file: &hdf5::File) -> hdf5::Result<()> {
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

            // Create hdf5 dataset for this signal:
            group
                .new_dataset_builder()
                .with_data(&signal)
                .create(signal_name.as_str())?;
        }
    }

    Ok(())
}

/// Import data from file into the database.
pub fn import_data(db: TsDbHandle, filename: &Path) -> hdf5::Result<()> {
    let file = hdf5::File::open(filename)?;
    import_data_inner(db, &file)
}

fn import_data_inner(db: TsDbHandle, file: &hdf5::File) -> hdf5::Result<()> {
    // TODO: determine internal storage schema
    let group = file.group("my_datorz")?;
    for name in group.member_names()? {
        debug!("Importing signal with name: {}", name);
        let dataset = group.dataset(&name)?;

        let data = dataset.read_2d::<f64>()?;

        let shape = data.shape();
        debug!("Signal data shape: {:?}", shape);

        if shape[1] == 2 {
            // println!("Let go!");
            let mut samples = vec![];
            for row in data.rows() {
                assert!(row.len() == 2);
                let timestamp = TimeStamp::new(row[0]);
                let value = Sample::new(row[1]);
                let observation = Observation::new(timestamp, value);
                samples.push(observation);
            }

            db.add_values(&name, samples);
        } else {
            warn!("Skipping signal due to shape: {:?}", shape);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{export_db, import_data_inner};
    use lognplot::time::TimeStamp;
    use lognplot::tsdb::{Observation, Sample, TsDb};

    #[test]
    fn export_test() -> hdf5::Result<()> {
        let mut db = TsDb::default();

        let trace_name = "foo";

        // Insert data:
        for x in 1..8 {
            let ts = TimeStamp::from_seconds(x);
            let sample = Sample::new(3.1415926 + x as f64);
            let observation = Observation::new(ts.clone(), sample);
            db.add_value(trace_name, observation);
        }

        let db_handle = db.into_handle();

        // Export data:
        let file = hdf5::File::create("export_test.h5")?;
        export_db(db_handle, &file)?;

        // Import the data back:
        let db2_handle = TsDb::default().into_handle();
        import_data_inner(db2_handle.clone(), &file)?;

        assert_eq!(vec![trace_name], db2_handle.get_signal_names());
        assert_eq!(7, db2_handle.quick_summary(trace_name).unwrap().count);
        Ok(())
    }
}
