// pub mod作用
pub fn init() {
    use crate::dir_1::dir_1_file::call_my_name;
    call_my_name();

    use crate::dir_1::dir_2_file::dir_2_file_fn;
    dir_2_file_fn();

    use crate::dir_1::my_app::use_my_app_1;
    use_my_app_1();
}