use dioxus::prelude::*;
use willi::WilliDocument;

use crate::components::{
  button::Button,
  card::Card,
  flex::{Column, Row},
  input_row::InputRow,
  reorderable_list::{ReorderableList, ReorderableListItem},
  text_input::TextInput,
};

#[derive(Clone, PartialEq, Props)]
pub struct StudentFormProps {
  schedule: Signal<Option<WilliDocument>>,
}

#[component]
pub fn StudentForm(props: StudentFormProps) -> Element {
  // TODO: this should be a memo over a signal of subjects.
  let subjects = use_signal(|| vec![
    ReorderableListItem { key: "physik".to_string(), data: "Physik" },
    ReorderableListItem { key: "chemie".to_string(), data: "Chemie" },
  ]);

  rsx! {
    Card {
      title: rsx! { Fragment { "Praktikanten-Eigenschaften" } },
      buttons: rsx! {
        Button {
          disabled: true,
          "Plan erstellen"
        }
      },

      InputRow {
        label: "Name",

        TextInput {
          placeholder: "Vor- und Nachname des Praktikanten"
        }
      }

      p {
        "Ordnen sie dem Praktikanten bis zu 3 Fächer zu, die für die Planung verwendet
         werden sollen."
      }

      InputRow {
        label: "Unterrichtsfächer",
        description: rsx! {
          "Ziehen sie die Punkte, um Fächer nach Priorität in Reihenfolge zu bringen.
           Das 1. Fach wird bei der Verteilung am stärksten berücksichtigt, die darauf
           Folgenden nach und nach weniger."
        },

        Column {
          style: "
            flex: 1 1 0;
            width: 100%;
            max-width: var(--input-max-width);
          ",

          Row {
            style: "
              width: 100%;
              max-width: unset;
              align-items: stretch;
            ",
            TextInput {
              placeholder: "Hier tippen, um ein Fach zu wählen"
            },
            Button {
              "Hinzufügen"
            }
          }

          ReorderableList {
            items: subjects,
            render: move |(key, subjects): (_, Signal<Vec<ReorderableListItem<&'static str>>>)| rsx! {
              TextInput {
                placeholder: subjects.read().iter().find(|s| s.key == key).map(|s| s.data.to_string())
              }
            }
          }
        }
      }
    }
  }
}
