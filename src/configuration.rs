use config::{ConfigError, Config, File, Environment};

#[derive(Debug, Deserialize)]
pub struct Video {
    vsync: bool,
    hi_dpi: bool,
    anisotropy: bool,
    render_distance: u8,
    gui_scale: u8,
    particles: u8,
    fancy: bool,
    clouds: bool,
    item_tooltips: bool,
    fov: u8,
    entity_shadows: bool,
}

#[derive(Debug, Deserialize)]
pub struct MSAA {
    enabled: bool,
    quality: u8,
}

#[derive(Debug, Deserialize)]
pub struct Input {
    invert_mouse: bool,
    sensitivity: u8,
    touch_screen: bool,
}

#[derive(Debug, Deserialize)]
pub struct Gameplay<'a> {
    difficulty: u8,
    resource_packs: Vec<&'a str>,
    direct_connect: &'a str,
    lang: &'a str,
    right_hand: bool,
    attack_indicator: bool,
    old_combat: bool,
    subtitles: bool,
    autojump: bool,
    narrator: bool,
    tutorial_step: u8,
}

#[derive(Debug, Deserialize)]
pub struct Window {
    width: u16,
    height: u16,
    fullscreen: bool,
}

#[derive(Debug, Deserialize)]
pub struct Chat {
    enabled: bool,
    links: bool,
    opacity: u8,
}

#[derive(Debug, Deserialize)]
pub struct Settings<'a> {
    version: u8,
    video: Video,
    #[serde(rename = "MSAA")]
    msaa: MSAA,
    input: Input,
    #[serde(borrow)]
    gameplay: Gameplay<'a>,
    window: Window,
    chat: Chat,
}

impl<'a> Settings<'a> {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s
            .set_default("version", 1)?

            // Video
            .set_default("video.vsync", true)?
            .set_default("video.hi_dpi", false)?
            .set_default("video.anisotropy", false)?
            .set_default("video.render_distance", 12)?
            .set_default("video.gui_scale", 1)?
            .set_default("video.particles", 2)?
            .set_default("video.fancy", true)?
            .set_default("video.clouds", true)?
            .set_default("video.item_tooltips", false)?
            .set_default("video.fov", 70)?
            .set_default("video.entity_shadows", true)?

            // MSAA
            .set_default("MSAA.enabled", false)?
            .set_default("MSAA.quality", 2)?

            // Input
            .set_default("input.invert_mouse", false)?
            .set_default("input.sensitivity", 1.0)?
            .set_default("input.touch_screen", false)?

            // Gameplay
            .set_default("gameplay.difficulty", 2)?
            .set_default("gameplay.resource_packs", vec!["default"])?
            .set_default("gameplay.direct_connect", "")?
            .set_default("gameplay.lang", "en-us")?
            .set_default("gameplay.right_hand", true)?
            .set_default("gameplay.attack_indicator", true)?
            .set_default("gameplay.old_combat", false)?
            .set_default("gameplay.subtitles", false)?
            .set_default("gameplay.autojump", false)?
            .set_default("gameplay.narrator", false)?
            .set_default("gameplay.tutorial_step", 0)?

            // Chat
            .set_default("chat.enabled", true)?
            .set_default("chat.links", true)?
            .set_default("chat.opacity", 100)?
            
            // Window
            .set_default("window.width", 800)?
            .set_default("window.height", 600)?
            .set_default("window.fullscreen", false)?;

        // Load default config
        s.merge(File::with_name("litecraft").required(false))?;

        s.merge(Environment::with_prefix("APP"))?;

        s.try_into()
    }
}