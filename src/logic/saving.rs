//! support for saving the game
#![allow(non_camel_case_types)]
use std::mem::{self, size_of};

use super::{game::{Game, G_MUTPROC, G_MUTPROC_LOCK}, trains::ObjId};

macro_rules! to_sized {
    ($e:expr) => {
        mem::transmute_copy(&$e)
    };
}

pub struct SaveMeta {
    pub playercount: u16,
    pub companycount: u16,
    pub worldsize: (u16,u16),
    pub routecount: u32,
    pub carcount: u32,
    pub lococount: u32,
    pub traincount: u32,
    pub trackcount: u32,
}
impl SaveMeta {
    fn tobytes(&self) -> [u8;size_of::<Self>()] {
        let mut s = [0;size_of::<Self>()];
        (&mut s[0..2]).copy_from_slice(&self.playercount.to_be_bytes());
        (&mut s[2..4]).copy_from_slice(&self.companycount.to_be_bytes());
        (&mut s[4..6]).copy_from_slice(&self.worldsize.0.to_be_bytes());
        (&mut s[6..8]).copy_from_slice(&self.worldsize.1.to_be_bytes());
        (&mut s[8..12]).copy_from_slice(&self.routecount.to_be_bytes());
        (&mut s[12..16]).copy_from_slice(&self.carcount.to_be_bytes());
        (&mut s[16..20]).copy_from_slice(&self.lococount.to_be_bytes());
        (&mut s[20..24]).copy_from_slice(&self.traincount.to_be_bytes());
        (&mut s[24..28]).copy_from_slice(&self.trackcount.to_be_bytes());
        return s;
    }
    fn frombytes(bytes: &[u8]) -> Self {
        unsafe{Self {
            playercount: u16::from_be_bytes(to_sized!(&bytes[0..2])),
            companycount: u16::from_be_bytes(to_sized!(&bytes[2..4])),
            worldsize: (u16::from_be_bytes(to_sized!(&bytes[4..6])), u16::from_be_bytes(to_sized!(&bytes[6..8]))),
            routecount: u32::from_be_bytes(to_sized!(&bytes[8..12])),
            carcount: u32::from_be_bytes(to_sized!(&bytes[12..16])),
            lococount: u32::from_be_bytes(to_sized!(&bytes[16..20])),
            traincount: u32::from_be_bytes(to_sized!(&bytes[20..24])),
            trackcount: u32::from_be_bytes(to_sized!(&bytes[24..28]))
        }}
    }
}

pub struct RawSave {
}
impl RawSave {
    fn gen_ids(game: &mut Game) -> SaveMeta {
        let mut routeid: ObjId = 0;
        let mut carid: ObjId = 0;
        let mut locoid: ObjId = 0;
        let mut trainid: ObjId = 0;
        let mut trackid: ObjId = 0;
        for tm in &game.tracks {
            let mut t = tm.lock().unwrap();
            trackid += 1;
            t.id = trackid;
        }
        for cm in &game.companies {
            let c = cm.lock().unwrap();
            for rm in &c.routes {
                let mut r = rm.lock().unwrap();
                routeid += 1;
                r.id = routeid;
            }
            for carm in &c.cars {
                let mut car = carm.lock().unwrap();
                carid += 1;
                car.id = carid;
            }
            for locom in &c.locos {
                let mut loco = locom.lock().unwrap();
                locoid += 1;
                loco.id = locoid;
            }
            for tm in &c.trains {
                let mut t = tm.lock().unwrap();
                trainid += 1;
                t.id = trainid;
            }
        }
        return SaveMeta{
            playercount: game.players.len() as u16,
            companycount: game.companies.len() as u16,
            worldsize: (game.world.width,(game.world.grid.len()/game.world.width as usize) as u16),
            routecount: routeid,
            carcount: carid,
            lococount: locoid,
            trackcount: trackid,
            traincount: trainid
        };
    }
    fn pack_players(game: &mut Game) -> Vec<u8> {
        let mut buf = Vec::new();
        for p in &game.players {
            buf.extend_from_slice(&p.id.to_be_bytes());
            buf.extend_from_slice(&p.name.len().to_be_bytes());
            buf.extend_from_slice(p.name.as_bytes());
            buf.extend_from_slice(&p.worth.to_be_bytes());
            buf.extend_from_slice(&p.shares.len().to_be_bytes());
            buf.reserve(p.shares.len()*4);
            for c in &p.shares {
                buf.extend_from_slice(&c.0.to_be_bytes());
                buf.extend_from_slice(&c.1.to_be_bytes());
            }
        }
        return buf;
    }
    pub fn save(game: &mut Game) -> Result<Box<[u8]>, &'static str> {
        if !game.ownssession {
            return Err("not session owner");
        }
        // TODO: make sure there is a way to identify players in netenabled games
        // pause mutable processing
        let _guard = G_MUTPROC_LOCK.lock().unwrap();
        unsafe {G_MUTPROC = false;}
        let meta = Self::gen_ids(game).tobytes();
        let playerbytes = Self::pack_players(game);
        // allow mutable processing to continue
        unsafe {G_MUTPROC = true;}
        let mut buf = Vec::with_capacity(meta.len()+playerbytes.len());
        buf.extend_from_slice(&meta);
        buf.extend_from_slice(&playerbytes);
        todo!();
        return Ok(buf.into_boxed_slice());
    }
    pub fn load(buf: &[u8]) -> Result<Game, &'static str> {
        if buf.len() < 24 {
            return Err("not enough data");
        }
        let meta = SaveMeta::frombytes(buf);
        todo!();
    }
}
