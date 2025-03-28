#[derive(Debug, Clone, PartialEq)]
pub struct FieldTypeInt {
    pub ge: Option<f64>,
    pub le: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldTypeFloat {
    pub ge: Option<f64>,
    pub le: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldTypeString {
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    Int(FieldTypeInt),
    Float(FieldTypeFloat),
    String(FieldTypeString),
    Bool,
    Datetime,
    Date,
}

// add struct field, which has name and type and optional

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
    pub nullable: bool,
}

impl Field {
    pub fn get_type_annotation(&self) -> String {
        let f = match self.field_type {
            FieldType::Int(_) => "int",
            FieldType::Float(_) => "float",
            FieldType::String(_) => "str",
            FieldType::Bool => "bool",
            FieldType::Datetime => "datetime.datetime",
            FieldType::Date => "datetime.date",
        };
        if self.nullable {
            format!("{} | None", f)
        } else {
            f.to_string()
        }
    }

    fn get_le_and_ge(&self, ge: Option<f64>, le: Option<f64>) -> (f64, f64) {
        match (ge, le) {
            (Some(ge), Some(le)) => (ge, le),
            (Some(ge), None) => (ge, ge + 100.0),
            (None, Some(le)) => (le - 100.0, le),
            _ => (0.0, 100.0),
        }
    }

    pub fn get_faker_method(&self) -> String {
        match self.field_type {
            FieldType::Int(FieldTypeInt { ge, le }) => {
                let (ge, le) = self.get_le_and_ge(ge, le);
                format!("f.gen_int(ge={}, le={})", ge, le)
            }
            FieldType::Float(FieldTypeFloat { ge, le }) => {
                let (ge, le) = self.get_le_and_ge(ge, le);
                format!("f.gen_float(ge={}, le={})", ge, le)
            }
            FieldType::String(FieldTypeString {
                min_length,
                max_length,
            }) => {
                let min_length = min_length.unwrap_or(5);
                let max_length = max_length.unwrap_or(min_length + 5);
                format!(
                    "f.gen_string(min_length={}, max_length={})",
                    min_length, max_length
                )
            }
            FieldType::Bool => "f.gen_bool()".to_string(),
            FieldType::Datetime => {
                let from = "datetime.datetime(2020, 1, 1)";
                let to = "datetime.datetime(2021, 1, 1)";
                format!("f.gen_datetime(from_datetime={}, to_datetime={})", from, to)
            }
            FieldType::Date => {
                let from = "datetime.date(2020, 1, 1)";
                let to = "datetime.date(2021, 1, 1)";
                format!("f.gen_date(from_date={}, to_date={})", from, to)
            }
        }
    }
}
