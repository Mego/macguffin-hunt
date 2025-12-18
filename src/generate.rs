use std::fs::File;

use itertools::Itertools;
use serde::Serialize;

static TABLE_TEMPLATE: &str = r#"
<html>
<head>
<title>Macguffin Hunt Results</title>
<link rel="stylesheet" type="text/css" href="index.css" />
</head>
<body>
<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Source Code Link</th>
            <th>Best</th>
            <th>Average</th>
            <th>Worst</th>
            <th>Total</th>
        </tr>
    </thead>
    <tbody>
        {% for entry in data %}
        <tr>
            <td>{{ entry.name }}</td>
            <td>{{ entry.link }}</td>
            <td>{{ entry.best }}</td>
            <td>{{ entry.avg }}</td>
            <td>{{ entry.worst }}</td>
            <td>{{ entry.total }}</td>
        </tr>
        {% endfor %}
    </tbody>
</table>
</body>
</html>
"#;

#[derive(Serialize)]
struct Entry {
    name: String,
    link: String,
    best: usize,
    avg: f64,
    worst: usize,
    total: usize,
}

impl From<csv::StringRecord> for Entry {
    fn from(value: csv::StringRecord) -> Self {
        Self {
            name: value[0].to_string(),
            link: value[1].to_string(),
            best: value[2].parse().unwrap(),
            avg: value[3].parse().unwrap(),
            worst: value[4].parse().unwrap(),
            total: value[5].parse().unwrap(),
        }
    }
}

fn main() {
    let mut engine = upon::Engine::new();
    engine.add_template("table", TABLE_TEMPLATE).unwrap();
    let reader = csv::Reader::from_path("data/data.csv").unwrap();
    let entries = reader
        .into_records()
        .map(|r| Entry::from(r.unwrap()))
        .collect_vec();
    let mut output = File::create("index.html").unwrap();
    engine
        .get_template("table")
        .unwrap()
        .render(upon::value! {data: entries})
        .to_writer(&mut output)
        .unwrap();
}
