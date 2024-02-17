use ic_cdk::export::candid::{CandidType, candid};
use serde::{Deserialize, Serialize};

// Kullanıcı veri yapısı
#[derive(CandidType, Deserialize, Serialize)]
pub struct User {
    pub id: u64,
    pub address: String,
    pub bid: u64,
}

// Akıllı sözleşme veri yapısı
pub struct Auction {
    pub users: Vec<User>,
}

impl Auction {
    // İhaleye katılan bir kullanıcıyı kaydetme işlevi
    pub fn register_user(&mut self, address: String, bid: u64) {
        let id = self.users.len() as u64 + 1;
        let user = User { id, address, bid };
        self.users.push(user);
    }

    // Diğer işlevler buraya gelebilir
}

#[ic_cdk_macros::query]
impl Auction {
    // İhale işlevleri
}

#[ic_cdk_macros::init]
fn init() -> Auction {
    Auction { users: Vec::new() }
}
