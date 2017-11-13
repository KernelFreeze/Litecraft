use config::{ConfigError, Config, File, Environment};

#[derive(Debug, Deserialize)]
pub struct Video {
    vsync: bool,
    hiDPI: bool,
    anisotropy: bool,
    renderDistance: u8,
    guiScale: u8,
    particles: u8,
    fancy: bool,
    clouds: bool,
    itemTooltips: bool,
    fov: u8,
    entityShadows: bool,
}

#[derive(Debug, Deserialize)]
pub struct MSAA {
    enabled: bool,
    quality: u8,
}

#[derive(Debug, Deserialize)]
pub struct Input {
    invertYMouse: bool,
    sensitivity: u8,
    touchScreen: bool,
}

#[derive(Debug, Deserialize)]
pub struct Gameplay<'a> {
    difficulty: u8,
    resourcePacks: Vec<&'a str>,
    directConnect: &'a str,
    lang: &'a str,
    rightHand: bool,
    attackIndicator: bool,
    oldCombat: bool,
    subtitles: bool,
    autojump: bool,
    narrator: bool,
    tutorialStep: u8,
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
            .set_default("video.hiDPI", false)?
            .set_default("video.anisotropy", false)?
            .set_default("video.renderDistance", 12)?
            .set_default("video.guiScale", 1)?
            .set_default("video.particles", 2)?
            .set_default("video.fancy", true)?
            .set_default("video.clouds", true)?
            .set_default("video.itemTooltips", false)?
            .set_default("video.fov", 70)?
            .set_default("video.entityShadows", true)?

            // MSAA
            .set_default("MSAA.enabled", false)?
            .set_default("MSAA.quality", 2)?

            // Input
            .set_default("input.invertYMouse", false)?
            .set_default("input.sensitivity", 1.0)?
            .set_default("input.touchScreen", false)?

            // Gameplay
            .set_default("gameplay.difficulty", 2)?
            .set_default("gameplay.resourcePacks", vec!["default"])?
            .set_default("gameplay.directConnect", "")?
            .set_default("gameplay.lang", "en-us")?
            .set_default("gameplay.rightHand", true)?
            .set_default("gameplay.attackIndicator", true)?
            .set_default("gameplay.oldCombat", false)?
            .set_default("gameplay.subtitles", false)?
            .set_default("gameplay.autojump", false)?
            .set_default("gameplay.narrator", false)?
            .set_default("gameplay.tutorialStep", 0)?

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