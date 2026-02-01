use proclamation::prelude::*;

mod dispatch;

macros! {
    attr dispatch => dispatch::dispatch;
}
