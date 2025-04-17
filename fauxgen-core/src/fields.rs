/// A struct representing an integer field type
#[derive(Debug, Clone, PartialEq)]
pub struct FieldTypeInt {
    /// Lower bound of generated values
    pub ge: Option<f64>,
    /// Upper bound of generated values
    pub le: Option<f64>,
    /// Field description
    pub description: Option<String>,
}

/// A struct representing a float field type
#[derive(Debug, Clone, PartialEq)]
pub struct FieldTypeFloat {
    /// Lower bound of generated values
    pub ge: Option<f64>,
    /// Upper bound of generated values
    pub le: Option<f64>,
    /// Field description
    pub description: Option<String>,
}

/// A struct representing a string field type
#[derive(Debug, Clone, PartialEq)]
pub struct FieldTypeString {
    /// Minimum length of generated strings
    pub min_length: Option<usize>,
    /// Maximum length of generated strings
    pub max_length: Option<usize>,
    /// Field description
    pub description: Option<String>,
}

/// A struct representing a boolean field type
#[derive(Debug, Clone, PartialEq)]
pub struct FieldTypeBool {
    /// Field description
    pub description: Option<String>,
}

/// A struct representing a datetime field type
#[derive(Debug, Clone, PartialEq)]
pub struct FieldTypeDatetime {
    /// Field description
    pub description: Option<String>,
}

/// A struct representing a date field type
#[derive(Debug, Clone, PartialEq)]
pub struct FieldTypeDate {
    /// Field description
    pub description: Option<String>,
}

/// A struct representing a date field type
#[derive(Debug, Clone, PartialEq)]
pub struct FieldTypeAny {
    /// Field description
    pub description: Option<String>,
}

/// An enum representing field types in the schema
#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    Int(FieldTypeInt),
    Float(FieldTypeFloat),
    String(FieldTypeString),
    Bool(FieldTypeBool),
    Datetime(FieldTypeDatetime),
    Date(FieldTypeDate),
    Any(FieldTypeAny),
}

/// A struct representing a field in the schema
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    /// Field name
    pub name: String,
    /// Field type
    pub field_type: FieldType,
    /// Whether the field can be null
    pub nullable: bool,
}

impl Field {
    /// Returns the Python type annotation as a string
    pub fn get_type_annotation(&self) -> String {
        let f = match self.field_type {
            FieldType::Int(_) => "int",
            FieldType::Float(_) => "float",
            FieldType::String(_) => "str",
            FieldType::Bool(_) => "bool",
            FieldType::Datetime(_) => "datetime.datetime",
            FieldType::Date(_) => "datetime.date",
            FieldType::Any(_) => "Any",
        };
        if self.nullable {
            format!("{} | None", f)
        } else {
            f.to_string()
        }
    }

    /// Gets lower and upper bounds for numeric fields
    ///
    /// If one bound is not specified, sets a reasonable range
    fn get_le_and_ge(&self, ge: Option<f64>, le: Option<f64>) -> (f64, f64) {
        match (ge, le) {
            (Some(ge), Some(le)) => (ge, le),
            (Some(ge), None) => (ge, ge + 100.0),
            (None, Some(le)) => (le - 100.0, le),
            _ => (0.0, 100.0),
        }
    }

    /// Returns a Python method call string for generating fake data
    pub fn get_faker_method(&self) -> String {
        match self.field_type {
            FieldType::Int(FieldTypeInt { ge, le, .. }) => {
                let (ge, le) = self.get_le_and_ge(ge, le);
                format!("f.gen_int(ge={}, le={}, seed=seed_)", ge, le)
            }
            FieldType::Float(FieldTypeFloat { ge, le, .. }) => {
                let (ge, le) = self.get_le_and_ge(ge, le);
                format!("f.gen_float(ge={}, le={}, seed=seed_)", ge, le)
            }
            FieldType::String(FieldTypeString {
                min_length,
                max_length,
                ..
            }) => {
                let min_length = min_length.unwrap_or(5);
                let max_length = max_length.unwrap_or(min_length + 5);
                format!(
                    "f.gen_string(min_length={}, max_length={}, seed=seed_)",
                    min_length, max_length
                )
            }
            FieldType::Bool(_) => "f.gen_bool(seed=seed_)".to_string(),
            FieldType::Datetime(_) => {
                let from = "datetime.datetime(2020, 1, 1)";
                let to = "datetime.datetime(2021, 1, 1)";
                format!(
                    "f.gen_datetime(from_datetime={}, to_datetime={}, seed=seed_)",
                    from, to
                )
            }
            FieldType::Date(_) => {
                let from = "datetime.date(2020, 1, 1)";
                let to = "datetime.date(2021, 1, 1)";
                format!("f.gen_date(from_date={}, to_date={}, seed=seed_)", from, to)
            }

            FieldType::Any(_) => "None".to_string(),
        }
    }
    pub fn get_description(&self) -> String {
        match &self.field_type {
            FieldType::Int(field) => field.description.clone(),
            FieldType::Float(field) => field.description.clone(),
            FieldType::String(field) => field.description.clone(),
            FieldType::Bool(field) => field.description.clone(),
            FieldType::Datetime(field) => field.description.clone(),
            FieldType::Date(field) => field.description.clone(),
            FieldType::Any(field) => field.description.clone(),
        }
        .unwrap_or_else(|| format!("Field {}", self.name))
    }
}
