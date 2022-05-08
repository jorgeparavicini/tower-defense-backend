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
    pub fn new(mut host: Client) -> Self {
        assert!(host.is_host());
        Self {
            host,
            clients: vec![],
        }
    }

    pub fn add_client(&mut self, mut client: Client) {
        client.set_is_host(false);
        self.clients.push(client);
        debug!("Added client to player list");
    }
}

impl<'a> Players {
    fn iter_mut(&'a mut self) -> PlayersMutIterator<'a> {
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

struct PlayersIterator<'a> {
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

struct PlayersMutIterator<'a> {
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
