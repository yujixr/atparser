extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

use chrono::DateTime;
use rss::{ChannelBuilder, ImageBuilder, ItemBuilder};
use scraper::{Html, Selector};

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

fn item_builder(
    content: &scraper::element_ref::ElementRef<'_>,
    date_selector: &scraper::Selector,
    title_selector: &scraper::Selector,
    level_selector: &scraper::Selector,
    time_selector: &scraper::Selector,
) -> rss::Item {
    let date = content.select(&date_selector).last().unwrap();
    let title = content.select(&title_selector).last().unwrap();
    let level = content.select(&level_selector).last().unwrap();
    let time = content.select(&time_selector).last().unwrap();

    let link = title.value().attr("href").unwrap();
    let link = format!("https://atcoder.jp{}", link);

    let title = title.inner_html();
    let date = DateTime::parse_from_str(date.inner_html().as_str(), "%Y-%m-%d %H:%M:%S%z").unwrap();
    let level = match level.value().attr("class").unwrap() {
        "user-red" => "All (AGC Class)",
        "user-orange" => "~2799 (ARC Class)",
        "user-blue" => "~1999 (ABC Class)",
        _ => "Unrated",
    };
    let time = time.inner_html();

    ItemBuilder::default()
        .description(format!(
            "<p>Rated: {}</p><p>Start: {}</p><p>Time: {}</p>",
            &level,
            &date.format("%Y/%m/%d %H:%M").to_string(),
            &time
        ))
        .title(title)
        .link(link)
        .pub_date(date.to_rfc2822())
        .build()
        .unwrap()
}

#[wasm_bindgen]
pub fn parser(html: &str) -> String {
    let document = Html::parse_document(&html);
    let upcoming_selector =
        Selector::parse("div#contest-table-upcoming>div>div.table-responsive>table>tbody>tr")
            .unwrap();
    let recent_selector =
        Selector::parse("div#contest-table-recent>div>div.table-responsive>table>tbody>tr")
            .unwrap();
    let date_selector = Selector::parse("td>a>time").unwrap();
    let title_selector = Selector::parse("td:nth-child(2)>a").unwrap();
    let level_selector = Selector::parse("td:nth-child(2)>span").unwrap();
    let time_selector = Selector::parse("td:nth-child(3)").unwrap();

    let mut items: Vec<rss::Item> = vec![];

    for content in document.select(&upcoming_selector) {
        items.push(item_builder(
            &content,
            &date_selector,
            &title_selector,
            &level_selector,
            &time_selector,
        ));
    }

    for content in document.select(&recent_selector) {
        items.push(item_builder(
            &content,
            &date_selector,
            &title_selector,
            &level_selector,
            &time_selector,
        ));
    }

    let image = ImageBuilder::default()
        .url("https://img.atcoder.jp/assets/top/img/logo_bk.svg")
        .title("AtCoder")
        .link("https://atcoder.jp/home")
        .build()
        .unwrap();

    ChannelBuilder::default()
        .title("AtCoder Contests")
        .link("https://atcoder.jp/home")
        .description("AtCoder Contest Feed")
        .image(image)
        .items(items)
        .build()
        .unwrap()
        .to_string()
}
