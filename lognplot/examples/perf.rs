/// Demo of raw database performance.
///
/// Strategy: insert 1 million points, and measure how long it took.
use lognplot::tsdb::{Query, Sample, SubResult, TsDb};
use lognplot::time::{TimeStamp};
use std::time::Instant;

fn main() {
    let mut db = TsDb::new();

    insertions(&mut db);
    do_query(&db);
}

fn insertions(db: &mut TsDb) {
    let num_insertions = 1_000_000;
    println!(
        "Created database, now inserting {} data points into a signal.",
        num_insertions
    );

    db.new_trace("fu");
    let t1 = Instant::now();
    for i in 0..num_insertions {
        let ts = TimeStamp::new(i as f64);
        let sample = Sample::new(ts, i as f64);
        db.add_value("fu", sample);
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
    let result = db.get_values("fu", query);

    println!("Got result: {:?}", result.query);
    println!("Num result: {:?}", result.samples.len());
    let raw_samples = result.into_vec();
    println!("Raw samples: {}", raw_samples.len());
}
