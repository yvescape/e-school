use kernel::{Page, PageRequest};

#[test]
fn defaults_to_first_page_of_twenty() {
    let req = PageRequest::new(None, None);
    assert_eq!((req.page(), req.per_page()), (1, 20));
    assert_eq!(req.offset(), 0);
}

#[test]
fn clamps_out_of_range_values() {
    let req = PageRequest::new(Some(0), Some(10_000));
    assert_eq!((req.page(), req.per_page()), (1, 100));
    assert_eq!(PageRequest::new(None, Some(0)).per_page(), 1);
}

#[test]
fn offset_skips_previous_pages() {
    assert_eq!(PageRequest::new(Some(3), Some(25)).offset(), 50);
}

#[test]
fn page_computes_total_pages() {
    let req = PageRequest::new(Some(1), Some(20));
    assert_eq!(Page::new(vec![1, 2], 41, req).total_pages(), 3);
    assert_eq!(Page::<u8>::new(vec![], 0, req).total_pages(), 0);
}

#[test]
fn page_map_keeps_metadata() {
    let req = PageRequest::new(Some(2), Some(2));
    let page = Page::new(vec![1, 2], 5, req).map(|n| n * 10);
    assert_eq!(page.items, vec![10, 20]);
    assert_eq!((page.total, page.page, page.per_page), (5, 2, 2));
}
