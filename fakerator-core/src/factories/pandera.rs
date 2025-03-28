use crate::{
    fields::{Field, FieldType, FieldTypeFloat, FieldTypeInt, FieldTypeString},
    strings,
};
use anyhow::{Ok, Result};
use askama::Template;
use rustpython_parser::ast::{ExprAttribute, ExprCall, ExprSubscript, StmtAnnAssign, StmtClassDef};
use std::fmt::Debug;

pub struct PanderaHandler<R: Debug> {
    _phantom: std::marker::PhantomData<R>,
    pa_int_regex: regex::Regex,
    pa_float_regex: regex::Regex,
}

struct PanderaFieldParameter {
    ge: Option<f64>,
    le: Option<f64>,
    nullable: Option<bool>,
}

impl Default for PanderaFieldParameter {
    fn default() -> Self {
        Self {
            ge: None,
            le: None,
            nullable: None,
        }
    }
}

#[derive(Template)]
#[template(path = "pandera_factory.py")]
struct PanderaFactoryTemplate<'a> {
    record_class_name: &'a str,
    record_factory_name: &'a str,
    fields: Vec<Field>,
}

impl<R: Debug> PanderaHandler<R> {
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
            pa_int_regex: regex::Regex::new(r"^(Int|Int8|Int16|Int32|Int64)$").unwrap(),
            pa_float_regex: regex::Regex::new(r"^(Float|Float16|Float32|Float64|Float128)$")
                .unwrap(),
        }
    }

    pub fn generate_pandera_dataframe_factory(
        &self,
        class_def: &StmtClassDef<R>,
    ) -> Result<Option<String>> {
        let (class_name, fields) = match self.parse_pandera_dataframe_model(class_def)? {
            Some(result) => result,
            None => return Ok(None),
        };

        let record_class_name = &format!("{}Record", class_name);
        Ok(Some(
            PanderaFactoryTemplate {
                record_class_name: &format!("{}Record", class_name),
                record_factory_name: strings::to_snake_case(&record_class_name).as_str(),
                fields,
            }
            .render()?,
        ))
    }

    fn parse_pandera_dataframe_model(
        &self,
        class_def: &StmtClassDef<R>,
    ) -> Result<Option<(String, Vec<Field>)>> {
        if !self.is_pandera_dataframe_model(class_def) {
            return Ok(None); // not a pandera dataframe model
        };

        let class_name = class_def.name.to_string();
        let fields: Vec<Field> = class_def
            .body
            .iter()
            .filter_map(|stmt| stmt.as_ann_assign_stmt())
            .map(|ann_assign| self.get_field_from_ann_assign(ann_assign))
            .collect::<Result<Vec<Option<Field>>>>()?
            .into_iter()
            .flatten()
            .collect();
        Ok(Some((class_name, fields)))
    }

    // check the class based on `pa.DataFrameModel`
    // NOTE: this function does not check inheritance
    // TODO: check inheritance
    fn is_pandera_dataframe_model(&self, class_def: &StmtClassDef<R>) -> bool {
        class_def.bases.iter().any(|base| {
            let Some(ExprAttribute { value, attr, .. }) = base.as_attribute_expr() else {
                return false;
            };
            if value
                .as_name_expr()
                .is_some_and(|name| name.id.as_str() != "pa")
            {
                return false;
            }
            attr == "DataFrameModel"
        })
    }

    fn get_pandera_parameter_from_field_value(
        &self,
        field_value: &ExprCall<R>,
    ) -> Result<Option<PanderaFieldParameter>> {
        let mut ge = None;
        let mut le = None;
        let mut nullable = None;

        for keyword in field_value.keywords.iter() {
            let Some(arg) = &keyword.arg else {
                continue;
            };
            let Some(value) = keyword.value.as_constant_expr() else {
                continue;
            };
            if arg.as_str() == "ge" {
                let ge_float = value.value.as_float().cloned();
                let ge_int = value
                    .value
                    .as_int()
                    .map(|v| v.to_string().parse::<f64>())
                    .transpose()?;
                ge = ge_float.or(ge_int);
            } else if arg.as_str() == "le" {
                let le_float = value.value.as_float().cloned();
                let le_int = value
                    .value
                    .as_int()
                    .map(|v| v.to_string().parse::<f64>())
                    .transpose()?;
                le = le_float.or(le_int);
            } else if arg.as_str() == "nullable" {
                nullable = value.value.as_bool().cloned();
            }
        }
        Ok(Some(PanderaFieldParameter { ge, le, nullable }))
    }

    fn get_field_type_from_series_ann(
        &self,
        field_ann: &ExprSubscript<R>,
        pandera_field_parameter: &PanderaFieldParameter,
    ) -> Option<FieldType> {
        if field_ann
            .value
            .as_name_expr()
            .is_some_and(|name| name.id.as_str() != "Series")
        {
            return None;
        }

        // handle type of pandera like pa.Int
        if let Some(attribute) = field_ann.slice.as_attribute_expr() {
            if attribute
                .value
                .as_name_expr()
                .is_some_and(|name| name.id.as_str() != "pa")
            {
                return None;
            }
            if self.pa_int_regex.is_match(attribute.attr.as_str()) {
                return Some(FieldType::Int(FieldTypeInt {
                    ge: pandera_field_parameter.ge,
                    le: pandera_field_parameter.le,
                }));
            }
            if self.pa_float_regex.is_match(attribute.attr.as_str()) {
                return Some(FieldType::Float(FieldTypeFloat {
                    ge: pandera_field_parameter.ge,
                    le: pandera_field_parameter.le,
                }));
            }
            if attribute.attr.as_str() == "String" {
                return Some(FieldType::String(FieldTypeString {
                    min_length: None,
                    max_length: None,
                }));
            }
            if attribute.attr.as_str() == "Bool" {
                return Some(FieldType::Bool);
            }
            if attribute.attr.as_str() == "Date" {
                return Some(FieldType::Date);
            }
            if attribute.attr.as_str() == "DateTime" {
                return Some(FieldType::Datetime);
            }
        }

        // handle type of python like int, float, str, bool
        if let Some(attribute) = field_ann.slice.as_name_expr() {
            if attribute.id.as_str() == "int" {
                return Some(FieldType::Int(FieldTypeInt {
                    ge: pandera_field_parameter.ge,
                    le: pandera_field_parameter.le,
                }));
            }
            if attribute.id.as_str() == "float" {
                return Some(FieldType::Float(FieldTypeFloat {
                    ge: pandera_field_parameter.ge,
                    le: pandera_field_parameter.le,
                }));
            }
            if attribute.id.as_str() == "str" {
                return Some(FieldType::String(FieldTypeString {
                    min_length: None,
                    max_length: None,
                }));
            }
            if attribute.id.as_str() == "bool" {
                return Some(FieldType::Bool);
            }
        }
        None
    }

    fn get_field_from_ann_assign(
        &self,
        ann_assign_stmt: &StmtAnnAssign<R>,
    ) -> Result<Option<Field>> {
        let Some(name) = ann_assign_stmt
            .target
            .as_name_expr()
            .map(|name_expr| name_expr.id.to_string())
        else {
            return Ok(None);
        };
        let Some(field_ann) = ann_assign_stmt.annotation.as_subscript_expr() else {
            return Ok(None);
        };

        let field_value = ann_assign_stmt.value.as_ref().map(|v| v.as_call_expr());
        let pandera_field_param = match field_value
            .and_then(|v| v)
            .map(|v| self.get_pandera_parameter_from_field_value(v))
        {
            Some(result) => result?.unwrap_or_default(),
            None => PanderaFieldParameter::default(),
        };

        let Some(field_type) =
            self.get_field_type_from_series_ann(&field_ann, &pandera_field_param)
        else {
            return Ok(None);
        };

        Ok(Some(Field {
            name,
            field_type,
            nullable: pandera_field_param.nullable.unwrap_or(false),
        }))
    }
}
