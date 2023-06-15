// use axum::{routing, Extension, Router};

// use crate::{
//     control::documents::{
//         get_documents, post_document_content, post_documents, put_documents_metadata,
//     },
//     repository::documents::PgDocumentRepository,
// };

// pub fn documents_router(documents_repository: PgDocumentRepository) -> Router {
//     let root_handler = routing::get(get_documents::<PgDocumentRepository>)
//         .post(post_documents::<PgDocumentRepository>);

//      Router::new()
//         .route("/", root_handler)
//         .route(
//             "/:document_id",
//             routing::post(post_document_content::<PgDocumentRepository>),
//         )
//         .route(
//             "/:document_id/metadata",
//             routing::put(put_documents_metadata::<PgDocumentRepository>),
//         )
//         .layer(Extension(documents_repository))
// }
