use std::ops::RangeInclusive;

use egui::{text::{CCursor, CCursorRange}, DragValue, Response, TextEdit, Ui, Widget};

pub fn select_text_on_tab(text_length: usize, text_edit: TextEdit, ui: &mut Ui) -> Response {
    let mut text_edit = text_edit.show(ui);
    if text_edit.response.gained_focus() && !text_edit.response.hovered() {
        text_edit.state.cursor.set_char_range(Some(
            CCursorRange::two(
                CCursor::new(0), 
                CCursor::new(text_length))
            )
        );
        text_edit.state.store(ui.ctx(), text_edit.response.id)
    }
    text_edit.response
}


// .style()
//                 .number_formatter
//                 .format(value, auto_decimals..=max_decimals),
pub fn select_drag_value_with_range_on_tab(val: &mut u32, range: RangeInclusive<u32>, ui: &mut Ui) -> Response{


    let drag_value = DragValue::new(val).range(range).ui(ui);
    if drag_value.gained_focus() && !drag_value.hovered() {
        let mut state = TextEdit::load_state(ui.ctx(), drag_value.id).unwrap_or_default();
        state.cursor.set_char_range(Some(
            CCursorRange::two(
                CCursor::new(0), 
                CCursor::new(ui.style().number_formatter.format(*val as f64, 0..=0).len()))
            )
        );
        state.store(ui.ctx(), drag_value.id)
    }
    drag_value
}