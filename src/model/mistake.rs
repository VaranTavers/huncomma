pub enum Msg {
    Static(&'static str),
    Dynamic(String),
}

pub struct Mistake {
    prob: f64,
    msg: Msg
}

impl Mistake {
    fn new(msg: &'static str, prob: f64) -> Mistake {
        Mistake {
            prob,
            msg: Msg::Static(msg),
        }
    }

    fn new_dyn(msg: String, prob: f64) -> Mistake {
        Mistake {
            prob,
            msg: Msg::Dynamic(msg),
        }
    }

    fn new_dyn_str(msg: &str, prob: f64) -> Mistake {
        Mistake {
            prob,
            msg: Msg::Dynamic(String::from(msg)),
        }
    }

    fn get_str(&self) -> &str {
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