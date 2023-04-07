use std::collections::HashMap;

pub struct QueryString<'buf>{
    data: HashMap<&'buf str, Value<'buf>>
}

pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>)
}

impl <'buf> QueryString <'buf>{
    pub fn get(self, key: &'buf str) -> Option<Value>{
        self.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut value = "";
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                value = &sub_str[i + 1..];
            }
            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_val) => {
                        // swap the memory that existing points to using dereferencing
                        *existing = Value::Multiple(vec![prev_val, value]);
                    }
                    Value::Multiple(vec) => vec.push(value)
                })
                .or_insert(Value::Single(value));
        }
        QueryString{data}
    }
}