use crate::*;

use std::os::raw::*;

use ctor::ctor;

/************
 * Settings *
 ************/

static mut SETTINGS: Option<Settings> = None;

#[ctor]
unsafe fn ctor() {
    SETTINGS = Some(Default::default());
}

#[no_mangle]
unsafe extern "C" fn settings_get_bar_on_top_by_default() -> bool {
    SETTINGS.unwrap().bar_on_top_by_default()
}

#[no_mangle]
unsafe extern "C" fn settings_set_bar_on_top_by_default(value: bool) {
    SETTINGS.unwrap().bar_on_top_by_default_set(value);
}

/***************
 * Constraints *
 ***************/

#[no_mangle]
extern "C" fn constraints_border_width(border_width: c_int) -> c_int {
    constraints::border_width(border_width)
}

#[no_mangle]
extern "C" fn constraints_default_clients_in_master(
    default_clients_in_master: c_int,
) -> c_int {
    constraints::default_clients_in_master(default_clients_in_master)
}

#[no_mangle]
extern "C" fn constraints_default_master_area_factor(
    default_master_area_factor: c_float,
) -> c_float {
    constraints::default_master_area_factor(default_master_area_factor)
}

#[no_mangle]
extern "C" fn constraints_gap_size(gap_size: c_int) -> c_int {
    constraints::gap_size(gap_size)
}

#[no_mangle]
extern "C" fn constraints_master_area_factor(
    master_area_factor: c_float,
) -> c_float {
    constraints::master_area_factor(master_area_factor)
}

#[no_mangle]
extern "C" fn constraints_max_clients_in_master(
    max_clients_in_master: c_int,
) -> c_int {
    constraints::max_clients_in_master(max_clients_in_master)
}

#[no_mangle]
extern "C" fn constraints_snap_distance(snap_distance: c_uint) -> c_uint {
    constraints::snap_distance(snap_distance)
}

/********
 * Geom *
 ********/

#[no_mangle]
unsafe extern "C" fn position_init(position: *mut geom::Position) {
    *position = Default::default();
}

#[no_mangle]
unsafe extern "C" fn sizes_init(sizes: &mut geom::Sizes) {
    *sizes = Default::default();
}

#[no_mangle]
unsafe extern "C" fn basic_geom_init(basic_geom: &mut geom::BasicGeom) {
    *basic_geom = Default::default();
}

#[no_mangle]
unsafe extern "C" fn win_geom_init(win_geom: &mut geom::WinGeom) {
    *win_geom = Default::default();
}

#[no_mangle]
unsafe extern "C" fn position_init_from_args(
    position: *mut geom::Position,
    x: c_int,
    y: c_int,
) {
    *position = geom::Position::new(x, y);
}

#[no_mangle]
unsafe extern "C" fn sizes_init_from_args(
    sizes: *mut geom::Sizes,
    width: c_int,
    height: c_int,
) {
    *sizes = geom::Sizes::new(width, height);
}

#[no_mangle]
unsafe extern "C" fn basic_geom_init_from_args(
    basic_geom: *mut geom::BasicGeom,
    x: c_int,
    y: c_int,
    width: c_int,
    height: c_int,
) {
    *basic_geom = geom::BasicGeom::new(
        geom::Position::new(x, y),
        geom::Sizes::new(width, height),
    );
}

#[no_mangle]
unsafe extern "C" fn win_geom_init_from_args(
    win_geom: *mut geom::WinGeom,
    x: c_int,
    y: c_int,
    width: c_int,
    height: c_int,
    border_width: c_int,
) {
    *win_geom = geom::WinGeom::new(
        geom::BasicGeom::new(
            geom::Position::new(x, y),
            geom::Sizes::new(width, height),
        ),
        border_width,
    );
}

#[no_mangle]
unsafe extern "C" fn win_geom_total_width(
    win_geom: *const geom::WinGeom,
) -> c_int {
    (*win_geom).total_sizes().width()
}

#[no_mangle]
unsafe extern "C" fn win_geom_total_height(
    win_geom: *const geom::WinGeom,
) -> c_int {
    (*win_geom).total_sizes().height()
}
