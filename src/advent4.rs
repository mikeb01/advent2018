
use std::io::{BufRead, BufReader, Error};
use std::fs::File;
use chrono::{Timelike, NaiveDateTime};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Debug)]
enum GuardAction {
    StartShift(i32),
    Sleep,
    Wake
}

#[derive(Debug)]
struct Guard {
    id: i32,
    total_minutes_asleep: i64,
    count_per_minute: HashMap<i32, i32>
}

#[derive(Clone, Debug)]
struct GuardEvent {
    timestamp: NaiveDateTime,
    action: GuardAction
}

pub fn advent4a() -> Result<(Option<i32>, Option<i32>), Error> {

    let f = File::open("input4.txt")?;
    let mut events: Vec<GuardEvent> = Vec::new();

    for buffer in BufReader::new(f).lines() {
        let line = buffer?;
        let date_time_str: String;
        let guard_no: i32;

        if line.contains("begins shift") {
            scan!(line.bytes() => "[{}] Guard #{} begins shift", date_time_str, guard_no);
            let action = GuardAction::StartShift(guard_no);

            let timestamp = NaiveDateTime::parse_from_str(date_time_str.as_str(), "%Y-%m-%d %H:%M").unwrap();
            events.push(GuardEvent{ timestamp, action });
        }
        else if line.contains("falls asleep") {
            scan!(line.bytes() => "[{}]", date_time_str);
            let action = GuardAction::Sleep;

            let timestamp = NaiveDateTime::parse_from_str(date_time_str.as_str(), "%Y-%m-%d %H:%M").unwrap();
            events.push(GuardEvent{ timestamp, action });
        }
        else if line.contains("wakes up") {
            scan!(line.bytes() => "[{}]", date_time_str);
            let action = GuardAction::Wake;

            let timestamp = NaiveDateTime::parse_from_str(date_time_str.as_str(), "%Y-%m-%d %H:%M").unwrap();
            events.push(GuardEvent{ timestamp, action });
        }
    }

    events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    let mut guards_by_id: HashMap<i32, Guard> = HashMap::new();
    let mut start_sleep: Option<NaiveDateTime> = None;
    let mut currency_guard_id: i32 = -1;

    for event in events.iter() {

        match event.action {
            GuardAction::StartShift(n) => {
                if guards_by_id.get_mut(&n).is_none() {
                    guards_by_id.insert(n, Guard { id: n, total_minutes_asleep: 0, count_per_minute: HashMap::new() });
                }
                currency_guard_id = n;
            }
            GuardAction::Sleep => {
                start_sleep = Some(event.timestamp);
            }
            GuardAction::Wake => {

                let current_guard = guards_by_id.get_mut(&currency_guard_id);
                if current_guard.is_some() && start_sleep.is_some() {
                    let start = start_sleep.unwrap();
                    let guard = current_guard.unwrap();

                    let duration = event.timestamp.signed_duration_since(start);
                    let minutes = duration.num_minutes();
                    guard.total_minutes_asleep += minutes;

                    let start_minute = (start.hour() * 60 +  start.minute()) as i32;
                    let finish_minute = start_minute + minutes as i32;

                    for m in start_minute..finish_minute {
                        let minute_count:i32 = *guard.count_per_minute.get(&m).unwrap_or(&0);
                        guard.count_per_minute.insert(m, minute_count + 1);
                    }
                }
            }
        }
    }


    let mut guards = Vec::new();
    for (_id, guard) in guards_by_id.drain() {
        guards.push(guard);
    }

    guards.sort_by(|a, b| a.total_minutes_asleep.cmp(&b.total_minutes_asleep));

    let solution_a = guards.last().map(|g| calc_sleepiest_minute(&g).0 * g.id);

    guards.sort_by(|a, b| calc_sleepiest_minute(a).1.cmp(&calc_sleepiest_minute(b).1));

    let solution_b = guards.last().map(|g| calc_sleepiest_minute(&g).0 * g.id);

    return Ok((solution_a, solution_b))
}

fn calc_sleepiest_minute(g: &Guard) -> (i32, i32) {
    let mut sleepiest_minute: i32 = -1;
    let mut max_sleep_count = 0;
    for (minute, count) in g.count_per_minute.iter() {
        if *count > max_sleep_count {
            sleepiest_minute = *minute;
            max_sleep_count = *count;
        }
    }
    (sleepiest_minute, max_sleep_count)
}