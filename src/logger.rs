use std::collections::HashMap;
use leon::{ParseError, Template, vals};
use crate::level::Level;
use crate::time;

pub struct Logger {
    template: Template<'static>,
    months: HashMap<u32, String>
}

impl Logger {
    pub fn new() -> Self {
        let months = HashMap::from([
            (1, "January".to_string()),
            (2, "February".to_string()),
            (3, "March".to_string()),
            (4, "April".to_string()),
            (5, "May".to_string()),
            (6, "June".to_string()),
            (7, "July".to_string()),
            (8, "August".to_string()),
            (9, "September".to_string()),
            (10, "October".to_string()),
            (11, "November".to_string()),
            (12, "December".to_string()),
        ]);

        Self {
            template: Template::parse("[{date} {time} {level}]\t{message}").unwrap(), // default template will not fail anyways
            months
        }
    }
    pub fn set_template(&mut self, template: &'static str) -> Result<(), ParseError> {
        self.template = match Template::parse(template) {
            Ok(template) => template,
            Err(e) => return Err(e)
        };
        Ok(())
    }
    pub fn log(&self, message: &str, level: Level) {
        let final_msg = self.template.render(
            &&vals(|key| {
                match key {
                    "level" =>      Some(level.to_string().into()),
                    "message" =>    Some(message.to_string().into()),
                    "hour" =>       Some(time::get_hour().into()),
                    "day" =>        Some(time::get_day().into()),
                    "month_num" =>  Some(time::get_month_num().into()),
                    "month_name" => Some(time::get_month_name(&self.months).into()),
                    "year" =>       Some(time::get_year().into()),
                    "time" =>       Some(time::get_time().into()),
                    "date" =>       Some(time::get_date().into()),
                    _ =>            Some("".into())
                }
            })
        ).unwrap();

        eprintln!("{}", final_msg);
    }

    pub fn debug(&self, message: &str) {
        self.log(message, Level::DEBUG);
    }
    pub fn info(&self, message: &str) {
        self.log(message, Level::INFO);
    }
    pub fn warn(&self, message: &str) {
        self.log(message, Level::WARN);
    }
    pub fn error(&self, message: &str) {
        self.log(message, Level::ERROR);
    }
}
