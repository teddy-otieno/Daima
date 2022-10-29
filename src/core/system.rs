use super::engine::SystemEvent;

type SysResult<T> = Result<T, SystemError>;

pub trait SystemTrait {
    fn step(&mut self, time: usize) -> SysResult<Vec<SystemEvent>>;
}

#[derive(Clone, Debug)]
pub struct SampleSystem {
    pub(crate) name: String,
}
impl SystemTrait for SampleSystem {
    fn step(&mut self, time: usize) -> SysResult<Vec<SystemEvent>> {
        println!("{:?} stepped", self);
        Ok(vec![])
    }
}

/// Static dispatch for systems

#[derive(Debug, Clone)]
pub struct SystemError {
    name: String,
    description: String,
}

#[derive(Clone, Debug)]
pub enum System {
    SampleSystem(SampleSystem),
}

impl System {
    pub fn update(&mut self, time: usize) -> Result<Vec<SystemEvent>, SystemError> {
        match self {
            System::SampleSystem(sys) => sys.step(time),
        }
    }
}
