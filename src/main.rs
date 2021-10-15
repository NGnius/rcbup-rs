mod args;
mod context;

use std::io::Write; // FIXME (crash) file IO errors cause panics

use libfj::robocraft::FactoryRobotGetInfo;

fn main() {
    let cli_args = args::CliArguments::from_env();
    if cli_args.empty() {
        // TODO GUI mode
    } else {
        // run as CLI application
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        // search for robots
        let robots = runtime
            .block_on(collect_robots(&cli_args));
        // save robots data
        // FIXME sometimes CRF API sends the same robot twice
        let out = cli_args.out.clone().unwrap_or("./".into());
        std::fs::create_dir_all(&out).unwrap();
        let mut handles = Vec::new();
        if cli_args.thumbnail {
            // save robot thumbnails
            for bot in &robots {
                let handle = runtime
                    .spawn(download_thumbnail(bot.clone(), out.clone()));
                handles.push(handle);
            }
        }
        // save robot json files
        let ext = cli_args.extension.clone().unwrap_or("bot".into());
        for bot in &robots {
            let path = bot_filename(bot, &out, &ext);
            let output = std::fs::File::create(path).unwrap();
            serde_json::to_writer_pretty(output, bot).unwrap();
        }
        runtime.block_on(async {
            for handle in handles {
                handle.await.unwrap();
            }
        });
        //runtime.shutdown_timeout(std::time::Duration::from_secs(5*60)); // 5 minutes
    }
}

#[inline(always)]
fn bot_filename(info: &FactoryRobotGetInfo, out: &str, ext: &str) -> String {
    format!("{}/{}-{}.{}", &out, info.item_name, info.item_id, ext)
}

#[inline(always)]
fn jpg_filename(info: &FactoryRobotGetInfo, out: &str) -> String {
    format!("{}/{}-{}.jpg", out, &info.item_name, info.item_id)
}

async fn collect_robots(cli_args: &args::CliArguments) -> Vec<FactoryRobotGetInfo> {
    let mut result = Vec::new();
    let out = cli_args.out.clone().unwrap_or(".".into());
    let ext = cli_args.extension.clone().unwrap_or("bot".into());
    //println!("Search param: {}", cli_args.search.clone().unwrap_or_else(|| "[no search]".into()));
    let ctx: context::Context = cli_args.clone().into();
    if let Ok(search) = ctx.get_search().await {
        let response_len = search.roboshop_items.len();
        let max = if cli_args.max.is_some() && cli_args.max.unwrap() < response_len {cli_args.max.unwrap()} else {response_len};
        for bot_index in 0..max {
            let bot_info = &search.roboshop_items[bot_index];
            if let Ok(extra_info) = ctx.get_extra_info(bot_info).await {
                // FIXME robot is actually already downloaded when this message is output
                println!("Downloading robot {} to {}", &bot_info.item_name, bot_filename(&extra_info, &out, &ext));
                result.push(extra_info);
            }
        }
    }
    result
}

async fn download_thumbnail(info: FactoryRobotGetInfo, out: String) {
    let path = jpg_filename(&info, &out);
    println!("Downloading thumbnail {} to {}", &info.item_name, path);
    match reqwest::get(&info.thumbnail).await {
        Err(e) => eprintln!("Failed to download thumbnail: {}", e),
        Ok(resp) => {
            // save file
            let mut output = std::fs::File::create(path).unwrap();
            output.write(&resp.bytes().await.unwrap()).unwrap();
        }
    }
}
