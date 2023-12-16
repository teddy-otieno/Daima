use glfw::WindowEvent;
use itertools::Itertools;
use std::{
    sync::{Arc, RwLock},
    thread::{self, yield_now, JoinHandle},
    time::Duration,
    usize, vec,
};

use super::{camera::Camera, components::SampleComponent, system::System};
use crossbeam_channel::{bounded, Receiver, Sender};
use sysinfo::{System as HardWareSystem, SystemExt};

const TOTAL_ENTITIES: usize = 1_000;

#[derive(Debug, Clone)]
pub enum GameStateEvent {
    InputEvent(WindowEvent),
}

#[derive(Debug, Clone)]
pub enum SystemEvent {
    ShutdownEngine,
    EngineEvent(GameStateEvent),
    AssetSystemEvent,
}

struct Worker {
    _join_handle: JoinHandle<()>,
}

/**
 * @game_state_channel -> Each worker thread will subscribe to this to game state changes
 * @system_events_channel -> Events received from the systems. Events will be broadcasted to all the systems
 * @game_tick_channel -> Send tick event together with the time that passes
 *
 */
struct SystemManager {
    tick_broadcast_bus: bus::Bus<usize>,
    game_state_broadcast_bus: bus::Bus<Vec<GameStateEvent>>,
    system_events_channel: (Sender<Vec<SystemEvent>>, Receiver<Vec<SystemEvent>>),
    thread_count: usize,
    no_of_systems: Option<usize>,
    workers: Vec<Worker>,
}

impl SystemManager {
    fn new(thread_count: usize) -> Self {
        Self {
            tick_broadcast_bus: bus::Bus::new(1),
            game_state_broadcast_bus: bus::Bus::new(1),
            system_events_channel: bounded(10),
            thread_count,
            no_of_systems: None,
            workers: vec![],
        }
    }
}

struct EntityID {
    id: usize,
    gen: usize,
}

impl EntityID {}

pub struct EntityManager {
    entities: Vec<EntityID>,
    render_components: Vec<SampleComponent>,
}

impl EntityManager {
    fn new() -> Self {
        Self {
            entities: Vec::with_capacity(TOTAL_ENTITIES),
            render_components: Vec::with_capacity(TOTAL_ENTITIES),
        }
    }
}

pub type EntityManagerRef = Arc<RwLock<EntityManager>>;

//TODO: (Teddy) Add detlta time parameter

#[derive(Clone)]
pub struct Engine {
    systems_manager: Arc<RwLock<SystemManager>>,
    previous_tick: usize,
    entity_manager: EntityManagerRef,
    pub camera: Arc<RwLock<Camera>>,
}

/*
    Rendering will have its own thread. Figure out how to create a seperate opengl context from the one
    used by the main thread.

    TODO: (teddy) I'll need a channel for engine level events, this channel will only be used by the rendering for now
*/

impl Engine {
    /*Send step event to the systems
     *Receive systems events.
     *Broadcast to other system
     *Update entit's components
     */
    pub fn update(&mut self) {
        let systems_manager = self.systems_manager.clone();
        let mut systems_manager_lock = systems_manager.write().unwrap();

        systems_manager_lock.tick_broadcast_bus.broadcast(16);
        systems_manager_lock
            .game_state_broadcast_bus
            .broadcast(vec![]);

        let mut events = vec![];

        let mut counter = 0;
        while let Ok(new_events) = systems_manager_lock
            .system_events_channel
            .1
            .recv_timeout(Duration::from_millis(16))
        {
            events.extend(new_events);
            counter += 1;

            let events_size = Self::events_channel_size(
                systems_manager_lock.no_of_systems.unwrap(),
                systems_manager_lock.thread_count,
            );

            if counter == events_size {
                break;
            }
        }

        //dbg!(&events);

        for event in events.iter() {
            match event {
                SystemEvent::ShutdownEngine => {
                    //Close the engine. Implement a clean up function
                    panic!("Closed");
                }
                SystemEvent::EngineEvent(GameStateEvent::InputEvent(e)) => {
                    self.update_camera_movements(e);
                }
                //Check camera movement and update_camera
                e => continue,
            }
        }
    }

    pub fn update_camera_movements(&mut self, e: &WindowEvent) {
        let camera = self.camera.clone();
        let camera_lock = camera.read().unwrap();

        let mut new_camera_pos = camera_lock.pos;

        match e {
            WindowEvent::Key(glfw::Key::Up, _, _, _) => new_camera_pos += camera_lock.front * 0.1,
            WindowEvent::Key(glfw::Key::Down, _, _, _) => new_camera_pos -= camera_lock.front * 0.1,
            WindowEvent::Key(glfw::Key::Left, _, _, _) => {
                new_camera_pos -= camera_lock.front.cross(&camera_lock.up).normalize() * 0.1;
            }
            WindowEvent::Key(glfw::Key::Right, _, _, _) => {
                new_camera_pos += camera_lock.front.cross(&camera_lock.up).normalize() * 0.1;
            }
            _ => (),
        }
        drop(camera_lock);

        let mut camera_lock = camera.write().unwrap();
        camera_lock.pos = new_camera_pos;
    }

    #[inline]
    fn events_channel_size(no_of_systems: usize, thread_count: usize) -> usize {
        if no_of_systems > thread_count {
            no_of_systems / thread_count
        } else {
            no_of_systems
        }
    }

    pub fn get_events() {
        // self
    }

    fn setup_systems(&mut self, systems: Vec<System>) {
        //Divide the systems across the available systems threads
        //Give the render system thread its own dedicated thread
        let systems_manager = self.systems_manager.clone();
        let mut systems_manager_lock = systems_manager.write().unwrap();

        let size_of_systems = systems.len();
        systems_manager_lock.no_of_systems = Some(size_of_systems);

        for (i, mut systems) in systems
            .into_iter()
            .chunks(Self::events_channel_size(
                size_of_systems,
                systems_manager_lock.thread_count,
            ))
            .into_iter()
            .map(|chunk| chunk.collect::<Vec<System>>())
            .enumerate()
        {
            let system_event_sender = systems_manager_lock.system_events_channel.0.clone();

            let mut game_state_receiver = systems_manager_lock.game_state_broadcast_bus.add_rx();
            let mut game_tick_receiver = systems_manager_lock.tick_broadcast_bus.add_rx();

            //Thread is crushing for some unknown reason
            //Invalid writes to a memory address.
            //The crash only appears when the debugger is active

            let entities_ref = self.entity_manager.clone();

            let engine_ref = self.clone();

            let worker_handle = thread::spawn(move || {
                for system in systems.iter_mut() {
                    system.init();
                }

                let mut event_buffer: Vec<SystemEvent> = vec![];
                loop {
                    let tick_time = game_tick_receiver.recv().unwrap();
                    //println!("Updated: Thread {}", i);
                    let _game_state_events = game_state_receiver.recv().unwrap();

                    for system in systems.iter_mut() {
                        match system.update(tick_time, &entities_ref, &engine_ref) {
                            Ok(events) => event_buffer.extend(events),
                            Err(_error) => {
                                //TODO: Log the errors
                            }
                        }
                    }

                    //NOTE: (teddy) this is a temporary fix
                    //Should invenstigate the blocking problem
                    if let Ok(_) = system_event_sender
                        .send_timeout(event_buffer.clone(), Duration::from_millis(16))
                    {
                        event_buffer.clear();
                    }
                }
            });

            let worker = Worker {
                _join_handle: worker_handle,
            };

            systems_manager_lock.workers.push(worker);
        }
    }
}

pub struct EngineBuilder {
    systems: Vec<System>,
}

impl EngineBuilder {
    pub fn builder() -> Self {
        Self { systems: vec![] }
    }

    pub fn add_system(mut self, system: System) -> Self {
        self.systems.push(system);
        self
    }

    pub fn build(self) -> Engine {
        //TODO: (teddy) bind event
        //Get the thread count from operating system
        let mut sys = HardWareSystem::new_all();
        sys.refresh_cpu();
        let mut engine = Engine {
            systems_manager: Arc::new(RwLock::new(SystemManager::new(sys.cpus().len()))),
            previous_tick: 0,
            entity_manager: Arc::new(RwLock::new(EntityManager::new())),
            camera: Arc::new(RwLock::new(Camera::new())),
        };
        let temp = self.systems;
        engine.setup_systems(temp);
        engine
    }
}
