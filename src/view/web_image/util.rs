use futures::StreamExt;
use gtk::gdk::Texture;
use gtk::gdk_pixbuf::PixbufLoader;
use gtk::glib;
use gtk::prelude::PixbufLoaderExt;
use reqwest::IntoUrl;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    PixBufError(#[from] glib::Error),
}

pub async fn load_image(url: impl IntoUrl) -> Result<Option<Texture>, Error> {
    let mut image_data = reqwest::get(url).await?.bytes_stream();
    let image = PixbufLoader::new();

    while let Some(chunk) = image_data.next().await {
        image.write(&chunk?)?;
    }
    image.close()?;
    let texture = image.pixbuf().map(|pixbuf| Texture::for_pixbuf(&pixbuf));
    Ok(texture)
}
