// Copyright (c) 2021 Alexis Le Provost
// SPDX-License-Identifier: MIT

use crate::{Error, Result};
use rss::Channel;
use scraper::{ElementRef, Html, Selector};
use std::{fmt, io, str::FromStr};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum NewsSource {
  HackerNews,
  ProductHunt,
  TechMeme,
  WSJ,
  GithubTrending,
}

impl FromStr for NewsSource {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    match s {
      "hackernews" => Ok(Self::HackerNews),
      "producthunt" => Ok(Self::ProductHunt),
      "techmeme" => Ok(Self::TechMeme),
      "wsj" => Ok(Self::WSJ),
      "github-trending" => Ok(Self::GithubTrending),
      _ => Err(Error::ParseError(io::Error::new(
        io::ErrorKind::InvalidInput,
        s,
      ))),
    }
  }
}

impl fmt::Display for NewsSource {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let output = match self {
      Self::HackerNews => "HACKER NEWS",
      Self::ProductHunt => "PRODUCT HUNT",
      Self::TechMeme => "TECHMEME",
      Self::WSJ => "WALL STREET JOURNAL",
      Self::GithubTrending => "GITHUB TRENDING",
    };

    write!(f, "{}", output)
  }
}

impl NewsSource {
  pub fn all() -> Vec<NewsSource> {
    vec![
      Self::HackerNews,
      Self::ProductHunt,
      Self::TechMeme,
      Self::WSJ,
      Self::GithubTrending,
    ]
  }
}

#[derive(PartialEq)]
pub struct NewsInfo {
  pub title: String,
  pub description: Option<String>,
}

impl fmt::Display for NewsInfo {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if let Some(description) = &self.description {
      write!(f, "{} - {}", self.title, description)
    } else {
      write!(f, "{}", self.title)
    }
  }
}

const HACKER_NEWS_URL: &str = "https://news.ycombinator.com/";
const PRODUCT_HUNT_URL: &str = "https://www.producthunt.com/";
const TECH_MEME_URL: &str = "https://www.techmeme.com/feed.xml";
const WSJ_URL: &str = "https://feeds.a.dj.com/rss/RSSWSJD.xml";
const GITHUB_TRENDING_URL: &str = "https://github.com/trending";

pub struct Scraper {
  max_page: usize,
}

impl Scraper {
  pub fn new(max_page: usize) -> Self {
    Self { max_page }
  }

  pub fn scrap(&self, source: NewsSource) -> Result<Vec<NewsInfo>> {
    let extract = |element: &ElementRef, selector: &Selector| {
      element
        .select(selector)
        .into_iter()
        .take(1)
        .map(|element| element.text().map(|s| s.trim()).collect::<String>())
        .collect::<String>()
    };

    Ok(match source {
      NewsSource::HackerNews => {
        let document =
          Html::parse_document(reqwest::blocking::get(HACKER_NEWS_URL)?.text()?.as_str());
        let selector = Selector::parse(r#"a[class="storylink"]"#).unwrap();

        document
          .select(&selector)
          .into_iter()
          .take(self.max_page)
          .map(|element| NewsInfo {
            title: element.text().collect(),
            description: None,
          })
          .collect::<Vec<_>>()
      }
      NewsSource::ProductHunt => {
        let document =
          Html::parse_document(reqwest::blocking::get(PRODUCT_HUNT_URL)?.text()?.as_str());
        let selector = Selector::parse(
          r#"div[class^="styles_container"]:nth-child(2) div[class^="styles_content"]"#,
        )
        .unwrap();
        let title_selector = Selector::parse("h3 a[data-test]").unwrap();
        let description_selector = Selector::parse("p a").unwrap();

        let mut news = document
          .select(&selector)
          .into_iter()
          .filter_map(|element| {
            match (
              extract(&element, &title_selector),
              extract(&element, &description_selector),
            ) {
              (title, description) if !title.is_empty() && !description.is_empty() => {
                Some(NewsInfo {
                  title,
                  description: Some(description),
                })
              }
              _ => None,
            }
          })
          .collect::<Vec<_>>();

        news.dedup();

        news.into_iter().take(self.max_page).collect()
      }
      NewsSource::TechMeme => {
        Channel::read_from(&reqwest::blocking::get(TECH_MEME_URL)?.bytes()?[..])?
          .into_items()
          .into_iter()
          .filter_map(|item| match item.title {
            Some(title) => Some(NewsInfo {
              title,
              description: None,
            }),
            None => None,
          })
          .take(self.max_page)
          .collect()
      }
      NewsSource::WSJ => Channel::read_from(&reqwest::blocking::get(WSJ_URL)?.bytes()?[..])?
        .into_items()
        .into_iter()
        .take(self.max_page)
        .map(|item| NewsInfo {
          title: item.title.unwrap(),
          description: item.description,
        })
        .collect(),
      NewsSource::GithubTrending => {
        let document = Html::parse_document(
          reqwest::blocking::get(GITHUB_TRENDING_URL)?
            .text()?
            .as_str(),
        );
        let selector = Selector::parse(r#"article[class="Box-row"]"#).unwrap();
        let title_selector = Selector::parse("h1").unwrap();
        let description_selector = Selector::parse("p").unwrap();

        document
          .select(&selector)
          .into_iter()
          .take(self.max_page)
          .map(|element| NewsInfo {
            title: extract(&element, &title_selector),
            description: Some(extract(&element, &description_selector)),
          })
          .collect()
      }
    })
  }
}
