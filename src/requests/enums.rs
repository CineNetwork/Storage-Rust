use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) enum VideoQuality {
    #[serde(rename = "360p")]
    P360,
    #[serde(rename = "480p")]
    P480,
    #[serde(rename = "720p")]
    P720,
    #[serde(rename = "1080p")]
    P1080,
}
impl VideoQuality {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::P360 => "360p",
            Self::P480 => "480p",
            Self::P720 => "720p",
            Self::P1080 => "1080p",
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnimeDirFileName {
    PosterLowQuality,
    PosterHighQuality,
    Banner,
}
impl AnimeDirFileName {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PosterLowQuality => "poster_low_quality",
            Self::PosterHighQuality => "poster_high_quality",
            Self::Banner => "banner",
        }
    }
}
#[derive(Deserialize,PartialEq,Eq)]
#[serde(rename_all = "snake_case")]
pub enum EpisodeDirFileName {
    Preview,
    MasterPlaylist,
}
impl EpisodeDirFileName {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Preview => "preview",
            Self::MasterPlaylist => "master-playlist",
        }
    }
}
#[derive(Deserialize,PartialEq,Eq)]
#[serde(rename_all = "snake_case")]
pub enum EpisodeDirImageFileName {
    Preview,
}
impl EpisodeDirImageFileName {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Preview => "preview",
        }
    }
}
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VideoQualityDirFileName {
    Playlist,
}
impl VideoQualityDirFileName {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Playlist => "playlist",
        }
    }
}
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AudioDubDirFileName {
    Playlist,
}
impl AudioDubDirFileName {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Playlist => "playlist",
        }
    }
}

#[derive(Deserialize,PartialEq,Eq)]
#[serde(rename_all = "snake_case")]
pub enum VttDirFileName {
    Thumbs,
    Thumbnails,
}
impl VttDirFileName {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Thumbs => "thumbs",
            Self::Thumbnails => "thumbnails",
        }
    }
}
#[derive(Deserialize,PartialEq,Eq)]
#[serde(rename_all = "snake_case")]
pub enum ImageExtension {
    #[serde(rename = "png")]
    PNG,
    #[serde(rename = "jpeg")]
    JPEG,
    #[serde(rename = "jpg")]
    JPG,
}
impl ImageExtension {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PNG => "png",
            Self::JPEG => "jpeg",
            Self::JPG => "jpg",
        }
    }
}
