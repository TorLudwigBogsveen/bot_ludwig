use std::io::Write;

async fn download_reddit_data(sub: &str, key: &str, time: &str, num: usize) -> serde_json::Value {
    let val: serde_json::Value = serde_json::from_str(&reqwest::get(&format!(
        "https://www.reddit.com/r/{}/{}.json?t={}&limit={}",
        sub,
        key,
        time,
        num
    )).await.unwrap().text().await.unwrap()).unwrap(); 

    let val = val["data"]["children"].clone();
    val
}

pub async fn get_image_urls(sub: Option<String>, count: Option<u32>, key: Option<String>, timespan: Option<String>) -> Vec<String> {
    let limit = 100;
    let sub         =  if let Some(sub) = sub { sub } else { String::from("dankmemes") };
    let count        =  if let Some(count) = count { count } else { 1 };
    let key         =  if let Some(key) = key { key } else { String::from("new") };
    let timespan    =  if let Some(timespan) = timespan { timespan } else { String::from("day") };

    let json = download_reddit_data(&sub, &key, &timespan, limit);
    let mut c = count;
    let mut urls = Vec::new();

    for post in json.await.as_array().unwrap().iter() {
        let image_url = &format!("{}", post["data"]["url"].as_str().unwrap());
        let image_url_chars: Vec<char> = image_url.to_lowercase().chars().collect();
        let image_type: &str = &image_url_chars[(image_url_chars.len()-3)..].iter().collect::<String>();
        match image_type {
            "png" | "jpg" | "gif" => {
                urls.push(String::from(image_url));

                c -= 1;
                if c <= 0 {
                    return urls;
                }
            }
            _ => {}
        }
    }

    urls
}

pub async fn _download(urls: &[&str]) {
    for (i, image_url) in urls.iter().enumerate() {
        let image_url_chars: Vec<char> = image_url.to_lowercase().chars().collect();
        let image_type: &str = &image_url_chars[(image_url_chars.len()-3)..].iter().collect::<String>();
        let image_data = reqwest::get(*image_url).await.unwrap().bytes().await.unwrap().to_vec();
        let mut path = format!("{}{}.{}", "", i, image_type);

        while let Ok(_) = std::fs::File::open(path.clone()) {
            path = format!("{}{}.{}", "", i, image_type);
        }

        println!("{} as {}", image_url, path);

        let mut file = std::fs::File::create(path).unwrap();
        file.write_all(&image_data).unwrap();
        file.flush().unwrap();
    }
}