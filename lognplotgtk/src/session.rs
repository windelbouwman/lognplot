use super::error_dialog::show_error;
use super::state::GuiStateHandle;
use gtk::prelude::*;
use lognplot::chart::Chart;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub dashboard: Vec<DashBoardItem>,
}

impl Session {
    pub fn new() -> Self {
        Session { dashboard: vec![] }
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

/// Popup a dialog to save session for later usage.
pub fn save_session(top_level: &gtk::Window, app_state: &GuiStateHandle) {
    let dialog = gtk::FileChooserDialog::with_buttons(
        Some("Export session as JSON"),
        Some(top_level),
        gtk::FileChooserAction::Save,
        &[
            ("Cancel", gtk::ResponseType::Cancel),
            ("Save", gtk::ResponseType::Accept),
        ],
    );
    let res = dialog.run();
    let filename = dialog.filename();

    if let gtk::ResponseType::Accept = res {
        if let Some(filename) = filename {
            info!("Saving session to filename: {:?}", filename);
            let res = { app_state.borrow().save_session(&filename) };
            if let Err(err) = res {
                let error_message = format!("Error saving session to {:?}: {}", filename, err);
                error!("{}", error_message);
                show_error(top_level, &error_message);
            } else {
                info!("Session saved!");
            }
        }
    }
}

/// Popup a dialog to restore a session from before.
pub fn load_session(top_level: &gtk::Window, app_state: &GuiStateHandle) {
    let dialog = gtk::FileChooserDialog::with_buttons(
        Some("Import session from JSON file"),
        Some(top_level),
        gtk::FileChooserAction::Open,
        &[
            ("Cancel", gtk::ResponseType::Cancel),
            ("Open", gtk::ResponseType::Accept),
        ],
    );

    let res = dialog.run();
    let filename = dialog.filename();

    if let gtk::ResponseType::Accept = res {
        if let Some(filename) = filename {
            info!("Loading session to filename: {:?}", filename);
            let res = { app_state.borrow_mut().load_session(&filename) };
            if let Err(err) = res {
                let error_message = format!("Error loading session from {:?}: {}", filename, err);
                error!("{}", error_message);
                show_error(top_level, &error_message);
            } else {
                info!("Session loaded!");
            }
        }
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
