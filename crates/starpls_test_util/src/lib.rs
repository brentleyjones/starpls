use starpls_bazel::{
    builtin::{Callable, Param, Type, Value},
    Builtins,
};
use starpls_syntax::{TextRange, TextSize};

pub const CURSOR_MARKER: &str = "$0";

pub fn parse_fixture(fixture: &str) -> (String, TextSize, Vec<TextRange>) {
    let offset = fixture.find(CURSOR_MARKER).unwrap();
    let mut contents = String::new();
    contents.push_str(&fixture[..offset]);
    contents.push_str(&fixture[offset + CURSOR_MARKER.len()..]);
    let selected_ranges = find_selected_ranges(&contents);
    (contents, (offset as u32).into(), selected_ranges)
}

fn find_selected_ranges(contents: &str) -> Vec<TextRange> {
    let mut line_starts = vec![TextSize::new(0)];
    let mut ranges = Vec::new();
    for line in contents.split_inclusive('\n') {
        if let Some(start) = line.find("#^") {
            let remaining = &line[start + "#^".len()..];
            let additional_carets = remaining.chars().take_while(|c| c == &'^').count();
            if let Some(prev_line_start) = line_starts.get(line_starts.len() - 2) {
                let range_start = prev_line_start + TextSize::try_from(start).unwrap();
                let range_end =
                    range_start + TextSize::try_from("#^".len() + additional_carets).unwrap();
                ranges.push(TextRange::new(range_start, range_end))
            }
        }
        line_starts.push(line_starts.last().unwrap() + TextSize::of(line));
    }
    ranges
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FixtureType {
    pub name: String,
    pub fields: Vec<(String, String)>,
}

pub fn make_test_builtins(functions: Vec<String>, types: Vec<FixtureType>) -> Builtins {
    Builtins {
        global: functions
            .iter()
            .map(|name| Value {
                name: name.to_string(),
                callable: Some(Callable {
                    param: vec![
                        Param {
                            name: "*args".to_string(),
                            is_star_arg: true,
                            ..Default::default()
                        },
                        Param {
                            name: "**kwargs".to_string(),
                            is_star_star_arg: true,
                            ..Default::default()
                        },
                    ],
                    return_type: "Unknown".to_string(),
                }),
                ..Default::default()
            })
            .collect(),
        r#type: types
            .into_iter()
            .map(|ty| Type {
                name: ty.name,
                field: ty
                    .fields
                    .into_iter()
                    .map(|field| Value {
                        name: field.0,
                        r#type: field.1,
                        ..Default::default()
                    })
                    .collect(),
                ..Default::default()
            })
            .collect(),
    }
}
