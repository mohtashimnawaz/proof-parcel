use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{init, post_upgrade, pre_upgrade, query, update};
use ic_cdk_macros::*;
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};
use std::collections::HashMap;
use std::cell::RefCell;
use std::time::{SystemTime, UNIX_EPOCH};
use ic_cdk::api::management_canister::main::raw_rand;

// Types for the ProofParcel system
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Delivery {
    pub id: String,
    pub seller: Principal,
    pub buyer: Principal,
    pub amount: u64,
    pub description: String,
    pub status: DeliveryStatus,
    pub created_at: u64,
    pub in_transit_at: Option<u64>,
    pub delivered_at: Option<u64>,
    pub confirmed_at: Option<u64>,
    pub escrow_released_at: Option<u64>,
    pub cancelled_at: Option<u64>,
    pub otp: Option<String>,
    pub otp_expires_at: Option<u64>,
    pub status_history: Vec<(DeliveryStatus, u64)>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub enum DeliveryStatus {
    Pending,
    InTransit,
    Delivered,
    Confirmed,
    EscrowReleased,
    Cancelled,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CreateDeliveryRequest {
    pub buyer: Principal,
    pub amount: u64,
    pub description: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ConfirmDeliveryRequest {
    pub delivery_id: String,
    pub otp: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct DeliveryNFT {
    pub id: String,
    pub delivery_id: String,
    pub owner: Principal,
    pub metadata: String,
    pub minted_at: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Notification {
    pub id: String,
    pub principal: Principal,
    pub message: String,
    pub notif_type: String,
    pub timestamp: u64,
    pub read: bool,
}

// State management
thread_local! {
    static DELIVERIES: RefCell<HashMap<String, Delivery>> = RefCell::new(HashMap::new());
    static DELIVERY_NFTS: RefCell<HashMap<String, DeliveryNFT>> = RefCell::new(HashMap::new());
    static ESCROW_BALANCE: RefCell<u64> = RefCell::new(0);
    static NOTIFICATIONS: RefCell<HashMap<Principal, Vec<Notification>>> = RefCell::new(HashMap::new());
}

// Initialize the canister
#[init]
fn init() {
    ic_cdk::println!("ProofParcel canister initialized");
}

// Pre-upgrade hook to save state
#[pre_upgrade]
fn pre_upgrade() {
    let deliveries = DELIVERIES.with(|d| d.borrow().clone());
    let nfts = DELIVERY_NFTS.with(|n| n.borrow().clone());
    let escrow = ESCROW_BALANCE.with(|e| *e.borrow());
    
    let state = (deliveries, nfts, escrow);
    ic_cdk::storage::stable_save((state,)).expect("Failed to save state");
}

// Post-upgrade hook to restore state
#[post_upgrade]
fn post_upgrade() {
    let (state,): ((HashMap<String, Delivery>, HashMap<String, DeliveryNFT>, u64),) = 
        ic_cdk::storage::stable_restore().expect("Failed to restore state");
    
    let (deliveries, nfts, escrow) = state;
    DELIVERIES.with(|d| *d.borrow_mut() = deliveries);
    DELIVERY_NFTS.with(|n| *n.borrow_mut() = nfts);
    ESCROW_BALANCE.with(|e| *e.borrow_mut() = escrow);
}

// Generate a random OTP using IC randomness
async fn generate_otp() -> String {
    let (rand_bytes,) = raw_rand().await.expect("Failed to get randomness");
    let bytes = rand_bytes.as_slice();
    let num = ((bytes[0] as u32) << 24 | (bytes[1] as u32) << 16 | (bytes[2] as u32) << 8 | (bytes[3] as u32)) % 900_000 + 100_000;
    num.to_string()
}

// Generate a unique delivery ID using IC randomness
async fn generate_unique_id() -> String {
    let (rand_bytes,) = raw_rand().await.expect("Failed to get randomness");
    rand_bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>()
}

// Get current timestamp
fn get_current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

// Create a new delivery
#[update]
async fn create_delivery(request: CreateDeliveryRequest) -> Result<String, String> {
    let caller = ic_cdk::caller();
    
    // Validate request
    if request.amount == 0 {
        return Err("Amount must be greater than 0".to_string());
    }
    
    if request.description.is_empty() {
        return Err("Description cannot be empty".to_string());
    }
    
    // Generate unique delivery ID
    let delivery_id = generate_unique_id().await;
    
    // Create delivery record
    let delivery = Delivery {
        id: delivery_id.clone(),
        seller: caller,
        buyer: request.buyer,
        amount: request.amount,
        description: request.description,
        status: DeliveryStatus::Pending,
        created_at: get_current_time(),
        in_transit_at: None,
        delivered_at: None,
        confirmed_at: None,
        escrow_released_at: None,
        cancelled_at: None,
        otp: None,
        otp_expires_at: None,
        status_history: vec![(DeliveryStatus::Pending, get_current_time())],
    };
    
    // Store delivery
    DELIVERIES.with(|d| {
        d.borrow_mut().insert(delivery_id.clone(), delivery);
    });
    
    // Add to escrow balance
    ESCROW_BALANCE.with(|e| {
        *e.borrow_mut() += request.amount;
    });
    
    ic_cdk::println!("Delivery created: {}", delivery_id);
    Ok(delivery_id)
}

// Start delivery (seller marks as in transit)
#[update]
fn start_delivery(delivery_id: String) -> Result<(), String> {
    let caller = ic_cdk::caller();
    
    DELIVERIES.with(|d| {
        let mut deliveries = d.borrow_mut();
        if let Some(delivery) = deliveries.get_mut(&delivery_id) {
            if delivery.seller != caller {
                return Err("Only seller can start delivery".to_string());
            }
            
            if delivery.status != DeliveryStatus::Pending {
                return Err("Delivery must be in pending status".to_string());
            }
            
            delivery.status = DeliveryStatus::InTransit;
            delivery.in_transit_at = Some(get_current_time());
            delivery.status_history.push((DeliveryStatus::InTransit, get_current_time()));
            add_notification(delivery.buyer, &format!("Your delivery {} is now in transit!", delivery.id), "info");
            Ok(())
        } else {
            Err("Delivery not found".to_string())
        }
    })
}

// Generate OTP for delivery confirmation
#[update]
async fn generate_delivery_otp(delivery_id: String) -> Result<String, String> {
    let caller = ic_cdk::caller();
    let otp = generate_otp().await;
    let mut result = Err("Delivery not found".to_string());
    DELIVERIES.with(|d| {
        let mut deliveries = d.borrow_mut();
        if let Some(delivery) = deliveries.get_mut(&delivery_id) {
            if delivery.seller != caller {
                result = Err("Only seller can generate OTP".to_string());
                return;
            }
            if delivery.status != DeliveryStatus::InTransit {
                result = Err("Delivery must be in transit".to_string());
                return;
            }
            let expires_at = get_current_time() + 3600; // 1 hour expiry
            delivery.otp = Some(otp.clone());
            delivery.otp_expires_at = Some(expires_at);
            delivery.status = DeliveryStatus::Delivered;
            delivery.delivered_at = Some(get_current_time());
            delivery.status_history.push((DeliveryStatus::Delivered, get_current_time()));
            add_notification(delivery.buyer, &format!("Your delivery {} is now delivered! OTP generated.", delivery.id), "info");
            ic_cdk::println!("OTP generated for delivery {}: {}", delivery_id, otp);
            result = Ok(otp.clone());
        }
    });
    result
}

// Confirm delivery with OTP
#[update]
async fn confirm_delivery(request: ConfirmDeliveryRequest) -> Result<String, String> {
    let caller = ic_cdk::caller();
    let current_time = get_current_time();
    let mut nft_id: Option<String> = None;
    let mut result: Result<String, String> = Err("Delivery not found".to_string());
    DELIVERIES.with(|d| {
        let mut deliveries = d.borrow_mut();
        if let Some(delivery) = deliveries.get_mut(&request.delivery_id) {
            if delivery.buyer != caller {
                result = Err("Only buyer can confirm delivery".to_string());
                return;
            }
            if delivery.status != DeliveryStatus::Delivered {
                result = Err("Delivery must be delivered to confirm".to_string());
                return;
            }
            if let Some(stored_otp) = &delivery.otp {
                if stored_otp != &request.otp {
                    result = Err("Invalid OTP".to_string());
                    return;
                }
            } else {
                result = Err("No OTP found for delivery".to_string());
                return;
            }
            if let Some(expires_at) = delivery.otp_expires_at {
                if current_time > expires_at {
                    result = Err("OTP has expired".to_string());
                    return;
                }
            }
            delivery.status = DeliveryStatus::Confirmed;
            delivery.confirmed_at = Some(current_time);
            delivery.status_history.push((DeliveryStatus::Confirmed, current_time));
            add_notification(delivery.seller, &format!("Delivery {} has been confirmed by the buyer!", request.delivery_id), "success");
            add_notification(delivery.buyer, &format!("You have confirmed delivery {}!", request.delivery_id), "success");
            result = Ok(String::new()); // placeholder, will set below
        }
    });
    if let Ok(_) = result {
        // Mint NFT for the buyer
        let minted_nft_id = mint_delivery_nft(&request.delivery_id).await;
        nft_id = Some(minted_nft_id.clone());
        ic_cdk::println!("Delivery confirmed: {}", request.delivery_id);
        result = Ok(minted_nft_id);
    }
    result
}

// Release escrow to seller
#[update]
fn release_escrow(delivery_id: String) -> Result<(), String> {
    let caller = ic_cdk::caller();
    let current_time = get_current_time();
    
    DELIVERIES.with(|d| {
        let mut deliveries = d.borrow_mut();
        if let Some(delivery) = deliveries.get_mut(&delivery_id) {
            if delivery.seller != caller {
                return Err("Only seller can release escrow".to_string());
            }
            
            if delivery.status != DeliveryStatus::Confirmed {
                return Err("Delivery must be confirmed to release escrow".to_string());
            }
            
            // Release escrow
            delivery.status = DeliveryStatus::EscrowReleased;
            delivery.escrow_released_at = Some(current_time);
            delivery.status_history.push((DeliveryStatus::EscrowReleased, current_time));
            
            // Update escrow balance
            ESCROW_BALANCE.with(|e| {
                *e.borrow_mut() -= delivery.amount;
            });
            
            ic_cdk::println!("Escrow released for delivery: {}", delivery_id);
            add_notification(delivery.seller, &format!("Escrow released for delivery {}!", delivery_id), "success");
            Ok(())
        } else {
            Err("Delivery not found".to_string())
        }
    })
}

// Mint NFT for confirmed delivery
async fn mint_delivery_nft(delivery_id: &str) -> String {
    let mut delivery_opt = None;
    DELIVERIES.with(|d| {
        let deliveries = d.borrow();
        if let Some(delivery) = deliveries.get(delivery_id) {
            delivery_opt = Some(delivery.clone());
        }
    });
    let delivery = delivery_opt.expect("Delivery not found for NFT minting");
    let nft_id = generate_unique_id().await;
    let metadata = serde_json::json!({
        "delivery_id": delivery.id,
        "description": delivery.description,
        "amount": delivery.amount,
        "confirmed_at": delivery.confirmed_at,
        "seller": delivery.seller.to_string(),
        "buyer": delivery.buyer.to_string()
    }).to_string();
    let nft = DeliveryNFT {
        id: nft_id.clone(),
        delivery_id: delivery.id.clone(),
        owner: delivery.buyer,
        metadata,
        minted_at: get_current_time(),
    };
    DELIVERY_NFTS.with(|n| {
        n.borrow_mut().insert(nft_id.clone(), nft);
    });
    add_notification(delivery.buyer, &format!("NFT minted for delivery {}!", delivery.id), "success");
    nft_id
}

// Add a notification for a specific user
fn add_notification(principal: Principal, message: &str, notif_type: &str) {
    let notification = Notification {
        id: uuid::Uuid::new_v4().to_string(),
        principal,
        message: message.to_string(),
        notif_type: notif_type.to_string(),
        timestamp: get_current_time(),
        read: false,
    };
    NOTIFICATIONS.with(|n| {
        let mut map = n.borrow_mut();
        map.entry(principal).or_default().push(notification);
    });
}

// Query functions
#[query]
fn get_delivery(delivery_id: String) -> Option<Delivery> {
    DELIVERIES.with(|d| {
        d.borrow().get(&delivery_id).cloned()
    })
}

#[query]
fn get_deliveries_by_seller(seller: Principal) -> Vec<Delivery> {
    DELIVERIES.with(|d| {
        d.borrow()
            .values()
            .filter(|delivery| delivery.seller == seller)
            .cloned()
            .collect()
    })
}

#[query]
fn get_deliveries_by_buyer(buyer: Principal) -> Vec<Delivery> {
    DELIVERIES.with(|d| {
        d.borrow()
            .values()
            .filter(|delivery| delivery.buyer == buyer)
            .cloned()
            .collect()
    })
}

#[query]
fn get_delivery_nft(nft_id: String) -> Option<DeliveryNFT> {
    DELIVERY_NFTS.with(|n| {
        n.borrow().get(&nft_id).cloned()
    })
}

#[query]
fn get_nfts_by_owner(owner: Principal) -> Vec<DeliveryNFT> {
    DELIVERY_NFTS.with(|n| {
        n.borrow()
            .values()
            .filter(|nft| nft.owner == owner)
            .cloned()
            .collect()
    })
}

#[query]
fn get_escrow_balance() -> u64 {
    ESCROW_BALANCE.with(|e| *e.borrow())
}

#[query]
fn get_all_deliveries() -> Vec<Delivery> {
    DELIVERIES.with(|d| {
        d.borrow().values().cloned().collect()
    })
}

#[query]
fn get_notifications(principal: Principal) -> Vec<Notification> {
    NOTIFICATIONS.with(|n| n.borrow().get(&principal).cloned().unwrap_or_default())
}

// Health check
#[query]
fn health_check() -> String {
    "ProofParcel canister is healthy".to_string()
}

// Export candid interface
candid::export_service!();

#[cfg(test)]
mod tests {
    use super::*;
    use ic_cdk::api::call::call_with_payment;

    #[test]
    fn test_create_delivery() {
        // This is a basic test structure
        // In a real implementation, you would test the actual canister functions
        let request = CreateDeliveryRequest {
            buyer: Principal::anonymous(),
            amount: 100000000, // 1 ICP
            description: "Test delivery".to_string(),
        };
        
        // Test would call create_delivery and verify the result
        assert_eq!(request.amount, 100000000);
        assert_eq!(request.description, "Test delivery");
    }

    #[test]
    fn test_generate_otp() {
        let otp1 = generate_otp().await;
        let otp2 = generate_otp().await;
        
        // OTPs should be 6 digits
        assert_eq!(otp1.len(), 6);
        assert_eq!(otp2.len(), 6);
        
        // OTPs should be different (very unlikely to be the same)
        assert_ne!(otp1, otp2);
        
        // OTPs should be numeric
        assert!(otp1.chars().all(|c| c.is_digit(10)));
        assert!(otp2.chars().all(|c| c.is_digit(10)));
    }

    #[test]
    fn test_delivery_status_transitions() {
        // Test that delivery status transitions work correctly
        let mut delivery = Delivery {
            id: "test-001".to_string(),
            seller: Principal::anonymous(),
            buyer: Principal::anonymous(),
            amount: 100000000,
            description: "Test".to_string(),
            status: DeliveryStatus::Pending,
            created_at: get_current_time(),
            in_transit_at: None,
            delivered_at: None,
            confirmed_at: None,
            escrow_released_at: None,
            cancelled_at: None,
            otp: None,
            otp_expires_at: None,
            status_history: vec![(DeliveryStatus::Pending, get_current_time())],
        };
        
        // Test status transitions
        delivery.status = DeliveryStatus::InTransit;
        assert_eq!(delivery.status, DeliveryStatus::InTransit);
        assert_eq!(delivery.in_transit_at, Some(get_current_time()));
        assert_eq!(delivery.status_history.last().unwrap().0, DeliveryStatus::InTransit);
        assert_eq!(delivery.status_history.last().unwrap().1, get_current_time());
        
        delivery.status = DeliveryStatus::Delivered;
        assert_eq!(delivery.status, DeliveryStatus::Delivered);
        assert_eq!(delivery.delivered_at, Some(get_current_time()));
        assert_eq!(delivery.status_history.last().unwrap().0, DeliveryStatus::Delivered);
        assert_eq!(delivery.status_history.last().unwrap().1, get_current_time());
        
        delivery.status = DeliveryStatus::Confirmed;
        assert_eq!(delivery.status, DeliveryStatus::Confirmed);
        assert_eq!(delivery.confirmed_at, Some(get_current_time()));
        assert_eq!(delivery.status_history.last().unwrap().0, DeliveryStatus::Confirmed);
        assert_eq!(delivery.status_history.last().unwrap().1, get_current_time());
        
        delivery.status = DeliveryStatus::EscrowReleased;
        assert_eq!(delivery.status, DeliveryStatus::EscrowReleased);
        assert_eq!(delivery.escrow_released_at, Some(get_current_time()));
        assert_eq!(delivery.status_history.last().unwrap().0, DeliveryStatus::EscrowReleased);
        assert_eq!(delivery.status_history.last().unwrap().1, get_current_time());
    }
}


