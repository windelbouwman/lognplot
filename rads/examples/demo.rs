/// Demo usage of the ADS client.
///
use rads::{connect, AdsError};

fn main() -> Result<(), AdsError> {
    println!("Demo usage!");

    let dest = "127.0.0.1:1333".parse().unwrap();

    let mut ads_client = connect(dest)?;
    let info = ads_client.ads_read_device_info()?;

    println!("Device info: {:?}", info);

    Ok(())
}
