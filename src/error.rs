use cpal::{BuildStreamError, PlayStreamError, SupportedStreamConfigsError};

#[derive(Debug)]
pub enum Error {
    NoOutputDevice,
    StreamConfig(SupportedStreamConfigsError),
    NoConfig,
    BuildStream(BuildStreamError),
    PlayStream(PlayStreamError),
}

impl From<SupportedStreamConfigsError> for Error {
    fn from(e: SupportedStreamConfigsError) -> Self {
        Error::StreamConfig(e)
    }
}

impl From<BuildStreamError> for Error {
    fn from(e: BuildStreamError) -> Self {
        Error::BuildStream(e)
    }
}

impl From<PlayStreamError> for Error {
    fn from(e: PlayStreamError) -> Self {
        Error::PlayStream(e)
    }
}
