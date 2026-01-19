#[link(name = "native_replacements", kind = "static")]
unsafe extern "C" {
    pub fn flip_duration();
    pub fn flip_timer();
    pub fn katamari_view_ascend();
    pub fn katamari_view_descend();
    pub fn spindash_gain_power();
    pub fn spindash_spinning();
}
