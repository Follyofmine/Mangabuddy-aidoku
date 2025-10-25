#![no_std]
use aidoku::{Source, prelude::*};
use madtheme::{Impl, MadTheme, Params};

const BASE_URL: &str = "https://mangabuddy.com";

struct MangaBuddy;

impl Impl for MangaBuddy {
	fn new() -> Self {
		Self
	}

	fn params(&self) -> Params {
		Params {
			base_url: BASE_URL.into(),
			..Default::default()
		}
	}
}

register_source!(MadTheme<MangaBuddy>, ImageRequestProvider, DeepLinkHandler);
