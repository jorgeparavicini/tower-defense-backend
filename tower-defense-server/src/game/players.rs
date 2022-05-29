use crate::game::Client;
use log::debug;

pub struct Players {
    host: Client,
    clients: Vec<Client>,
}

trait IntoMutIterator {
    type Item;
    type IntoIter;
}

impl Players {
    pub fn new(host: Client) -> Self {
        assert!(host.is_host());
        Self {
            host,
            clients: vec![],
        }
    }

    pub fn add_client(&mut self, client: Client) {
        self.clients.push(client);
        debug!("Added client to player list");
    }

    pub fn remove_client(&mut self, name: &str) {
        debug_assert!(self.host.get_name() != name);
        self.clients.retain(|client| client.get_name() != name);
    }

    pub fn find_client(&self, name: &str) -> Option<&Client> {
        if self.host.get_name() == name {
            return Some(&self.host);
        }

        self.clients.iter().find(|client| client.get_name() == name)
    }

    pub fn get_host(&self) -> &Client {
        &self.host
    }
}

impl<'a> Players {
    pub(crate) fn iter_mut(&'a mut self) -> PlayersMutIterator<'a> {
        PlayersMutIterator {
            players: self,
            index: 0,
        }
    }
}

impl<'a> IntoIterator for &'a Players {
    type Item = &'a Client;
    type IntoIter = PlayersIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PlayersIterator {
            players: self,
            index: 0,
        }
    }
}

pub struct PlayersIterator<'a> {
    players: &'a Players,
    index: usize,
}

impl<'a> Iterator for PlayersIterator<'a> {
    type Item = &'a Client;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => Some(&self.players.host),
            _ => self.players.clients.get(self.index - 1),
        };
        self.index += 1;
        result
    }
}

pub struct PlayersMutIterator<'a> {
    players: &'a mut Players,
    index: usize,
}

impl<'a> Iterator for PlayersMutIterator<'a> {
    type Item = &'a mut Client;

    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.index == 0 {
            let ptr = &mut self.players.host as *mut Client;
            unsafe { Some(&mut *ptr) }
        } else if self.index <= self.players.clients.len() {
            Some(unsafe {
                &mut *self
                    .players
                    .clients
                    .as_mut_ptr()
                    .offset((self.index - 1) as isize)
            })
        } else {
            None
        };

        self.index += 1;
        result
    }
}
