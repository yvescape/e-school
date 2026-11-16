use super::PageRequest;
use serde::Serialize;

/// Page de résultats renvoyée au client.
#[derive(Debug, Clone, Serialize)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub per_page: u32,
}

impl<T> Page<T> {
    pub fn new(items: Vec<T>, total: u64, req: PageRequest) -> Self {
        let (page, per_page) = (req.page(), req.per_page());
        Self {
            items,
            total,
            page,
            per_page,
        }
    }

    pub fn total_pages(&self) -> u64 {
        self.total.div_ceil(u64::from(self.per_page))
    }

    pub fn map<U>(self, f: impl FnMut(T) -> U) -> Page<U> {
        let items = self.items.into_iter().map(f).collect();
        let (total, page, per_page) = (self.total, self.page, self.per_page);
        Page {
            items,
            total,
            page,
            per_page,
        }
    }
}
