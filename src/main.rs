use std::time::{Duration, SystemTime, UNIX_EPOCH};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "TimeStamp Tool", about = "Display / offset  a timestamp. If timestamp not provided (0), current time is used.")]
struct Opt {
    #[structopt(short = "o", long = "offset")]
    offset: Option<String>,

    #[structopt(name = "Specific epoch timestamp", default_value = "0")]
    timestamp: u64,
}

fn main() {
    let opt = Opt::from_args();

    let offset_str = opt.offset.unwrap_or(String::from("0"));
    let offset = parse_offset_to_seconds(&offset_str);
    let timestamp = match opt.timestamp {
        0 => SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        _ => opt.timestamp,
    };

    // parse timestamp into i64
    let timestamp: i64 = timestamp.try_into().unwrap();
    println!("{}", timestamp + offset);
}

fn parse_offset_to_seconds(offset: &str) -> i64 {
    let is_negative = offset.starts_with('-');
    let offset = offset.trim_start_matches('-');
    let mut result: i64;

    // if it contains a 's' at the end, remove it and parse the number as seconds
    if offset.ends_with('s') {
        result = offset[..offset.len() - 1].parse::<i64>().unwrap();
    }
    // if it contains a 'm' at the end, remove it and parse the number as minutes
    else if offset.ends_with('m') {
        result = offset[..offset.len() - 1].parse::<i64>().unwrap() * 60;
    }
    // if it contains a 'h' at the end, remove it and parse the number as hours
    else if offset.ends_with('h') {
        result = offset[..offset.len() - 1].parse::<i64>().unwrap() * 60 * 60;
    }
    // if it contains a 'd' at the end, remove it and parse the number as days
    else if offset.ends_with('d') {
        result = offset[..offset.len() - 1].parse::<i64>().unwrap() * 60 * 60 * 24;
    }
    // if it contains a 'w' at the end, remove it and parse the number as weeks
    else if offset.ends_with('w') {
        result = offset[..offset.len() - 1].parse::<i64>().unwrap() * 60 * 60 * 24 * 7;
    }
    // if it contains a 'y' at the end, remove it and parse the number as years
    else if offset.ends_with('y') {
        result = offset[..offset.len() - 1].parse::<i64>().unwrap() * 60 * 60 * 24 * 365;
    }
    // if it contains a 'M' at the end, remove it and parse the number as months
    else if offset.ends_with('M') {
        result = offset[..offset.len() - 1].parse::<i64>().unwrap() * 60 * 60 * 24 * 30;
    }
    // if it contains a 'y' at the end, remove it and parse the number as years
    else if offset.ends_with('y') {
        result = offset[..offset.len() - 1].parse::<i64>().unwrap() * 60 * 60 * 24 * 365;
    } else {
        result = offset.parse::<i64>().unwrap();
    }

    if is_negative {
        result = -result;
    }
    result
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn test_parse_offset_positive() {
        assert_eq!(parse_offset_to_seconds("10s"), 10);
        assert_eq!(parse_offset_to_seconds("10m"), 600);
        assert_eq!(parse_offset_to_seconds("10h"), 36000);
        assert_eq!(parse_offset_to_seconds("10d"), 864000);
        assert_eq!(parse_offset_to_seconds("10w"), 6048000);
        assert_eq!(parse_offset_to_seconds("10y"), 315360000);
        assert_eq!(parse_offset_to_seconds("10M"), 25920000);
        assert_eq!(parse_offset_to_seconds("10y"), 315360000);
        assert_eq!(parse_offset_to_seconds("10"), 10);
        assert_eq!(parse_offset_to_seconds("37s"), 37);
        assert_eq!(parse_offset_to_seconds("22m"), 1320);
        assert_eq!(parse_offset_to_seconds("3h"), 10800);
        assert_eq!(parse_offset_to_seconds("4d"), 345600);
        assert_eq!(parse_offset_to_seconds("5w"), 3024000);
        assert_eq!(parse_offset_to_seconds("6y"), 189216000);
        assert_eq!(parse_offset_to_seconds("7M",), 18144000);
    }

    #[test]
    fn test_parse_offset_negative() {
        assert_eq!(parse_offset_to_seconds("-10s"), -10);
        assert_eq!(parse_offset_to_seconds("-10m"), -600);
        assert_eq!(parse_offset_to_seconds("-10h"), -36000);
        assert_eq!(parse_offset_to_seconds("-10d"), -864000);
        assert_eq!(parse_offset_to_seconds("-10w"), -6048000);
        assert_eq!(parse_offset_to_seconds("-10y"), -315360000);
        assert_eq!(parse_offset_to_seconds("-10M"), -25920000);
        assert_eq!(parse_offset_to_seconds("-10y"), -315360000);
        assert_eq!(parse_offset_to_seconds("-10"), -10);
        assert_eq!(parse_offset_to_seconds("-37s"), -37);
        assert_eq!(parse_offset_to_seconds("-22m"), -1320);
        assert_eq!(parse_offset_to_seconds("-3h"), -10800);
        assert_eq!(parse_offset_to_seconds("-4d"), -345600);
        assert_eq!(parse_offset_to_seconds("-5w"),-3024000);
        assert_eq!(parse_offset_to_seconds("-6y"), -189216000);
        assert_eq!(parse_offset_to_seconds("-7M",), -18144000);
        assert_eq!(parse_offset_to_seconds("-10"), -10);
    }
}