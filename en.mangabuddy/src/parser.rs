use aidoku::{Page, PageContent, Result, imports::{html::Html, net::Request}, alloc::vec::Vec};

/// Attempt to fetch and parse the page list for a chapter.
/// This mirrors the logic used in the vendored templates so the crate has
/// a local copy in case the packaging step referenced it.
pub fn get_page_list_from_url(url: &str, _referer: &str) -> Result<Vec<Page>> {
	let response = Request::get(url)?.string()?;

	// Helper that parses img elements from an HTML document
	fn parse_pages_from_html(html: &Html) -> Vec<Page> {
		// Note: this selector matches common mad-theme chapter image containers
		html.select("#chapter-images img, .chapter-image[data-src]")
			.map(|els| {
				els.filter_map(|el| {
					Some(Page {
						content: PageContent::url(el.attr("data-src")?),
						..Default::default()
					})
				})
				.collect()
			})
			.unwrap_or_default()
	}

	if response.contains("var chapImages = '") {
		Ok(response
			.split_once("var chapImages = '")
			.ok_or(error!("String not found: `var chapImages = '`"))?
			.1
			.split_once("';")
			.ok_or_else(|| error!("String not found: `';`"))?
			.0
			.split(',')
			.map(|s| s.to_string())
			.map(|url| Page {
				content: PageContent::url(url),
				..Default::default()
			})
			.collect())
	} else {
		let html = Html::parse_with_url(&response, url)?;
		let pages = parse_pages_from_html(&html);
		if pages.is_empty() {
			// no images found — return an error so callers can fall back
			bail!("No content found")
		} else {
			Ok(pages)
		}
	}
}
404: Not Found