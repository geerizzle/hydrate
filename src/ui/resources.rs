use iced::widget::svg::Handle;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SvgId {
    Add,
    Home,
    Hydrate,
}

#[derive(Debug)]
pub struct ResourceManager {
    svgs: HashMap<SvgId, Handle>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self { svgs: HashMap::new() }
    }

    pub fn load_all(&mut self) {
        for id in SvgId::ALL {
            self.svgs.insert(*id, Handle::from_memory(Self::bytes(*id)));
        }
    }
    pub fn svg(&self, id: SvgId) -> Handle {
        Handle::from_memory(Self::bytes(id))
    }

    fn bytes(id: SvgId) -> &'static [u8] {
        match id {
            SvgId::Add => include_bytes!("../../resources/svg/add.svg"),
            SvgId::Home => include_bytes!("../../resources/svg/home.svg"),
            SvgId::Hydrate => include_bytes!("../../resources/svg/hydrate.svg"),
        }
    }
}

impl SvgId {
    const ALL: &[SvgId] = &[SvgId::Add, SvgId::Home, SvgId::Hydrate];
}
