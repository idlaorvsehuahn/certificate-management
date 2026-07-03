/// Validates pagination query parameters.
/// Returns Ok(()) if parameters are valid, or an Err with a descriptive message.
pub fn validate_list_query(page: u32, page_size: u32, max_page_size: u32) -> Result<(), String> {
    if page == 0 {
        return Err("page must be greater than 0".to_string());
    }

    if page_size == 0 || page_size > max_page_size {
        return Err(format!(
            "page_size must be between 1 and {max_page_size}"
        ));
    }

    Ok(())
}

/// Calculates the total pages count based on total items and page size.
pub fn total_pages(total_items: i64, page_size: u32) -> u32 {
    if total_items == 0 {
        return 0;
    }

    ((total_items as f64) / f64::from(page_size)).ceil() as u32
}
