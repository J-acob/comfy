use crate::*;

pub const COMBAT_TEXT_LIFETIME: f32 = 0.4;

#[derive(Copy, Clone, Debug)]
pub enum ResolutionConfig {
    Physical(u32, u32),
    Logical(u32, u32),
}

impl ResolutionConfig {
    pub fn width(&self) -> u32 {
        match self {
            Self::Physical(w, _) => *w,
            Self::Logical(w, _) => *w,
        }
    }

    pub fn height(&self) -> u32 {
        match self {
            Self::Physical(_, h) => *h,
            Self::Logical(_, h) => *h,
        }
    }
}

/// Behavior of the presentation engine based on frame rate.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "trace", derive(Serialize))]
#[cfg_attr(feature = "replay", derive(Deserialize))]
pub enum PresentMode {
    /// Chooses FifoRelaxed -> Fifo based on availability.
    ///
    /// Because of the fallback behavior, it is supported everywhere.
    AutoVsync = 0,
    /// Chooses Immediate -> Mailbox -> Fifo (on web) based on availability.
    ///
    /// Because of the fallback behavior, it is supported everywhere.
    AutoNoVsync = 1,
    /// Presentation frames are kept in a First-In-First-Out queue approximately 3 frames
    /// long. Every vertical blanking period, the presentation engine will pop a frame
    /// off the queue to display. If there is no frame to display, it will present the same
    /// frame again until the next vblank.
    ///
    /// When a present command is executed on the gpu, the presented image is added on the queue.
    ///
    /// No tearing will be observed.
    ///
    /// Calls to get_current_texture will block until there is a spot in the queue.
    ///
    /// Supported on all platforms.
    ///
    /// If you don't know what mode to choose, choose this mode. This is traditionally called "Vsync On".
    #[default]
    Fifo = 2,
    /// Presentation frames are kept in a First-In-First-Out queue approximately 3 frames
    /// long. Every vertical blanking period, the presentation engine will pop a frame
    /// off the queue to display. If there is no frame to display, it will present the
    /// same frame until there is a frame in the queue. The moment there is a frame in the
    /// queue, it will immediately pop the frame off the queue.
    ///
    /// When a present command is executed on the gpu, the presented image is added on the queue.
    ///
    /// Tearing will be observed if frames last more than one vblank as the front buffer.
    ///
    /// Calls to get_current_texture will block until there is a spot in the queue.
    ///
    /// Supported on AMD on Vulkan.
    ///
    /// This is traditionally called "Adaptive Vsync"
    FifoRelaxed = 3,
    /// Presentation frames are not queued at all. The moment a present command
    /// is executed on the GPU, the presented image is swapped onto the front buffer
    /// immediately.
    ///
    /// Tearing can be observed.
    ///
    /// Supported on most platforms except older DX12 and Wayland.
    ///
    /// This is traditionally called "Vsync Off".
    Immediate = 4,
    /// Presentation frames are kept in a single-frame queue. Every vertical blanking period,
    /// the presentation engine will pop a frame from the queue. If there is no frame to display,
    /// it will present the same frame again until the next vblank.
    ///
    /// When a present command is executed on the gpu, the frame will be put into the queue.
    /// If there was already a frame in the queue, the new frame will _replace_ the old frame
    /// on the queue.
    ///
    /// No tearing will be observed.
    ///
    /// Supported on DX11/12 on Windows 10, NVidia on Vulkan and Wayland on Vulkan.
    ///
    /// This is traditionally called "Fast Vsync"
    Mailbox = 5,
}

#[derie(Copy, Clone, Debug)]
pub struct RenderConfig {
    target_framerate: f32,
    present_mode: PresentMode,
}

#[derive(Copy, Clone, Debug)]
pub struct RenderConfig {
    pub target_framerate: f32,
    pub present_mode: i32, 
}

#[derive(Copy, Clone, Debug)]
pub struct GameConfig {
    pub game_name: &'static str,
    pub version: &'static str,

    pub resolution: ResolutionConfig,

    pub bloom_enabled: bool,
    pub lighting: GlobalLightingParams,
    pub lighting_enabled: bool,

    pub enable_dynamic_camera: bool,

    pub dev: DevConfig,

    pub scroll_speed: f32,

    pub music_enabled: bool,

    pub show_combat_text: bool,
    pub spawn_exp: bool,

    pub render_config: RenderConfig,
}

impl Default for GameConfig {
    fn default() -> Self {
        #[cfg(target_arch = "wasm32")]
        let resolution = ResolutionConfig::Logical(1106, 526);
        #[cfg(not(target_arch = "wasm32"))]
        let resolution = ResolutionConfig::Physical(1920, 1080);

        Self {
            game_name: "TODO_GAME_NAME",
            version: "TODO_VERSION",

            resolution,

            bloom_enabled: false,
            lighting: GlobalLightingParams::default(),
            lighting_enabled: false,

            dev: DevConfig::default(),

            enable_dynamic_camera: false,

            scroll_speed: 7.0,
            music_enabled: false,

            show_combat_text: true,
            spawn_exp: true,
            render_config: RenderConfig { target_framerate: 60., present_mode: PresentMode::AutoNoVsync }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct DevConfig {
    pub show_lighting_config: bool,
    pub show_buffers: bool,
    pub show_fps: bool,
    pub show_editor: bool,

    pub show_tiktok_overlay: bool,

    pub log_collisions: bool,

    pub show_ai_target: bool,
    pub show_linear_acc_target: bool,
    pub show_angular_acc_target: bool,

    pub draw_colliders: bool,
    pub draw_collision_marks: bool,

    pub show_debug_bullets: bool,

    pub orig_props: bool,

    pub collider_outlines: bool,

    pub show_debug: bool,

    pub recording_mode: RecordingMode,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RecordingMode {
    None,
    Tiktok,
    Landscape,
}

impl Default for DevConfig {
    fn default() -> Self {
        Self {
            show_lighting_config: false,
            show_buffers: false,
            show_editor: false,

            log_collisions: false,

            show_ai_target: false,
            show_linear_acc_target: false,
            show_angular_acc_target: false,
            show_tiktok_overlay: false,

            show_debug_bullets: false,

            #[cfg(feature = "ci-release")]
            show_fps: false,
            #[cfg(not(feature = "ci-release"))]
            show_fps: true,

            draw_colliders: false,
            draw_collision_marks: false,

            collider_outlines: false,
            orig_props: true,
            show_debug: false,

            recording_mode: RecordingMode::Landscape,
        }
    }
}
