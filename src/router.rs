#[derive(Clone, Debug, PartialEq)]
pub enum Location {
    Home,
    NodesList,
    Node(String, NodeSubLocation),
    Simulation,
    Exploits,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NodeSubLocation {
    ActionMenu,
    ConnectPeer,
    ListChannels,
    NewAddress,
}

#[derive(Debug, Clone)]
pub enum Action {
    Push(Location),
    Replace(Location),
    Pop,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ActiveBlock {
    NoneBlock,
    Menu,
    Nodes,
    Main(Location),
}

#[derive(Clone, Debug, PartialEq)]
pub enum HoveredBlock {
    Menu,
    Nodes,
}

pub struct Router {
    screen_stack: Vec<Location>,
    active_route: Location,
    active_block: ActiveBlock,
    hovered_block: HoveredBlock,
}

impl Router {
    pub fn new() -> Self {
        let screen_stack = vec![Location::Home];
        Self {
            screen_stack,
            active_route: Location::Home,
            active_block: ActiveBlock::Menu,
            hovered_block: HoveredBlock::Menu,
        }
    }

    pub fn go_to(&mut self, action: Action) {
        let (next_route, next_block) = match action {
            Action::Push(location) => {
                self.screen_stack.push(location.clone());

                (location.clone(), location_to_active_block(location.clone()))
            }
            Action::Replace(location) => {
                // if menu item, don't replace route, just the active block
                let next_route = match location {
                    Location::Home => self.screen_stack[self.screen_stack.len() - 1].clone(),
                    Location::NodesList => self.screen_stack[self.screen_stack.len() - 1].clone(),
                    _ => location.clone(),
                };

                (next_route, location_to_active_block(location.clone()))
            }
            Action::Pop => {
                let next_location = match self.screen_stack.pop() {
                    Some(_) => {
                        if self.screen_stack.len() == 0 {
                            self.screen_stack.push(Location::Home);
                            Location::Home
                        } else {
                            self.screen_stack[self.screen_stack.len() - 1].clone()
                        }
                    }
                    None => {
                        self.screen_stack.push(Location::Home);
                        Location::Home
                    }
                };

                (
                    next_location.clone(),
                    location_to_active_block(next_location.clone()),
                )
            }
        };

        self.active_route = next_route;
        self.active_block = next_block
    }

    pub fn get_current_route(&self) -> &Location {
        &self.active_route
    }

    pub fn get_stack(&self) -> &Vec<Location> {
        &self.screen_stack
    }

    pub fn get_active_block(&self) -> &ActiveBlock {
        &self.active_block
    }
}

pub fn location_to_active_block(loc: Location) -> ActiveBlock {
    match loc {
        Location::Node(n, s) => ActiveBlock::Main(Location::Node(n, s)),
        Location::Simulation => ActiveBlock::Main(Location::Simulation),
        Location::Home => ActiveBlock::Menu,
        Location::NodesList => ActiveBlock::Nodes,
        Location::Exploits => ActiveBlock::Main(Location::Exploits),
        _ => ActiveBlock::NoneBlock,
    }
}
