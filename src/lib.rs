// Copyright (c) 2021 Alexis Le Provost
// SPDX-License-Identifier: MIT

mod error;
pub mod sources;

pub use error::Error;
pub use sources::{NewsSource, Scraper};

pub type Result<T> = std::result::Result<T, Error>;
