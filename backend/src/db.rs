use mongodb::{Client, Collection, IndexModel};
use mongodb::options::IndexOptions;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub password: String,  // Şifrelenmiş olarak saklanacak
    pub created_at: DateTime<Utc>,
    pub subscription: Subscription,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscription {
    pub plan: String,         // "Basic" veya "Pro"
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub status: String,       // "active" veya "expired"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
    pub tx_hash: String,
    pub network: String,      // "solana" veya "polygon"
    pub token: String,        // "USDC" veya "USDT"
    pub amount: f64,          // 50.0 veya 100.0
    pub user_email: String,
    pub payment_date: DateTime<Utc>,
    pub verification_status: String,
    pub plan_type: String,    // "Basic" veya "Pro"
}

pub struct Database {
    pub users: Collection<User>,
    pub payments: Collection<Payment>,
}

impl Database {
    pub async fn init(client: &Client) -> mongodb::error::Result<Self> {
        let db = client.database("tradetrack");
        let users = db.collection("users");
        let payments = db.collection("payments");

        // Users koleksiyonu için email indeksi
        let email_index = IndexModel::builder()
            .keys(mongodb::bson::doc! { "email": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build();
        users.create_index(email_index, None).await?;

        // Payments koleksiyonu için tx_hash ve network indeksi
        let payment_index = IndexModel::builder()
            .keys(mongodb::bson::doc! { 
                "tx_hash": 1, 
                "network": 1 
            })
            .options(IndexOptions::builder().unique(true).build())
            .build();
        payments.create_index(payment_index, None).await?;

        Ok(Self { users, payments })
    }

    // Kullanıcı ekleme
    pub async fn add_user(&self, user: User) -> mongodb::error::Result<()> {
        self.users.insert_one(user, None).await?;
        Ok(())
    }

    // Ödeme ekleme
    pub async fn add_payment(&self, payment: Payment) -> mongodb::error::Result<()> {
        self.payments.insert_one(payment, None).await?;
        Ok(())
    }

    // Hash kontrolü
    pub async fn check_transaction(&self, tx_hash: &str, network: &str) -> mongodb::error::Result<bool> {
        let filter = mongodb::bson::doc! {
            "tx_hash": tx_hash,
            "network": network
        };
        
        Ok(self.payments.find_one(filter, None).await?.is_some())
    }

    // Kullanıcıyı email ile bul
    pub async fn get_user_by_email(&self, email: &str) -> mongodb::error::Result<Option<User>> {
        self.users.find_one(
            mongodb::bson::doc! { "email": email },
            None
        ).await
    }

    // Abonelik güncelleme
    pub async fn update_subscription(&self, email: &str, plan_type: &str) -> mongodb::error::Result<()> {
        let start_date = Utc::now();
        let end_date = start_date + chrono::Duration::days(90); // 3 aylık abonelik

        self.users.update_one(
            mongodb::bson::doc! { "email": email },
            mongodb::bson::doc! {
                "$set": {
                    "subscription": {
                        "plan": plan_type,
                        "start_date": start_date,
                        "end_date": end_date,
                        "status": "active"
                    }
                }
            },
            None
        ).await?;

        Ok(())
    }

    // Clone trait implementation
    impl Clone for Database {
        fn clone(&self) -> Self {
            Self {
                users: self.users.clone(),
                payments: self.payments.clone(),
            }
        }
    }
} 