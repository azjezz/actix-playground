use std::collections::HashMap;

use tarjama::{context::Context, context::Value as ContextValue, Translator};
use tera::{Error, Filter, Result, Value};

pub struct TranslatorFilter {
    translator: Translator,
    default_domain: String,
}

impl TranslatorFilter {
    pub fn new(translator: Translator) -> Self {
        Self {
            translator,
            default_domain: "messages".to_string(),
        }
    }
}

impl Filter for TranslatorFilter {
    fn filter(&self, value: &Value, args: &HashMap<String, Value>) -> Result<Value> {
        let id = value.as_str().unwrap();
        let locale = args.get("locale").unwrap().as_str().unwrap();
        let domain = match args.get("domain") {
            Some(d) => d.as_str().unwrap(),
            None => &self.default_domain,
        };
        let context = match args.get("context") {
            Some(c) => {
                let (count, parameters) = if c.is_object() {
                    let mut count = None;
                    let mut values = vec![];

                    let context_map = c.as_object().unwrap();
                    for (key, value) in context_map {
                        if key == "?" {
                            count = Some(value.as_i64().unwrap());
                            continue;
                        }

                        let v = match value {
                            Value::String(s) => ContextValue::String(s.clone()),
                            Value::Number(n) if n.is_i64() => {
                                ContextValue::Integer(n.as_i64().unwrap())
                            }
                            Value::Number(n) => ContextValue::Double(n.as_f64().unwrap()),
                            _ => ContextValue::String(value.to_string()),
                        };
                        values.push((key.clone(), v));
                    }

                    (count, values)
                } else {
                    (Some(c.as_i64().unwrap()), vec![])
                };

                Context::new(parameters, count)
            }
            None => Context::new(vec![], None),
        };

        let result = self.translator.trans(locale, domain, id, context);

        match result {
            Ok(s) => Ok(Value::String(s)),
            Err(e) => Err(Error::msg(e.to_string())),
        }
    }
}
