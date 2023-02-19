use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct HttpHeaders(HashMap<String, String>);

impl HttpHeaders {
    pub fn empty() -> Self {
        HttpHeaders(HashMap::new())
    }

    pub fn add_header(mut self, header: impl Into<HttpHeader>) -> Self {
        let header: HttpHeader = header.into();
        self.0.insert(header.name, header.value);
        self
    }

    pub fn inner(&self) -> &HashMap<String, String> {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct HttpHeader {
    name: String,
    value: String,
}

impl HttpHeader {
    fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

pub mod header {
    use super::HttpHeader;

    #[derive(Clone, Debug, PartialEq)]
    pub enum Accept {
        JSON,
    }

    impl From<Accept> for HttpHeader {
        fn from(it: Accept) -> Self {
            let value = match it {
                Accept::JSON => "application/json",
            };
            HttpHeader::new("Accept", value)
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum ContentType {
        JSON,
    }

    impl From<ContentType> for HttpHeader {
        fn from(it: ContentType) -> Self {
            let value = match it {
                ContentType::JSON => "application/json",
            };
            HttpHeader::new("Content-Type", value)
        }
    }
}
