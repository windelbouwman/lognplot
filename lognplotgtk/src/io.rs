// Data IO. This could be moved to the lognplot crate?

use lognplot::tsdb::TsDbHandle;

pub fn export_data(db: TsDbHandle) -> hdf5::Result<()> {
    let file = hdf5::File::open("datorz.h5", "w")?;
    let group = file.create_group("my_datorz")?;

    let signal_names = db.get_signal_names();
    for signal_name in signal_names {
        let sig1 = group.new_dataset::<f64>().create(&signal_name, 2)?;
        sig1.write(&[3.14, 2.7])?;
    }
    
    Ok(())
}
