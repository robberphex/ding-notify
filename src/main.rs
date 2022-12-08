use clap::Parser;
use dingtalk::DingTalk;
use std::path::PathBuf;
use tokio::fs;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// path to markdown file
    #[clap()]
    markdown_path: PathBuf,

    // the title of markdown message
    #[clap(short, long)]
    markdonw_title: Option<String>,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("finding markdown path {:?}!", args.markdown_path);

    let markdown_content = fs::read_to_string(args.markdown_path).await?;

    let dt = DingTalk::from_file("~/.dingtalk-token.json")?;
    let title =args.markdonw_title.unwrap_or("markdown".to_string());
    dt.send_markdown(&title, &markdown_content).await?;

    Ok(())
}
