use comfy_wgpu::WgpuRenderer;

use crate::*;

pub trait GameLoop {
    fn performance_metrics(&self, _world: &mut World, _ui: &mut egui::Ui) {}
    fn engine(&mut self) -> &mut EngineState;
    fn update(&mut self);
    fn fixed_update(&mut self);
}

pub type GameLoopBuilder = Box<dyn Fn() -> Arc<Mutex<dyn GameLoop>>>;

pub struct EngineState {
    pub cached_loader: RefCell<CachedImageLoader>,

    pub draw: RefCell<Draw>,
    pub egui: egui::Context,

    pub frame: u64,
    pub flags: RefCell<HashSet<String>>,

    pub dt_stats: MovingStats,
    pub fps_stats: MovingStats,

    pub renderer: Option<WgpuRenderer>,
    pub texture_creator: Option<Arc<AtomicRefCell<WgpuTextureCreator>>>,

    pub lighting: GlobalLightingParams,

    pub meta: AnyMap,

    pub world: Rc<RefCell<World>>,
    pub commands: RefCell<CommandBuffer>,

    pub config: RefCell<GameConfig>,

    pub cooldowns: RefCell<Cooldowns>,
    pub changes: RefCell<ChangeTracker>,
    pub notifications: RefCell<Notifications>,

    pub game_loop: Option<Arc<Mutex<dyn GameLoop>>>,

    pub is_paused: RefCell<bool>,
    pub show_pause_menu: bool,
    pub quit_flag: bool,

    pub to_despawn: RefCell<Vec<Entity>>,

    // Fixed update stuff
    pub accumulator: f32,
    pub previous_time: f32,
}

impl EngineState {
    pub fn new(config: GameConfig) -> Self {
        cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                std::panic::set_hook(Box::new(console_error_panic_hook::hook));
                // console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");
                console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
            } else {
                #[cfg(feature = "ci-release")]
                std::panic::set_hook(Box::new(|info| {
                    error!("Panic: {:?}", info);
                }));

                initialize_logger();
            }
        }

        srand(thread_rng().next_u64());
        set_main_camera_zoom(30.0);

        ASSETS.borrow_mut().load_sound_from_bytes(
            "error",
            include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/error.ogg"
            )),
            StaticSoundSettings::default(),
        );

        Self {
            cached_loader: RefCell::new(CachedImageLoader::new()),
            egui: egui::Context::default(),

            renderer: None,
            texture_creator: None,

            draw: RefCell::new(Draw::new()),

            dt_stats: MovingStats::new(20),
            fps_stats: MovingStats::new(100),

            frame: 0,
            flags: RefCell::new(HashSet::new()),

            meta: AnyMap::new(),

            lighting: config.lighting,

            world: Rc::new(RefCell::new(World::new())),
            commands: RefCell::new(CommandBuffer::new()),

            config: RefCell::new(config),

            cooldowns: RefCell::new(Cooldowns::new()),
            changes: RefCell::new(ChangeTracker::new()),
            notifications: RefCell::new(Notifications::new()),

            game_loop: None,

            is_paused: RefCell::new(false),
            show_pause_menu: false,
            quit_flag: false,

            to_despawn: RefCell::new(vec![]),
            // Fixed timestep stuff
            accumulator: 0.0,
            previous_time: get_time() as f32,
        }
    }

    pub fn on_event(&mut self, event: &WindowEvent) -> bool {
        self.renderer.as_mut().unwrap().on_event(event, &self.egui)
    }

    // #[cfg_attr(feature = "exit-after-startup", allow(unreachable_code))]
    // pub fn update(&mut self) {
    //     if self.game_loop.is_none() {
    //         self.game_loop = Some((self.builder.take().unwrap())());
    //     }
    //
    //     let game_loop = self.game_loop.clone().unwrap();
    //
    //     let mut c = self.make_context();
    //
    //     run_update_stages(&mut *game_loop.lock(), &mut c);
    // }

    pub fn make_context(&mut self) -> EngineContext {
        let renderer = self.renderer.as_mut().unwrap();
        // let egui = renderer.egui_ctx();
        let texture_creator = self.texture_creator.as_ref().unwrap();

        EngineContext {
            cached_loader: &self.cached_loader,
            // graphics_context: &renderer.context,
            // textures: &renderer.textures,
            // surface_config: &renderer.config,
            // render_texture_format: renderer.render_texture_format,
            egui_wants_mouse: self.egui.wants_pointer_input(),
            renderer,

            delta: delta(),

            egui: &self.egui,
            draw: &mut self.draw,
            frame: self.frame,

            dt_stats: &mut self.dt_stats,
            fps_stats: &mut self.fps_stats,

            mouse_world: mouse_world(),

            flags: &mut self.flags,
            lighting: &mut self.lighting,

            meta: &mut self.meta,

            world: &mut self.world,
            commands: &mut self.commands,

            config: &mut self.config,
            game_loop: &mut self.game_loop,

            cooldowns: &mut self.cooldowns,
            changes: &mut self.changes,
            notifications: &mut self.notifications,

            // post_processing_effects: &renderer.post_processing_effects,
            // shaders: &renderer.shaders,
            is_paused: &mut self.is_paused,
            show_pause_menu: &mut self.show_pause_menu,
            quit_flag: &mut self.quit_flag,

            to_despawn: &mut self.to_despawn,

            texture_creator,
        }
    }

    // #[cfg(feature = "tracy")]
    // tracy_client::Client::running()
    //     .expect("client must be running")
    //     .secondary_frame_mark(tracy_client::frame_name!("update"));


    // TODO: this really needs a cleanup
    pub fn renderer(&mut self) -> &mut WgpuRenderer {
        self.renderer.as_mut().expect("renderer must be initialized")
    }

    // TODO: this really needs a cleanup
    pub fn resize(&mut self, new_size: UVec2) {
        self.renderer.as_mut().unwrap().resize(new_size);
    }

    // TODO: this really needs a cleanup
    pub fn quit_flag(&mut self) -> bool {
        self.quit_flag
    }

    // TODO: this really needs a cleanup
    pub fn title(&self) -> String {
        // TODO: make this configurable
        format!("{} (COMFY ENGINE)", self.config.borrow().game_name)
    }
}
