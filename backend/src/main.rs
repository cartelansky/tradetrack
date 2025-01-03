use actix_web::{web, App, HttpServer, HttpResponse};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use mongodb::{Client, options::ClientOptions};
use dotenv::dotenv;
use std::env;
use chrono::{DateTime, Utc};
use actix_web::http;

mod db;
use db::{Database, User, Payment, Subscription};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    email: String,
    password: String,  // Şifrelenmiş olarak saklanmalı
    created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Payment {
    tx_hash: String,
    network: String,
    payment_date: DateTime<Utc>,
    user_email: String,
}

#[derive(Debug, Serialize)]
struct VerificationResponse {
    success: bool,
    message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PaymentVerification {
    tx_hash: String,
    network: String,
    email: String,
}

async fn verify_payment(
    payment: web::Json<PaymentVerification>,
    database: web::Data<Database>,
) -> HttpResponse {
    // Hash kontrolü
    match database.check_transaction(&payment.tx_hash, &payment.network).await {
        Ok(true) => {
            return HttpResponse::BadRequest().json(VerificationResponse {
                success: false,
                message: Some("Bu işlem hash'i daha önce kullanılmış".to_string())
            });
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(VerificationResponse {
                success: false,
                message: Some(format!("Veritabanı hatası: {}", e))
            });
        }
        _ => {}
    }

    // Kullanıcı kontrolü
    let user = match database.get_user_by_email(&payment.email).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return HttpResponse::BadRequest().json(VerificationResponse {
                success: false,
                message: Some("Kullanıcı bulunamadı".to_string())
            });
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(VerificationResponse {
                success: false,
                message: Some(format!("Veritabanı hatası: {}", e))
            });
        }
    };

    // Ödeme doğrulama işlemi...
    let verification_result = match payment.network.as_str() {
        "solana" => {
            // Solana doğrulama...
        }
        "polygon" => {
            // Polygon doğrulama...
        }
        _ => {
            return HttpResponse::BadRequest().json(VerificationResponse {
                success: false,
                message: Some("Geçersiz ağ".to_string())
            });
        }
    };

    // Ödeme başarılıysa kaydet
    if verification_result {
        let new_payment = Payment {
            tx_hash: payment.tx_hash.clone(),
            network: payment.network.clone(),
            token: payment.token.clone(),
            amount: payment.amount,
            user_email: payment.email.clone(),
            payment_date: Utc::now(),
            verification_status: "verified".to_string(),
            plan_type: payment.plan_type.clone(),
        };

        if let Err(e) = database.add_payment(new_payment).await {
            return HttpResponse::InternalServerError().json(VerificationResponse {
                success: false,
                message: Some(format!("Ödeme kaydedilemedi: {}", e))
            });
        }

        // Kullanıcının aboneliğini güncelle
        if let Err(e) = database.update_subscription(&payment.email, &payment.plan_type).await {
            return HttpResponse::InternalServerError().json(VerificationResponse {
                success: false,
                message: Some(format!("Abonelik güncellenemedi: {}", e))
            });
        }

        HttpResponse::Ok().json(VerificationResponse {
            success: true,
            message: Some("Ödeme başarıyla doğrulandı".to_string())
        })
    } else {
        HttpResponse::BadRequest().json(VerificationResponse {
            success: false,
            message: Some("Ödeme doğrulanamadı".to_string())
        })
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mongodb_uri = env::var("MONGODB_URI")
        .expect("MONGODB_URI must be set");
    
    let client_options = ClientOptions::parse(&mongodb_uri)
        .await
        .expect("Failed to parse MongoDB URI");
    
    let client = Client::with_options(client_options)
        .expect("Failed to initialize MongoDB client");

    let database = Database::init(&client)
        .await
        .expect("Failed to initialize database");

    println!("MongoDB bağlantısı başarılı!");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("https://your-frontend-domain.vercel.app")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::CONTENT_TYPE])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(database.clone()))
            .service(
                web::scope("/api")
                    .route("/verify-payment", web::post().to(verify_payment))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
} 