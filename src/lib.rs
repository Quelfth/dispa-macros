use proclamation::prelude::*;

mod implementation;

macros! {
    attr dispatch => implementation::dispatch;
}
