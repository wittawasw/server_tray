use askama::Template;
use sea_orm::{
    Database, DatabaseConnection, DerivePrimaryKey, DeriveEntityModel, DeriveRelation, EnumIter, Set, entity::*,
    // query::*,
};
use sea_orm_migration::{
    prelude::{MigrationTrait, SchemaManager, Table, Iden, DeriveMigrationName},
    sea_query, sea_query::ColumnDef,
};
use std::{
    fs,
    net::SocketAddr,
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
};
use tokio::{runtime::Runtime, sync::oneshot};
use warp::{Filter, Rejection, Reply};

#[derive(Clone)]
pub struct ServerConfig {
    pub static_dir: PathBuf,
    pub address: SocketAddr,
    pub db_path: String,
}

#[derive(Clone)]
pub struct ServerHandle {
    handle: Arc<Mutex<Option<thread::JoinHandle<()>>>>,
    shutdown_tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
    config: ServerConfig,
}

// Product Entity
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "products")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub quantity: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// Migration
#[derive(DeriveMigrationName)]
pub struct ProductMigration;

#[async_trait::async_trait]
impl MigrationTrait for ProductMigration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), sea_orm_migration::DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Products::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Products::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Products::Name).string().not_null())
                    .col(ColumnDef::new(Products::Quantity).integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), sea_orm_migration::DbErr> {
        manager
            .drop_table(Table::drop().table(Products::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Products {
    Table,
    Id,
    Name,
    Quantity,
}

// Server Handle
impl ServerHandle {
    pub fn new(config: ServerConfig) -> Self {
        Self {
            handle: Arc::new(Mutex::new(None)),
            shutdown_tx: Arc::new(Mutex::new(None)),
            config,
        }
    }

    pub fn start(&self) {
        let mut handle_guard = self.handle.lock().unwrap();
        if handle_guard.is_some() {
            return;
        }

        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        *self.shutdown_tx.lock().unwrap() = Some(shutdown_tx);
        let config = self.config.clone();

        let h = thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                // --- Ensure DB file exists
                if !PathBuf::from(&config.db_path).exists() {
                    if let Some(parent) = PathBuf::from(&config.db_path).parent() {
                        fs::create_dir_all(parent).unwrap();
                    }
                    fs::File::create(&config.db_path).unwrap();
                }

                let db = Database::connect(format!("sqlite://{}", config.db_path))
                    .await
                    .expect("DB connection failed");

                // --- Run migration automatically
                let schema_manager = SchemaManager::new(&db);
                ProductMigration.up(&schema_manager).await.unwrap();

                // --- Routes
                let html_route = warp::path::end().and_then(render_home);

                let api = warp::path("api");
                let db_filter = warp::any().map(move || db.clone());

                let get_products = warp::path("products")
                    .and(warp::get())
                    .and(db_filter.clone())
                    .and_then(get_products);

                let add_product = warp::path("product")
                    .and(warp::post())
                    .and(warp::body::json())
                    .and(db_filter.clone())
                    .and_then(add_product);

                let routes = html_route
                    .or(api.and(get_products))
                    .or(api.and(add_product))
                    .or(warp::path("assets").and(warp::fs::dir(config.static_dir.clone())));

                let (_, server) =
                    warp::serve(routes).bind_with_graceful_shutdown(config.address, async {
                        let _ = shutdown_rx.await;
                    });

                server.await;
            });
        });

        *handle_guard = Some(h);
    }

    pub fn stop(&self) {
        let mut handle_guard = self.handle.lock().unwrap();
        let mut shutdown_guard = self.shutdown_tx.lock().unwrap();

        if let Some(tx) = shutdown_guard.take() {
            let _ = tx.send(());
        }

        if let Some(h) = handle_guard.take() {
            let _ = h.join();
        }
    }

    pub fn is_running(&self) -> bool {
        self.handle.lock().unwrap().is_some()
    }

    pub fn address(&self) -> SocketAddr {
        self.config.address
    }
}

// HTML Rendering + CRUD
#[derive(Template)]
#[template(path = "index.html")]
struct HomeTemplate<'a> {
    application_name: &'a str,
}

async fn render_home() -> Result<impl Reply, Rejection> {
    let template = HomeTemplate {
        application_name: "Simple Shop",
    };
    Ok(warp::reply::html(template.render().unwrap()))
}

async fn get_products(db: DatabaseConnection) -> Result<impl Reply, Rejection> {
    let products = Entity::find().all(&db).await.unwrap();
    Ok(warp::reply::json(&products))
}

#[derive(serde::Deserialize)]
struct NewProduct {
    name: String,
    quantity: i32,
}

async fn add_product(new: NewProduct, db: DatabaseConnection) -> Result<impl Reply, Rejection> {
    let product = ActiveModel {
        name: Set(new.name),
        quantity: Set(new.quantity),
        ..Default::default()
    };
    let res = product.insert(&db).await.unwrap();
    Ok(warp::reply::json(&res))
}

// runner
// pub fn run_blocking(config: ServerConfig) {
//     let rt = Runtime::new().unwrap();
//     rt.block_on(async move {
//         if !PathBuf::from(&config.db_path).exists() {
//             if let Some(parent) = PathBuf::from(&config.db_path).parent() {
//                 fs::create_dir_all(parent).unwrap();
//             }
//             fs::File::create(&config.db_path).unwrap();
//         }

//         let db = Database::connect(format!("sqlite://{}", config.db_path))
//             .await
//             .expect("DB connection failed");

//         let schema_manager = SchemaManager::new(&db);
//         ProductMigration.up(&schema_manager).await.unwrap();

//         let html_route = warp::path::end().and_then(render_home);
//         let db_filter = warp::any().map(move || db.clone());
//         let get_products = warp::path("products")
//             .and(warp::get())
//             .and(db_filter.clone())
//             .and_then(get_products);

//         let routes = html_route
//             .or(warp::path("api").and(get_products))
//             .or(warp::path("assets").and(warp::fs::dir(config.static_dir.clone())));

//         let (_, server) = warp::serve(routes).bind_with_graceful_shutdown(config.address, async {
//             let _ = tokio::signal::ctrl_c().await;
//         });

//         server.await;
//     });
// }
