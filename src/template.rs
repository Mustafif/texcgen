// This is where your template will be created.
use texcore::template::*;
use texcore::*;

// Name of the template
static NAME: &str = "test";
// Description of the template
static DESC: &str = r#"This is a test template"#;

// Used to distribute the name
pub fn name() -> String {
    NAME.to_string()
}

// where all the template elements will be created
fn elements() -> Vec<Element<Any>> {
    let chapter = Chapter::new("A Chapter");
    let header = Header::new("A Header", 1);
    let subheader = Header::new("A Sub Header", 2);
    // This macro converts all TexCore types that implement
    // the `Tex` trait to Element<Any>
    Elements![chapter, header, subheader]
}

// where you may declare the version of the template
fn version() -> Version {
    let mut version = Version::new();
    // semantic versioning follows M.m.p
    // where M is major, m is minor and p is patch
    let major = 0;
    let minor = 0;
    let patch = 0;
    // sets the version
    version.set_version(major, minor, patch);
    version
}

// generates the template
pub fn generate_template() -> Template {
    // sets the metadata defaults of the template
    let metadata = Metadata::default();
    // we will create a new template setting the name, description and metadata
    let mut template = Template::new(NAME, DESC, &metadata);
    // sets the template's version
    template.version = version();
    // push the elements into the template
    template.push_element_array(elements());
    template
}
