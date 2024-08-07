#[derive(Debug, Clone)]
pub struct RandomIntText {
    inclusive_min: i64,
    exclusive_max: i64,
    digits: Option<usize>,
}

impl RandomIntText {
    pub fn new(inclusive_min: i64, exclusive_max: i64) -> Self {
        Self {
            inclusive_min,
            exclusive_max,
            digits: None,
        }
    }

    pub fn new_with_digits(inclusive_min: i64, exclusive_max: i64, digits: usize) -> Self {
        Self {
            inclusive_min,
            exclusive_max,
            digits: Some(digits),
        }
    }
}

impl serde::Serialize for RandomIntText {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.digits {
            None => {
                serializer.serialize_str(&format!("{}, {}", self.inclusive_min, self.exclusive_max))
            }
            Some(digits) => serializer.serialize_str(&format!(
                "{:0>width$}, {:0>width$}",
                self.inclusive_min,
                self.exclusive_max,
                width = digits
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RandomOfText {
    pub elements: Vec<String>,
    pub delimiter: String,
}

impl serde::Serialize for RandomOfText {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!(
            "{}{}",
            self.delimiter,
            self.elements.join(&self.delimiter)
        ))
    }
}

#[derive(serde::Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Expression {
    #[serde(rename = "CONST")]
    Const { text: String },
    // EXTRACT { text: String },
    // EXTRACTHEADER { text: String },
    // VARIABLE { text: String },
    #[serde(rename = "RANDOMINT")]
    RandomInt { text: RandomIntText },
    #[serde(rename = "RANDOMSTRING")]
    RandomString { text: usize },
    #[serde(rename = "RANDOMOF")]
    RandomOf { text: RandomOfText },
    // RANDOMUUID,
    // INTARITHMETIC { text: String },
    #[serde(rename = "LOCALNOW")]
    LocalNow { text: String },
    #[serde(rename = "COMPOSITE")]
    Composite { children: Vec<Expression> },
    // TEMPLATE { text: String },
}
