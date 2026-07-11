pub struct SearchRecord {
    status: String,
    total: u32,
}

impl SearchRecord {
    pub fn new(status: impl Into<String>, total: u32) -> Self {
        Self {
            status: status.into(),
            total,
        }
    }
}

pub enum SearchFilter {
    Equals { field: String, value: String },
    GreaterThan { field: String, value: u32 },
}

impl SearchFilter {
    pub fn parse(input: &str) -> Result<Self, String> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 3 {
            return Err("filter must have field operator value".to_string());
        }

        match parts[1] {
            "=" => Ok(Self::Equals {
                field: parts[0].to_string(),
                value: parts[2].to_string(),
            }),
            ">" => Ok(Self::GreaterThan {
                field: parts[0].to_string(),
                value: parts[2]
                    .parse()
                    .map_err(|_| "greater-than value must be numeric".to_string())?,
            }),
            operator => Err(format!("unsupported operator {operator}")),
        }
    }

    pub fn matches(&self, record: &SearchRecord) -> bool {
        match self {
            Self::Equals { field, value } if field == "status" => record.status == *value,
            Self::GreaterThan { field, value } if field == "total" => record.total > *value,
            _ => false,
        }
    }
}
