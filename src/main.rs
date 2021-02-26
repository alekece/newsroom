// Copyright (c) 2021 Alexis Le Provost
// SPDX-License-Identifier: MIT

use newsroom::{NewsSource, Scraper};
use rayon::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
  pub source: Option<NewsSource>,
  #[structopt(short, long, default_value = "10")]
  pub max_page: usize,
}

fn main() {
  let opt = Opt::from_args();
  let sources = match opt.source {
    Some(source) => vec![source],
    None => NewsSource::all(),
  };
  let scraper = Scraper::new(opt.max_page);

  let sources = sources
    .par_iter()
    .map(|source| (source, scraper.scrap(*source)))
    .collect::<Vec<_>>();

  sources.into_iter().for_each(|(source, news)| {
    println!("{}", source);

    match news {
      Ok(news) => news
        .into_iter()
        .enumerate()
        .for_each(|(i, news)| println!("{}. {}", i + 1, news)),
      Err(e) => println!("Could not fetch news: {:#?}", e),
    };

    println!();
  });
}
