use rsass::output::Format;
use rsass::{
    Error, FileContext, FsFileContext, ScopeRef, SourceFile, SourceName,
};
use std::collections::BTreeMap;

pub fn runner() -> TestRunner {
    TestRunner::new()
}

#[derive(Clone)]
pub struct TestRunner {
    format: Format,
    file_context: TestFileContext,
}

#[derive(Clone, Debug)]
struct TestFileContext {
    parent: FsFileContext,
    mock: BTreeMap<String, String>,
    cwd: String,
}

impl TestFileContext {
    fn new() -> Self {
        let mut parent = FsFileContext::new();
        parent.push_path("tests/spec".as_ref());
        TestFileContext {
            parent,
            mock: BTreeMap::new(),
            cwd: String::new(),
        }
    }
    fn mock_file(&mut self, name: &str, content: &str) {
        self.mock.insert(name.into(), content.into());
    }
    #[allow(unused)] // only used while building tests
    fn has_mock_files(&self) -> bool {
        !self.mock.is_empty()
    }
    fn with_cwd(&mut self, cwd: &str) {
        self.cwd = url_join(&self.cwd, cwd);
    }
}

fn url_join(p: &str, c: &str) -> String {
    if p.is_empty() {
        c.to_string()
    } else if c.is_empty() {
        p.to_string()
    } else if p.ends_with('/') {
        format!("{}{}", p, c)
    } else {
        format!("{}/{}", p, c)
    }
}

use std::io::Cursor;
use std::io::Read;

impl FileContext for TestFileContext {
    type File = Cursor<Vec<u8>>;

    fn find_file(
        &self,
        name: &str,
    ) -> Result<Option<(String, Self::File)>, Error> {
        let mut cwd = self.cwd.trim_end_matches('/');
        let mut lname = name;
        while let Some(name) = lname.strip_prefix("../") {
            cwd = cwd.rfind('/').map(|p| &self.cwd[..p]).unwrap_or("");
            lname = name;
        }
        let tname = url_join(cwd, lname);

        if let Some(data) = self.mock.get(&tname) {
            return Ok(Some((
                name.to_string(),
                Cursor::new(data.as_bytes().to_vec()),
            )));
        }

        Ok(self.parent.find_file(name)?.map(|(name, mut file)| {
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).unwrap();
            (name, Cursor::new(buf))
        }))
    }
}

impl TestRunner {
    pub fn new() -> TestRunner {
        TestRunner {
            format: Default::default(),
            file_context: TestFileContext::new(),
        }
    }
    pub fn set_precision(mut self, precision: usize) -> Self {
        self.format = Format {
            precision,
            ..self.format
        };
        self
    }
    pub fn mock_file(mut self, name: &str, content: &str) -> Self {
        self.file_context.mock_file(name, content);
        self
    }
    #[allow(unused)] // only used while building tests
    pub fn has_files(&self) -> bool {
        self.file_context.has_mock_files()
    }
    pub fn with_cwd(mut self, cwd: &str) -> Self {
        self.file_context.with_cwd(cwd);
        self
    }
    #[allow(unused)] // only used while executing tests
    pub fn ok(&self, input: &str) -> String {
        match self.rsass(input) {
            Ok(css) => String::from_utf8(css).unwrap().replace("\n\n", "\n"),
            Err(err) => panic!("Unexpected error:\n{}\n", err),
        }
    }
    #[allow(unused)] // only used while executing tests
    pub fn err(&self, input: &str) -> String {
        match self.rsass(input) {
            Ok(css) => panic!(
                "Unexpected result:\n{}\n",
                String::from_utf8(css).unwrap()
            ),
            Err(err) => err.to_string(),
        }
    }
    #[allow(unused)] // only used while building tests
    pub fn run(&self, input: &str) -> Result<String, String> {
        self.rsass(input)
            .map(|css| String::from_utf8(css).unwrap().replace("\n\n", "\n"))
            .map_err(|err| err.to_string())
    }

    fn rsass(&self, input: &str) -> Result<Vec<u8>, Error> {
        let name = SourceName::root("input.scss");
        self.format.write_root(
            SourceFile::read(&mut input.as_bytes(), name)?.parse()?,
            ScopeRef::new_global(self.format),
            &self.file_context,
        )
    }
}
