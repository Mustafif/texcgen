mod builder;
mod generated;
mod template;

use crate::builder::{generate, generate_all, save, Builder};
use structopt::StructOpt;
use tokio::{io::{Result, AsyncWriteExt}, fs::{File, read_to_string}};

#[derive(StructOpt)]
#[structopt(about = "A template generator for the TexCreate project")]
enum CLI {
    #[structopt(about = "Initialize Output Directory Structure")]
    Init,
    #[structopt(about = "Refreshes `src/template.rs` using the default template.")]
    Refresh, 
    #[structopt(about = "Generate template")]
    Gen {
        #[structopt(short, long)]
        level: Option<u8>,
    },
    #[structopt(about = "Generate all templates in `src/generated`")]
    GenAll {
        #[structopt(short, long)]
        level: Option<u8>,
    },
    #[structopt(about = "Saves template to TexCreate custom directory")]
    Save {
        #[structopt(short, long)]
        name: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = CLI::from_args();

    match cli {
        CLI::Init => Builder::default().create_dirs().await?,
        CLI::Refresh => {
            println!("Refreshing default template...");
            let mut template = File::create("src/template.rs").await?;
            let s = read_to_string("default/default_template.rs").await?;
            template.write_all(s.as_bytes()).await?;
        }
        CLI::Gen { level } => {
            let builder = match level {
                None => Builder::default(),
                Some(l) => Builder::new(l),
            };
            generate(builder).await?
        }
        CLI::GenAll { level } => {
            let builder = match level {
                None => Builder::default(),
                Some(l) => Builder::new(l),
            };
            generate_all(builder).await?
        }
        CLI::Save { name } => save(&name).await?,
    }

    Ok(())
}
