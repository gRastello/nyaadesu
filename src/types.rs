/// Struct that represents a torrent and contains some of its basic information.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Torrent {
    /// The category of the torrent.
    pub category: Category,
    /// The name of the torrent.
    pub name: String,
    /// Links for the torrent (magnet and .torrent) as a pair. If you want to
    /// get a url to the .torrent file or the magnet please use the
    /// [`torrent_file`] or the [`magnet_link`] methods.
    ///
    /// [`torrent_file`]: #method.torrent_file
    /// [`magnet_link`]: #method.magnet_link
    pub links: (Option<String>, Option<String>),
    /// Total size of the torrent's files in bytes.
    pub size: u64,
    /// Date of pubblication of the torrent as unix timestamp.
    pub date: u64,
    /// Number of seeders.
    pub seeders: u32,
    /// Number of leechers.
    pub leechers: u32,
    /// Number of completed downloads.
    pub completed_downloads: u32,
}

impl Torrent {
    /// Extract `.torrent`'s file url as `String`.
    pub fn torrent_file(&self) -> Option<String> {
        let mut result = String::from("https://nyaa.si");
        let (first, second) = &self.links;

        if let Some(torrent_file) = first {
            if torrent_file.starts_with("/download") {
                result.push_str(torrent_file);
                return Some(result.clone());
            }
        }

        if let Some(torrent_file) = second {
            if torrent_file.starts_with("/download") {
                result.push_str(torrent_file);
                return Some(result.clone());
            }
        }

        None
    }

    /// Extract magnet link as `String`.
    pub fn magnet_link(&self) -> Option<String> {
        let (first, second) = &self.links;

        if let Some(magnet) = first {
            if magnet.starts_with("magnet") {
                return Some(magnet.clone());
            }
        }

        if let Some(magnet) = second {
            if magnet.starts_with("magnet") {
                return Some(magnet.clone());
            }
        }

        None
    }
}

/// Enum that encodes a torrent's category.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Category {
    Anime(Anime),
    Audio(Audio),
    Literature(Literature),
    LiveAction(LiveAction),
    Pictures(Pictures),
    Software(Software),
}

/// Enum that encodes variants of anime torrents.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Anime {
    AnimeMusicVideo,
    EnglishTranslated,
    NonEnglishTranslated,
    Raw,
}

/// Enum that encodes variants of audio torrents.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Audio {
    Lossless,
    Lossy,
}

/// Enum that encodes variants of literature torrents.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Literature {
    EnglishTranslated,
    NonEnglishTranslated,
    Raw,
}

/// Enum that encodes variants of live action torrents.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LiveAction {
    EnglishTranslated,
    IdolPromotionalVideo,
    NonEnglishTranslated,
    Raw,
}

/// Enum that encodes variants of pictures torrents.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Pictures {
    Graphics,
    Photos,
}

/// Enum that encodes variants of software torrents.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Software {
    Applications,
    Games,
}

/// Emun that encodes possible errors
#[derive(Debug)]
pub enum Error {
    /// Error with the request e.g. the server do not respond or you don't have
    /// access to it.
    Request,
    /// Scraping error. Since all the data is obtained via scraping of html
    /// pages if somehting in how Nyaa.si's pages are generated is changed
    /// searching for torrents may fail. This should hopefully never happen but
    /// I highly suggest you to always check for scaping errors and, if you ever
    /// record one, to report it as an issue on [GitHub] so that the library can
    /// be updated.
    ///
    /// [GitHub]: https://github.com/grastello/nyaadesu
    Scraping,
}
