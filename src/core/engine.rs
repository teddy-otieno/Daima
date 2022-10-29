use std::{
    thread::{self, JoinHandle, Thread},
    vec, time::Duration,
};

use super::system::System;
use crossbeam_channel::{bounded, Receiver, Sender};
use sysinfo::{System as HardWareSystem, SystemExt};

#[derive(Clone, Copy)]
enum GameStateEvent {}

#[derive(Debug, Clone, Copy)]
pub enum SystemEvent {}

struct Worker {
    join_handle: JoinHandle<()>,
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
    game_tick_channel: (Sender<usize>, Receiver<usize>),
    game_state_channel: (Sender<Vec<GameStateEvent>>, Receiver<Vec<GameStateEvent>>),
    system_events_channel: (Sender<Vec<SystemEvent>>, Receiver<Vec<SystemEvent>>),
    thread_count: usize,
    systems: Vec<System>,
    workers: Vec<Worker>,
}

impl SystemManager {
    fn new(thread_count: usize, systems: Vec<System>) -> Self {
        let size_of_sys_event_channel = if systems.len() < thread_count {
            1
        } else {
            systems.len() / thread_count
        };

        Self {
            tick_broadcast_bus: bus::Bus::new(1),
            game_state_broadcast_bus: bus::Bus::new(1),
            game_tick_channel: bounded(0),
            game_state_channel: bounded(1),
            system_events_channel: bounded(10),
            thread_count,
            systems,
            workers: vec![],
        }
    }

    fn events_channel_size(&self) -> usize {
        if self.systems.len() < self.thread_count {
            1
        } else {
            self.systems.len() / self.thread_count
        }
    }
}

pub struct Engine {
    systems_manager: SystemManager,
    previous_tick: usize,
}

/*
    Rendering will have its own thread. Figure out how to create a seperate opengl context from the one
    used by the main thread.
*/

impl Engine {
    /*Send step event to the systems
     *Receive systems events.
     *Broadcast to other system
     *Update entit's components
     */
    pub fn update(&mut self) {
        self.systems_manager.tick_broadcast_bus.broadcast(16);
        self.systems_manager
            .game_state_broadcast_bus
            .broadcast(vec![]);

        let mut events = vec![];

        let mut counter = 0;
        while let Ok(new_events) = self.systems_manager.system_events_channel.1.recv_timeout(Duration::from_millis(16)) {
            events.extend(new_events);
            counter += 1;

            if counter == self.systems_manager.events_channel_size() {
                break;
            }
        }


    }

    fn setup_systems(&mut self) {
        //Divide the systems across the available systems threads
        //Give the render system thread its own dedicated thread
        let systems_per_thread =
            if self.systems_manager.systems.len() > self.systems_manager.thread_count {
                self.systems_manager.systems.len() / self.systems_manager.thread_count
            } else {
                self.systems_manager.systems.len()
            };

        for (i ,systems) in self.systems_manager.systems.chunks(systems_per_thread).enumerate() {
            let mut systems = systems.to_owned();
            let system_event_sender = self.systems_manager.system_events_channel.0.clone();

            let mut game_state_receiver = self.systems_manager.game_state_broadcast_bus.add_rx();
            let mut game_tick_receiver = self.systems_manager.tick_broadcast_bus.add_rx();

            let worker = Worker {
                join_handle: thread::spawn(move || {
                    //Thread is crushing for some unknown reason
                    //Invalid writes to a memory address.
                    //The crash only appears when the debugger is active

                    let mut event_buffer: Vec<SystemEvent> = vec![];
                    loop {
                        let tick_time = game_tick_receiver.recv().unwrap();
                        println!("Updated: Thread {}", i);
                        let _game_state_events = game_state_receiver.recv().unwrap();

                        for system in &mut systems {
                            match system.update(tick_time) {
                                Ok(events) => event_buffer.extend(events),
                                Err(_error) => {
                                    //TODO: Log the errors
                                }
                            }
                        }

                        //NOTE: (teddy) this is a temporary fix
                        //Should invenstigate the blocking problem 
                        if let Ok(_) =  system_event_sender.send_timeout(event_buffer.clone(), Duration::from_millis(16)) {
                            println!("Sending was successful");
                            event_buffer.clear();
                        }                     
                    }
                }),
            };

            self.systems_manager.workers.push(worker);
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
            systems_manager: SystemManager::new(sys.cpus().len(), self.systems),
            previous_tick: 0,
        };
        engine.setup_systems();
        engine
    }
}
