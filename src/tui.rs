// All of this code was wrote by hand, please respect it by following the guideline in contributing.md
use crate::packages::{
    Parameters, ParametersType, get_parameters, get_supported_packages, reload_and_get_packages,
};
use crossterm::event::KeyCode;
use parking_lot::Mutex;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::Style,
    widgets::{Block, List, ListState},
};
use std::{
    sync::{
        LazyLock,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
mod popup;
// Here mark the start of the great define
/// The list of packages
pub static PACKAGES: LazyLock<Mutex<Vec<String>>> =
    LazyLock::new(|| Mutex::new(get_supported_packages()));
/// The currently selected package. Indexing at 0 (like a Vec)
pub static CURENTLY_SELECTED_PACKAGE_INDEX: AtomicUsize = AtomicUsize::new(0);
/// the currently selected parameter of a package. Indexing at 0 (like a Vec)
pub static CURENTLY_SELECTED_PARAMETER_INDEX: AtomicUsize = AtomicUsize::new(0);
/// The current state in a popup (0 -> button 1 is selected)
pub static CURRENT_POPUP_STATE: AtomicUsize = AtomicUsize::new(0);
/// The state (selecting package: 0, selecting parameter: 1, in popup: 2)
pub static CURENT_STATE: AtomicUsize = AtomicUsize::new(0);
/// Current package parameters
pub static PARAMETERS: LazyLock<Mutex<Option<Vec<Parameters>>>> = LazyLock::new(|| {
    Mutex::new(
        PACKAGES
            .lock()
            .first()
            .and_then(|package| get_parameters(package.clone())),
    )
});

/// Draw the app and handle keyboard input (`r` reloads packages, Esc quits).
pub fn start(terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
    loop {
        if let Ok(is_available) = crossterm::event::poll(Duration::from_millis(100))
            && is_available
        {
            match crossterm::event::read()?.as_key_event() {
                Some(key) => {
                    match key.code {
                        KeyCode::Up => match CURENT_STATE.load(Ordering::Relaxed) {
                            0 => {
                                let current =
                                    CURENTLY_SELECTED_PACKAGE_INDEX.load(Ordering::Relaxed);
                                if current >= 1 {
                                    CURENTLY_SELECTED_PACKAGE_INDEX
                                        .store(current - 1, Ordering::Relaxed);
                                    CURENTLY_SELECTED_PARAMETER_INDEX.store(0, Ordering::Relaxed);
                                    refresh_parameters();
                                }
                            }
                            1 => {
                                let current =
                                    CURENTLY_SELECTED_PARAMETER_INDEX.load(Ordering::Relaxed);
                                if current >= 1 {
                                    CURENTLY_SELECTED_PARAMETER_INDEX
                                        .store(current - 1, Ordering::Relaxed);
                                }
                            }
                            2 => {
                                let current = CURRENT_POPUP_STATE.load(Ordering::Relaxed);
                                if current >= 1 {
                                    CURRENT_POPUP_STATE.store(current - 1, Ordering::Relaxed);
                                }
                            }
                            _ => {}
                        },
                        KeyCode::Down => match CURENT_STATE.load(Ordering::Relaxed) {
                            0 => {
                                let current =
                                    CURENTLY_SELECTED_PACKAGE_INDEX.load(Ordering::Relaxed);
                                if current + 1 < PACKAGES.lock().len() {
                                    CURENTLY_SELECTED_PACKAGE_INDEX
                                        .store(current + 1, Ordering::Relaxed);
                                    CURENTLY_SELECTED_PARAMETER_INDEX.store(0, Ordering::Relaxed);
                                    refresh_parameters();
                                }
                            }
                            1 => {
                                let current =
                                    CURENTLY_SELECTED_PARAMETER_INDEX.load(Ordering::Relaxed);
                                let count = PARAMETERS
                                    .lock()
                                    .as_ref()
                                    .map_or(0, |parameters| parameters.len());
                                if current + 1 < count {
                                    CURENTLY_SELECTED_PARAMETER_INDEX
                                        .store(current + 1, Ordering::Relaxed);
                                }
                            }
                            2 => {
                                let count = PARAMETERS
                                    .lock()
                                    .as_ref()
                                    .and_then(|parameters| {
                                        parameters.get(
                                            CURENTLY_SELECTED_PARAMETER_INDEX
                                                .load(Ordering::Relaxed),
                                        )
                                    })
                                    .map(popup_option_count)
                                    .unwrap_or(0);
                                let current = CURRENT_POPUP_STATE.load(Ordering::Relaxed);
                                if current + 1 < count {
                                    CURRENT_POPUP_STATE.store(current + 1, Ordering::Relaxed);
                                }
                            }
                            _ => {}
                        },
                        KeyCode::Enter => {
                            let state = CURENT_STATE.load(Ordering::Relaxed);
                            match state {
                                0 => {
                                    CURENT_STATE.store(1, Ordering::Relaxed);
                                } // selecting package -> selecting parameter
                                1 => {
                                    CURRENT_POPUP_STATE.store(0, Ordering::Relaxed);
                                    CURENT_STATE.store(2, Ordering::Relaxed)
                                } // selecting parameter -> selecting state in popup
                                2 => {
                                    apply_popup_selection()?;
                                    CURRENT_POPUP_STATE.store(0, Ordering::Relaxed);
                                    CURENT_STATE.store(1, Ordering::Relaxed);
                                } // apply the selection and exit the popup
                                _ => {}
                            }
                        }
                        KeyCode::Esc => {
                            let state = CURENT_STATE.load(Ordering::Relaxed);
                            match state {
                                1 => {
                                    CURENT_STATE.store(0, Ordering::Relaxed);
                                    CURENTLY_SELECTED_PARAMETER_INDEX.store(0, Ordering::Relaxed);
                                } // selecting parameter --> selecting package
                                2 => {
                                    CURRENT_POPUP_STATE.store(0, Ordering::Relaxed);
                                    CURENT_STATE.store(1, Ordering::Relaxed);
                                } // exit the popup without applying
                                _ => return Ok(()),
                            }
                        }
                        KeyCode::Char('r') => {
                            *PACKAGES.lock() = reload_and_get_packages();
                            CURENTLY_SELECTED_PACKAGE_INDEX.store(0, Ordering::Relaxed);
                            CURENTLY_SELECTED_PARAMETER_INDEX.store(0, Ordering::Relaxed);
                            CURRENT_POPUP_STATE.store(0, Ordering::Relaxed);
                            CURENT_STATE.store(0, Ordering::Relaxed);
                            refresh_parameters();
                        }
                        _ => {}
                    };
                }
                None => continue,
            }
        }
        terminal.draw(render)?;
    }
}

/// Refresh `PARAMETERS` from the currently selected package.
fn refresh_parameters() {
    *PARAMETERS.lock() = PACKAGES
        .lock()
        .get(CURENTLY_SELECTED_PACKAGE_INDEX.load(Ordering::Relaxed))
        .and_then(|package| get_parameters(package.clone()));
}

/// Run the command matching the option selected in the popup.
fn apply_popup_selection() -> anyhow::Result<()> {
    let option = CURRENT_POPUP_STATE.load(Ordering::Relaxed);
    let parameter = PARAMETERS
        .lock()
        .as_ref()
        .and_then(|parameters| {
            parameters.get(CURENTLY_SELECTED_PARAMETER_INDEX.load(Ordering::Relaxed))
        })
        .cloned();

    let Some(parameter) = parameter else {
        return Ok(());
    };

    match &parameter.param_type {
        ParametersType::Bool { command } => {
            // The popup lists "No" first and "Yes" second.
            if option == 1 {
                command.clone().run()?;
            }
        }
        ParametersType::Choice { command, .. } => {
            if let Some(command) = command.get(option) {
                command.clone().run()?;
            }
        }
    }

    Ok(())
}

/// define everything that you want to be on screen here.
fn render(frame: &mut Frame) {
    let [sidebar_area, content_side] =
        Layout::horizontal([Constraint::Fill(1), Constraint::Fill(3)]).areas(frame.area());
    {
        let packages_list = List::new(PACKAGES.lock().clone())
            .direction(ratatui::widgets::ListDirection::TopToBottom)
            .block(Block::bordered().title("Packages"))
            .highlight_symbol(">>> ")
            .highlight_style(Style::default().reversed());
        let mut sidebar_state = ListState::default();
        sidebar_state.select(Some(
            CURENTLY_SELECTED_PACKAGE_INDEX.load(Ordering::Relaxed),
        ));
        frame.render_stateful_widget(packages_list, sidebar_area, &mut sidebar_state);
    };
    {
        let inner_layout =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(content_side);
        let parameters = PARAMETERS.lock().clone();

        let left_side: Vec<String> = parameters
            .iter()
            .flatten()
            .map(|param| param.param_name.clone())
            .collect();

        let right_side: Vec<ParametersType> = parameters
            .iter()
            .flatten()
            .map(|param| param.param_type.clone())
            .collect();

        let parameters_list = List::new(left_side)
            .direction(ratatui::widgets::ListDirection::TopToBottom)
            .highlight_symbol(">>> ")
            .highlight_style(Style::default().reversed());

        let mut parameters_list_state = ListState::default();
        parameters_list_state.select(Some(
            CURENTLY_SELECTED_PARAMETER_INDEX.load(Ordering::Relaxed),
        ));

        let type_list = List::new(right_side.iter().map(|param_type| match param_type {
            ParametersType::Bool { .. } => "[]",
            ParametersType::Choice { .. } => "",
        }))
        .direction(ratatui::widgets::ListDirection::TopToBottom)
        .highlight_symbol("    ")
        .highlight_style(Style::default().reversed());

        let mut type_list_state = ListState::default();
        type_list_state.select(Some(
            CURENTLY_SELECTED_PARAMETER_INDEX.load(Ordering::Relaxed),
        ));

        frame.render_stateful_widget(parameters_list, inner_layout[0], &mut parameters_list_state);
        frame.render_stateful_widget(type_list, inner_layout[1], &mut type_list_state);
    };
    if CURENT_STATE.load(Ordering::Relaxed) == 2 {
        popup::render_popup(frame);
    }
}

fn popup_option_count(param: &Parameters) -> usize {
    match &param.param_type {
        ParametersType::Bool { .. } => 2,
        ParametersType::Choice { choices, .. } => choices.len(),
    }
}
