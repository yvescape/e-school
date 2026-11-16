const DEFAULT_PER_PAGE: u32 = 20;
const MAX_PER_PAGE: u32 = 100;

/// Demande de page, toujours ramenée dans des bornes sûres.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageRequest {
    page: u32,
    per_page: u32,
}

impl PageRequest {
    pub fn new(page: Option<u32>, per_page: Option<u32>) -> Self {
        Self {
            page: page.unwrap_or(1).max(1),
            per_page: per_page.unwrap_or(DEFAULT_PER_PAGE).clamp(1, MAX_PER_PAGE),
        }
    }

    pub const fn page(&self) -> u32 {
        self.page
    }

    pub const fn per_page(&self) -> u32 {
        self.per_page
    }

    pub fn offset(&self) -> u64 {
        u64::from(self.page - 1) * u64::from(self.per_page)
    }
}
