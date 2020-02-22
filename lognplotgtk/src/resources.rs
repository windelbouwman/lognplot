//! Hold various resources used in the application.
//!
//! See for background information: https://blog.samwhited.com/2019/02/gtk3-patterns-in-rust-structure/

use gdk_pixbuf::PixbufLoaderExt;

const IMG_ICON: &[u8] = include_bytes!("../../logo/icon.png");
const IMG_LOGO: &[u8] = include_bytes!("../../logo/logo.png");

pub fn load_icon() -> Result<Option<gdk_pixbuf::Pixbuf>, glib::Error> {
    let loader = gdk_pixbuf::PixbufLoader::new();
    loader.write(IMG_ICON)?;
    loader.close()?;
    Ok(loader.get_pixbuf())
}

pub fn load_logo() -> Result<Option<gdk_pixbuf::Pixbuf>, glib::Error> {
    let loader = gdk_pixbuf::PixbufLoader::new();
    loader.write(IMG_LOGO)?;
    loader.close()?;
    Ok(loader.get_pixbuf())
}
