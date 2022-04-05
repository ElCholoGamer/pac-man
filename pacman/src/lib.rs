pub mod util {
    use sdl2::keyboard::Mod;

    pub fn has_ctrl(keymod: Mod) -> bool {
        keymod.contains(Mod::LCTRLMOD) || keymod.contains(Mod::RCTRLMOD)
    }
}
