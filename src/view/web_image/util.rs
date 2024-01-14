use std::path::PathBuf;
use directories::ProjectDirs;
use futures::StreamExt;
use gtk::gdk::Texture;
use gtk::gdk_pixbuf::PixbufLoader;
use gtk::glib;
use gtk::prelude::PixbufLoaderExt;
use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use lazy_static::lazy_static;
use reqwest::{Client, IntoUrl};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use thiserror::Error;

lazy_static!(
    static ref CACHE_FOLDER: PathBuf = ProjectDirs::from("at.ac", "tgm", "spelling_trainer").expect("Failed to get project dirs").cache_dir().to_owned();
    static ref CLIENT: ClientWithMiddleware = ClientBuilder::new(Client::new())
        .with(Cache(HttpCache {
            mode: CacheMode::Default,
            manager: CACacheManager {
                path: CACHE_FOLDER.clone(),
            },
            options: HttpCacheOptions::default(),
        }))
        .build();
);

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    PixBufError(#[from] glib::Error),
    #[error(transparent)]
    ReqwestMiddlewareError(#[from] reqwest_middleware::Error),
}

pub async fn load_image(url: impl IntoUrl) -> Result<Option<Texture>, Error> {
    let mut image_data = CLIENT.get(url).send().await?.bytes_stream();
    let image = PixbufLoader::new();

    while let Some(chunk) = image_data.next().await {
        image.write(&chunk?)?;
    }
    image.close()?;
    let texture = image.pixbuf().map(|pixbuf| Texture::for_pixbuf(&pixbuf));
    Ok(texture)
}
