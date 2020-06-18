use super::api::DictionaryHandle;
use crate::Result;
use std::sync::Arc;

/// A pre-compiled dictionary for the efficient compression
///
/// **Cited from lz4frame.h:**
///
/// A Dictionary is useful for the compression of small messages (KB range).
/// It dramatically improves compression efficiency.
///
/// LZ4 can ingest any input as dictionary, though only the last 64 KB are
/// useful. Best results are generally achieved by using Zstandard's Dictionary
/// Builder to generate a high-quality dictionary from a set of samples.
#[derive(Clone)]
pub struct Dictionary(Arc<DictionaryHandle>);

impl Dictionary {
    pub fn new(data: &[u8]) -> Result<Self> {
        DictionaryHandle::new(data).map(|dict| Self(Arc::new(dict)))
    }

    pub(crate) fn handle(&self) -> &DictionaryHandle {
        &self.0
    }
}