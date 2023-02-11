use crate::template::{generate_template, name};
use crate::generated::*;
use std::io::ErrorKind;
use std::path::PathBuf;
use texcgen_macro::run_templates;
use texcore::template::Template;
use texcore::Tex;
use tokio::fs::{copy, create_dir, File, OpenOptions};
use tokio::io::{AsyncWriteExt, Error, Result};
use tokio::try_join;

// The template builder config
#[derive(Debug, Clone)]
pub struct Builder {
    // This is where all of the template json, tex and pdf files would be stored
    out_dir: PathBuf,
    // This is to determine what build profile to use on the template(s)
    /*
    1 - Only builds the JSON file in {out}/json/
    2 - Builds TeX file in {out}/tex/
    3 - Builds JSON and TeX in ...
     */
    build_level: u8,
}

impl Default for Builder {
    fn default() -> Self {
        Self::new(1)
    }
}

impl Builder {
    pub fn new(build_level: u8) -> Self {
        Self {
            out_dir: PathBuf::from("out"),
            build_level,
        }
    }
    fn json_path(&self) -> PathBuf {
        self.out_dir.join("json")
    }
    fn tex_path(&self) -> PathBuf {
        self.out_dir.join("tex")
    }
    fn paths(&self, filename: &str) -> Vec<PathBuf> {
        // order will always be [json, tex]
        let mut paths = Vec::with_capacity(2);
        paths.push(self.json_path().join(&format!("{filename}.json")));
        paths.push(self.tex_path().join(&format!("{filename}.tex")));
        paths
    }
    async fn build_file(&self, path: &PathBuf, data: String) -> Result<()> {
        let mut file = File::create(path).await?;
        file.write_all(data.as_bytes()).await?;
        Ok(())
    }
    // creates all of the necessary directories
    pub async fn create_dirs(&self) -> Result<()> {
        create_dir(&self.out_dir).await?;
        create_dir(self.json_path()).await?;
        create_dir(self.tex_path()).await?;
        Ok(())
    }
    // builds depending on the build level
    pub async fn build(&self, template: &Template) -> Result<()> {
        let paths = self.paths(&template.name);
        match self.build_level {
            1 => {
                // only create json
                let json = template.to_json_string();
                println!("Generating {}", &paths[0].display());
                self.build_file(&paths[0], json).await?;
            }
            2 => {
                let tex = template.to_latex_string();
                println!("Generating {}", &paths[1].display());
                self.build_file(&paths[1], tex).await?;
            }
            3 => {
                let json = template.to_json_string();
                let tex = template.to_latex_string();

                try_join!(
                    {
                        println!("Generating {}", &paths[0].display());
                        self.build_file(&paths[0], json)
                    },
                    {
                        println!("Generating {}", &paths[1].display());
                        self.build_file(&paths[1], tex)
                    }
                )
                .unwrap();
            }
            _ => return Err(Error::from(ErrorKind::InvalidData)),
        }
        Ok(())
    }
    pub async fn build_all(&self, templates: Vec<Template>) -> Result<()> {
        for t in templates {
            self.build(&t).await?;
        }
        Ok(())
    }
}

pub async fn generate(builder: Builder) -> Result<()> {
    // copies the template as a generated file
    let path = PathBuf::from("src/generated");
    let filename = format!("{}.rs", name());
    let _ = File::create(path.join(&filename)).await?;

    copy("src/template.rs", path.join(&filename)).await?;
    // updates generated.rs
    let mut gen_rs = OpenOptions::new()
        .append(true)
        .open("src/generated.rs")
        .await?;
    let mod_ = format!("pub mod {};\n", name());
    gen_rs.write_all(mod_.as_bytes()).await?;
    // creates the template
    let template = generate_template();
    // need to consider Builder
    builder.build(&template).await?;
    // format the code
    let _ = tokio::process::Command::new("cargo")
        .arg("fmt")
        .spawn()
        .unwrap();
    Ok(())
}

pub async fn generate_all(builder: Builder) -> Result<()> {
    let templates = run_templates!();
    builder.build_all(templates).await?;
    Ok(())
}

fn custom_dir() -> Option<PathBuf> {
    let path = PathBuf::from(".texcreate").join("custom");
    match dirs::home_dir() {
        None => None,
        Some(p) => Some(p.join(path)),
    }
}

pub async fn save(name: &str) -> Result<()> {
    let custom = custom_dir();

    match custom {
        None => {
            return Err(Error::from(ErrorKind::InvalidData));
        }
        Some(p) => {
            let builder = Builder::default();
            let json_path = builder.json_path();
            generate_all(builder).await?;
            let file_name = format!("{name}.json");

            let from_path = json_path.join(&file_name);
            let to_path = p.join(&file_name);

            let _ = File::create(&to_path).await?;

            copy(&from_path, &to_path).await?;
        }
    }

    Ok(())
}
