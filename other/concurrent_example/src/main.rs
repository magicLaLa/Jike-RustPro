// https://medium.com/pragmatic-programmers/fearless-concurrency-with-rust-part-3-asynchronous-concurrency-e23bad856087
use concurrent_example::is_prime;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rocket::{response::content::RawHtml, serde::Deserialize, tokio::fs};
use std::time::Instant;
use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
};
#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

#[get("/")]
async fn index() -> RawHtml<String> {
    use rocket::tokio::join;
    let (index_html, menu, news) = join!(
        fs::read_to_string("other/concurrent_example/src/index.html"),
        build_menu(),
        build_news(),
    );
    let footer = build_footer();

    RawHtml(
        index_html.unwrap()
            .replace("!!MENU!!", &menu)
            .replace("!!NEWS!!", &news)
            .replace("!!FOOTER!!", &footer),
    )
}

async fn build_menu() -> String {
    fs::read_to_string("other/concurrent_example/src/menu.html").await.unwrap()
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct NewsItem {
    title: String,
    summary: String,
}

async fn build_news() -> String {
    let all_news = fs::read_to_string("other/concurrent_example/src/news.json").await.unwrap();
    let news_feed: Vec<NewsItem> = rocket::serde::json::from_str(&all_news).unwrap();
    news_feed
        .iter()
        .map(|news| {
            format!(
                "<div class='news'><h1>{}</h1><p>{}</p></div>",
                news.title, news.summary
            )
        })
        .reduce(|cur: String, nxt: String| cur + &nxt)
        .unwrap()
}

fn build_footer() -> String {
    "<p id='footer'>&copy; Copyright My Cool Company</p>".to_string()
}

fn main2_2() {
    let now = Instant::now();
    let primes_under_2_million: Vec<u32> = (2..2_000_000)
        .into_par_iter()
        .filter(|n| is_prime(*n))
        .collect();
    let elapsed = now.elapsed().as_secs_f32();
    println!(
        "Found {} primes in {:1} seconds",
        primes_under_2_million.len(),
        elapsed
    );
}

fn main2() {
    let now = Instant::now();
    let candidates: Vec<u32> = (2..2_000_000).collect();

    let mut threads: Vec<JoinHandle<Vec<u32>>> = candidates
        .chunks(2_000_000 / 12)
        .into_iter()
        .map(|chunk| {
            let my_chunk: Vec<u32> = chunk.to_owned();
            thread::spawn(move || {
                my_chunk
                    .iter()
                    .filter(|n| is_prime(**n))
                    .map(|n| *n)
                    .collect()
            })
        })
        .collect();

    let primes_under_2_million: Vec<u32> =
        threads.drain(0..).flat_map(|t| t.join().unwrap()).collect();
    let elapsed = now.elapsed().as_secs_f32();
    println!(
        "Found {} primes in {:1} seconds",
        primes_under_2_million.len(),
        elapsed
    );
}

fn main1() {
    const MAX: u32 = 200_000;
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_1 = counter.clone();
    let counter_2 = counter.clone();

    let t1 = std::thread::spawn(move || {
        counter_1.fetch_add(
            (2..MAX / 2).filter(|i| is_prime(*i)).count(),
            Ordering::SeqCst,
        )
    });
    let t2 = std::thread::spawn(move || {
        counter_2.fetch_add(
            (MAX / 2..MAX).filter(|i| is_prime(*i)).count(),
            Ordering::SeqCst,
        )
    });
    t1.join().unwrap();
    t2.join().unwrap();
    println!(
        "Found {} prime numbers in the range 2..{MAX}",
        counter.load(Ordering::SeqCst)
    );
}
