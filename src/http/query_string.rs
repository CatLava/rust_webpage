use std::collections::HashMap;


// example a=1&b=2&c=3&d=&e&f===&d=abc
// there are multiple examples to handle, how to put these into
//  for this we can create an enum, this will help handle all the use cases
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}

// handling multiple values
// Have to specify array length at compile time
// If unknown, will need to put this on the heap
// heap allocated dynamic array is a vec
pub enum Value {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf>{
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

// will use From Trait
// This will split the string for processing
//

// example a=1&b=2&c=3&d=&e&f===&d=abc
impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        // iterate through the splits and process
        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";
            // finding index of equal sign to seperate on
            if let Some(i) = s.find('='){

            }
        }

        QueryString { data }
        unimplemented!()
    }
}
