use lognplot::time::TimeStamp;
use lognplot::tsdb::observations::{Observation, Sample};
/// Demo of raw database performance.
///
/// Strategy: insert 1 million points, and measure how long it took.
use lognplot::tsdb::{Query, TsDb, TsDbApi};
use std::time::Instant;

fn main() {
    let mut db = TsDb::default();

    insertions(&mut db);
    do_query(&db);
}

fn insertions(db: &mut TsDb) {
    let num_insertions = 1_000_000;
    println!(
        "Created database, now inserting {} data points into a signal.",
        num_insertions
    );

    let t1 = Instant::now();
    for i in 0..num_insertions {
        let ts = TimeStamp::new(i as f64);
        let sample = Sample::new(i as f64);
        let observation = Observation::new(ts, sample);
        db.add_value("fu", observation);
    }
    let t2 = Instant::now();
    let time_delta = t2 - t1;
    let time_delta = time_delta.as_secs_f64();

    println!(
        "Inserted {} points in {} seconds.",
        num_insertions, time_delta
    );
    let rate = num_insertions as f64 / time_delta;
    println!("That means {} mega-points per second.", rate / 1.0e6);
}

fn do_query(db: &TsDb) {
    // Query the data
    println!("Querying the database!");

    let query = Query::create()
        .start(TimeStamp::new(0.0))
        .end(TimeStamp::new(1000.0))
        .build();
    let result = db.query("fu", query).unwrap();

    println!("Num result: {:?}", result.len());
    // let raw_samples = result.into_vec();
    // println!("Raw samples: {}", raw_samples.len());
}
