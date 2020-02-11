use lognplot::chart::Chart;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    dashboard: Vec<DashBoardItem>,
}

impl Session {
    pub fn new() -> Self {
        Session { dashboard: vec![] }
    }

    pub fn first(&self) -> Option<&DashBoardItem> {
        self.dashboard.first()
    }

    pub fn add_item(&mut self, item: DashBoardItem) {
        self.dashboard.push(item);
    }
}

/*
#[derive(Serialize, Deserialize, Debug)]
struct DashBoard {
    items: Vec<DashBoardItem>,
}
*/

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum DashBoardItem {
    #[serde(rename = "graph")]
    Graph { curves: Vec<String> },

    #[serde(rename = "empty")]
    Empty,
}

impl From<&Chart> for DashBoardItem {
    fn from(chart: &Chart) -> Self {
        let curves: Vec<String> = chart.curves.iter().map(|c| c.name()).collect();
        DashBoardItem::Graph { curves }
    }
}

#[cfg(test)]
mod tests {
    use super::{DashBoardItem, Session};

    #[test]
    fn test_session_decode() {
        let example_session = r#"
        {
            "dashboard": [
                {
                    "type": "graph",
                    "curves": [
                        "C3",
                        "C5"
                    ]
                },
                {
                    "type": "empty"
                },
                {
                    "type": "empty"
                },
                {
                    "type": "empty"
                }
            ]
        }
        "#;

        let session: Session = serde_json::from_str(&example_session).unwrap();

        assert_eq!(
            DashBoardItem::Graph {
                curves: vec!["C3".to_owned(), "C5".to_owned()]
            },
            session.dashboard[0]
        );
    }
}
