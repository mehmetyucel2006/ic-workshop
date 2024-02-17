use ic_cdk_macros::*;
use ic_cdk::export::candid::CandidType;

// Kullanıcı verileri
#[derive(CandidType)]
struct User {
    id: u64,            // Kullanıcı kimliği
    address: String,    // Kullanıcının adresi 
    bid: u64,           // Kullanıcının teklifi
}

// Akıllı sözleşme veri yapısı
struct Auction {
    users: Vec<User>,   // İhaleye katılan kullanıcıları saklamak için vektör
}

// Akıllı sözleşme işleyişi
#[ic_cdk_macros::query]
impl Auction {
    // İhaleye katılan bir kullanıcıyı kaydet
    fn register_user(&mut self, address: String, bid: u64) {
        let id = self.users.len() as u64 + 1;
        let user = User { id, address, bid };
        self.users.push(user);
    }

    // En yüksek teklifi veren kullanıcıyı bul
    fn get_highest_bidder(&self) -> Option<&User> {
        self.users.iter().max_by_key(|user| user.bid)
    }

    // En yüksek teklifi alır
    fn get_highest_bid(&self) -> Option<u64> {
        self.get_highest_bidder().map(|user| user.bid)
    }

    // İhale sonucunu belirler
    fn finalize_auction(&self) -> Option<&User> {
        self.get_highest_bidder()
    }
}

#[ic_cdk_macros::init]
fn init() -> Auction {
    Auction { users: Vec::new() }
}
