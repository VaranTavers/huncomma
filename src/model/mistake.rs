#[derive(Clone)]
pub enum Msg {
    Static(&'static str),
    Dynamic(String),
}

#[derive(Clone)]
pub struct Mistake {
    pub prob: f64,
    pub msg: Msg
}

impl Mistake {
    pub fn new(msg: &'static str, prob: f64) -> Mistake {
        Mistake {
            prob,
            msg: Msg::Static(msg),
        }
    }

    pub fn new_dyn(msg: String, prob: f64) -> Mistake {
        Mistake {
            prob,
            msg: Msg::Dynamic(msg),
        }
    }

    pub fn new_dyn_str(msg: &str, prob: f64) -> Mistake {
        Mistake {
            prob,
            msg: Msg::Dynamic(String::from(msg)),
        }
    }

    pub fn get_str(&self) -> &str {
        match &self.msg {
            Msg::Dynamic(msg) => {
                msg.as_str()
            }
            Msg::Static(msg) => {
                msg
            }
        }
    }
}