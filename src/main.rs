use std::io;
use std::io::prelude::*;
use serde_json::Value;
use failure::format_err;
use colored::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "j2l", about = "Converts a stream of JSON logs to human-readable logs")]
struct Opt {
    /// Print field stack_trace
    #[structopt(short, long)]
    stacktrace: bool,

    /// Additionnal fields
    #[structopt(short, long)]
    addfields: Vec<String>,
}

fn extract_field<'a>(map: &'a serde_json::map::Map<String, Value>, field: &str) -> Result<&'a str, failure::Error> {
  let value = map
      .get(field).ok_or(format_err!("field `{}` no found in json", field))?
      .as_str().ok_or(format_err!("field `{}` is not a string", field))?;
  Ok(value)
}

fn parse_log(json: serde_json::Value, opt: &Opt) -> Result<String, failure::Error> {
  // extract hardcoded fields
  let map = json.as_object().ok_or(format_err!("invalid json"))?;
  let timestamp = extract_field(map, "@timestamp")?;
  let level = extract_field(map, "level")?;
  let hostname = extract_field(map, "hostname")?;
  let logger_name = extract_field(map, "logger_name")?;
  let message = extract_field(map, "message")?;

  // extract custom additional fields
  let af: Vec<_> = opt.addfields.iter().map(|f|{
    extract_field(map, f).unwrap_or("-")
  }).collect();
  
  // extract optional stack trace
  let stack_trace = extract_field(map, "stack_trace");

  let mut log = format!("{} {} {} {}", timestamp.red(), level.green(), hostname.blue().bold(), logger_name.yellow());

  for field in af {
    log.push_str(&format!(" {}", field.red().bold()));
  }

  log.push_str(&format!(" {}", message.white()));

  match stack_trace {
    Ok(st) if opt.stacktrace => {
        log.push_str("\n");
        log.push_str(st);
    }
    _ => ()
  };

  Ok(log)
}

fn main() -> Result<(),failure::Error> {
  let opt = Opt::from_args();

  for line in io::stdin().lock().lines() {
     let line = line?;
     let res: serde_json::Result<Value> = serde_json::from_str(&line); 
     let out = if let Ok(json) = res {
       parse_log(json, &opt).unwrap_or_else(|_|{line})
     } else {
       line
     };

     println!("{}", out);
  };

  Ok(())
}
