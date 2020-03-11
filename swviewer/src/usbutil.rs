pub fn lsusb() -> rusb::Result<()> {
    for device_list in rusb::devices().iter() {
        info!("Device list:");
        for device in device_list.iter() {
            let desc = device.device_descriptor()?;
            info!(
                "- Device: bus={:?} vendor:product = {:04X}:{:04X}",
                device.bus_number(),
                desc.vendor_id(),
                desc.product_id()
            );
        }
    }

    Ok(())
}
