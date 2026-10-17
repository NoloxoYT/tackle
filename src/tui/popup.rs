// All of this code was wrote by hand, please respect it by following the guideline in contributing.md
use super::{
    CURENTLY_SELECTED_PACKAGE_INDEX, CURENTLY_SELECTED_PARAMETER_INDEX, CURRENT_POPUP_STATE,
    PACKAGES, PARAMETERS,
};
use crate::packages::ParametersType;
use ratatui::{
    Frame,
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Text},
};
use std::sync::atomic::Ordering;
use tui_popup::Popup;
pub fn render_popup(frame: &mut Frame) {
    let selected_pkg = CURENTLY_SELECTED_PACKAGE_INDEX.load(Ordering::Relaxed);
    let Some(package) = PACKAGES.lock().get(selected_pkg).cloned() else {
        return;
    };
    let Some(parameters) = PARAMETERS.lock().clone() else {
        return;
    };
    let selected_param = CURENTLY_SELECTED_PARAMETER_INDEX.load(Ordering::Relaxed);
    let Some(param) = parameters.get(selected_param) else {
        return;
    };

    let selected = CURRENT_POPUP_STATE.load(Ordering::Relaxed);

    let mut lines = vec![Line::from(param.param_name.clone()).bold(), Line::from("")];

    match &param.param_type {
        ParametersType::Bool { .. } => {
            lines.push(option_line("No", selected == 0));
            lines.push(option_line("Yes", selected == 1));
        }
        ParametersType::Choice { choices, .. } => {
            for (i, choice) in choices.iter().enumerate() {
                lines.push(option_line(choice, selected == i));
            }
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from("↑/↓ select · Enter apply · Esc cancel").dark_gray());

    let popup = Popup::new(Text::from(lines))
        .title(format!("{package} · {}", param.param_name))
        .border_set(border::ROUNDED)
        .border_style(Style::new().fg(Color::Cyan))
        .style(Style::new().bg(Color::Black));

    frame.render_widget(popup, frame.area());
}

fn option_line(label: &str, selected: bool) -> Line<'static> {
    let text = format!("{}[ {label} ]", if selected { ">>> " } else { "    " });
    if selected {
        Line::from(text).reversed()
    } else {
        Line::from(text)
    }
}
