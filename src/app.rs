use shiplift::builder::ContainerListOptions;
use shiplift::rep::{Container, Info, Version};
use shiplift::Docker;
use tui::layout::Rect;

pub struct App {
    pub docker: Docker,
    pub size: Rect,
    pub version: Version,
    pub info: Info,
    pub containers: Vec<Container>,
    pub selected: usize,
    pub only_running: bool,
    pub tabs: MyTabs,
}

impl App {
    pub fn new() -> App {
        let docker = Docker::new();
        let info = docker.info().unwrap();
        let version = docker.version().unwrap();
        App {
            docker,
            size: Rect::default(),
            version,
            info,
            containers: Vec::new(),
            selected: 0,
            only_running: true,
            tabs: MyTabs::new(),
        }
    }

    pub fn refresh(&mut self) {
        let options = if self.only_running {
            ContainerListOptions::builder().build()
        } else {
            ContainerListOptions::builder().all().build()
        };
        let containers = self.docker.containers().list(&options).unwrap();
        let info = self.docker.info().unwrap();
        self.containers = containers;
        self.info = info;
        if self.selected >= self.containers.len() {
            self.selected = self.containers.len() - 1;
        }
    }

    pub fn get_selected_container(&self) -> Option<&Container> {
        self.containers.get(self.selected)
    }
}

pub struct MyTabs {
    pub titles: Vec<String>,
    pub selected: usize,
}

impl MyTabs {
    pub fn new() -> MyTabs {
        MyTabs {
            titles: vec!["Containers".into(), "Docker".into()],
            selected: 0,
        }
    }

    pub fn next(&mut self) {
        self.selected += 1;
        if self.selected >= self.titles.len() {
            self.selected = 0;
        }
    }

    pub fn previous(&mut self) {
        if self.selected == 0 {
            self.selected = self.titles.len() - 1;
        } else {
            self.selected -= 1;
        }
    }
}
