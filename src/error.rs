// Copyright (c) 2021 Alexis Le Provost
// SPDX-License-Identifier: MIT

use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error(transparent)]
  HttpError(#[from] reqwest::Error),
  #[error(transparent)]
  ParseError(#[from] io::Error),
  #[error(transparent)]
  RssError(#[from] rss::Error),
}
