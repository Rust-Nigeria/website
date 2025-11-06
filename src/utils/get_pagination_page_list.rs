/// Represents a page number or an ellipsis in pagination
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageItem {
    Page(usize),
    Ellipsis,
}

#[derive(Debug, Clone)]
pub struct PaginationUiData {
    total_pages: usize,
    current_page: usize,
    /// Number of pages to show on each side of current page
    siblings: usize,
    /// Number of pages to always show at start and end
    boundaries: usize,
}

impl PaginationUiData {
    pub fn new(total_pages: usize, current_page: usize) -> Self {
        Self {
            total_pages,
            current_page,
            siblings: 1,
            boundaries: 1,
        }
    }

    pub fn with_siblings(mut self, siblings: usize) -> Self {
        self.siblings = siblings;
        self
    }

    pub fn with_boundaries(mut self, boundaries: usize) -> Self {
        self.boundaries = boundaries;
        self
    }

    /// Generate the pagination items
    pub fn items(&self) -> Vec<PageItem> {
        if self.total_pages == 0 || self.current_page == 0 || self.current_page > self.total_pages {
            return vec![];
        }

        // Calculate the range of pages to show around current page
        let start_range = self.current_page.saturating_sub(self.siblings).max(1);
        let end_range = (self.current_page + self.siblings).min(self.total_pages);

        // Calculate boundary ranges
        let left_boundary_end = self.boundaries;
        let right_boundary_start = self.total_pages.saturating_sub(self.boundaries - 1);

        let mut items = Vec::new();

        // Add pages from 1 up to where we need them
        for page in 1..=self.total_pages {
            let in_left_boundary = page <= left_boundary_end;
            let in_main_range = page >= start_range && page <= end_range;
            let in_right_boundary = page >= right_boundary_start;

            if in_left_boundary || in_main_range || in_right_boundary {
                // Add ellipsis before this page if there's a gap
                if let Some(&PageItem::Page(last_page)) = items.last() {
                    if page > last_page + 1 {
                        items.push(PageItem::Ellipsis);
                    }
                }
                items.push(PageItem::Page(page));
            }
        }

        items
    }

    /// Get just the page numbers (useful for simple cases)
    pub fn pages(&self) -> Vec<usize> {
        self.items()
            .into_iter()
            .filter_map(|item| match item {
                PageItem::Page(p) => Some(p),
                PageItem::Ellipsis => None,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_pagination() {
        let pagination = PaginationUiData::new(10, 5);
        let items = pagination.items();

        assert_eq!(
            items,
            vec![
                PageItem::Page(1),
                PageItem::Ellipsis,
                PageItem::Page(4),
                PageItem::Page(5),
                PageItem::Page(6),
                PageItem::Ellipsis,
                PageItem::Page(10),
            ]
        );
    }

    #[test]
    fn test_at_start() {
        let pagination = PaginationUiData::new(10, 1);
        let items = pagination.items();

        assert_eq!(
            items,
            vec![
                PageItem::Page(1),
                PageItem::Page(2),
                PageItem::Ellipsis,
                PageItem::Page(10),
            ]
        );
    }

    #[test]
    fn test_at_end() {
        let pagination = PaginationUiData::new(10, 10);
        let items = pagination.items();

        assert_eq!(
            items,
            vec![
                PageItem::Page(1),
                PageItem::Ellipsis,
                PageItem::Page(9),
                PageItem::Page(10),
            ]
        );
    }

    #[test]
    fn test_near_start() {
        let pagination = PaginationUiData::new(10, 2);
        let items = pagination.items();

        assert_eq!(
            items,
            vec![
                PageItem::Page(1),
                PageItem::Page(2),
                PageItem::Page(3),
                PageItem::Ellipsis,
                PageItem::Page(10),
            ]
        );
    }

    #[test]
    fn test_small_total() {
        let pagination = PaginationUiData::new(5, 3);
        let items = pagination.items();

        // Should show all pages when total is small
        assert_eq!(
            items,
            vec![
                PageItem::Page(1),
                PageItem::Page(2),
                PageItem::Page(3),
                PageItem::Page(4),
                PageItem::Page(5),
            ]
        );
    }

    #[test]
    fn test_custom_siblings() {
        let pagination = PaginationUiData::new(20, 10).with_siblings(2);
        let items = pagination.items();

        assert_eq!(
            items,
            vec![
                PageItem::Page(1),
                PageItem::Ellipsis,
                PageItem::Page(8),
                PageItem::Page(9),
                PageItem::Page(10),
                PageItem::Page(11),
                PageItem::Page(12),
                PageItem::Ellipsis,
                PageItem::Page(20),
            ]
        );
    }

    #[test]
    fn test_custom_boundaries() {
        let pagination = PaginationUiData::new(20, 10).with_boundaries(2);
        let items = pagination.items();

        assert_eq!(
            items,
            vec![
                PageItem::Page(1),
                PageItem::Page(2),
                PageItem::Ellipsis,
                PageItem::Page(9),
                PageItem::Page(10),
                PageItem::Page(11),
                PageItem::Ellipsis,
                PageItem::Page(19),
                PageItem::Page(20),
            ]
        );
    }

    #[test]
    fn test_invalid_inputs() {
        assert_eq!(PaginationUiData::new(0, 1).items(), vec![]);
        assert_eq!(PaginationUiData::new(10, 0).items(), vec![]);
        assert_eq!(PaginationUiData::new(10, 11).items(), vec![]);
    }

    #[test]
    fn test_pages_method() {
        let pagination = PaginationUiData::new(10, 5);
        let pages = pagination.pages();

        assert_eq!(pages, vec![1, 4, 5, 6, 10]);
    }
}
