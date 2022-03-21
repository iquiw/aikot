use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use anyhow::Error;
use gtmpl::error::ExecError;
use gtmpl::{Context, Template, Value};

pub struct PassTmpl {
    tmpl: Template,
}

impl PassTmpl {
    pub fn new() -> Self {
        PassTmpl {
            tmpl: Template::default(),
        }
    }

    pub fn load_default(&mut self) -> Result<(), Error> {
        Ok(self.tmpl.parse("{{ .Content }}\n")?)
    }

    pub fn load<P>(&mut self, tmpl_path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let mut f = File::open(&tmpl_path)?;
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;
        Ok(self.tmpl.parse(buf)?)
    }

    pub fn render(&self, content: &str, path: &str) -> Result<String, ExecError> {
        let mut map = HashMap::<String, Value>::new();
        map.insert("Content".to_string(), content.into());
        map.insert("Path".to_string(), path.into());
        let context = Context::from(map);
        self.tmpl.render(&context)
    }
}

#[cfg(test)]
mod test {
    use std::env;
    use std::io::Write;

    use super::*;

    #[test]
    fn render_default() {
        let mut ptmpl = PassTmpl::new();
        ptmpl.load_default().unwrap();
        let result = ptmpl.render("context", "path");
        assert!(result.is_ok());
        assert_eq!(&result.unwrap(), "context\n");
    }

    #[test]
    fn render_from_file() {
        let mut path = env::temp_dir().clone();
        path.push("pass-template");
        let mut f = File::create(&path).unwrap();
        write!(
            f,
            "{{{{ .Content }}}}\nhost: {{{{ .Path }}}}\nurl: https://{{{{ .Path }}}}\n"
        )
        .unwrap();
        drop(f);
        let mut ptmpl = PassTmpl::new();
        ptmpl.load(&path).unwrap();
        let result = ptmpl.render("context", "path");
        assert!(result.is_ok());
        assert_eq!(&result.unwrap(), "context\nhost: path\nurl: https://path\n");
    }
}
