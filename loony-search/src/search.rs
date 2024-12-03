use std::path::Path;

use crate::error::AppError;
use crate::AppState;
use axum::{
    extract::Path as AxumPath,
    extract::State,
    http::{header, StatusCode},
    response::IntoResponse,
};
use tantivy::query::QueryParser;
use tantivy::{collector::TopDocs, Searcher};
use tantivy::{directory::MmapDirectory, schema::*};
use tantivy::{doc, Index, IndexWriter, ReloadPolicy};

const TITLE: [&str; 4] = [
    "What is Lorem Ipsum?",
    "Why do we use it?",
    "Where does it come from?",
    "Where can I get some?",
];

const BODY: [&str; 4] = [
    "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.",
    "It is a long established fact that a reader will be distracted by the readable content of a page when looking at its layout. The point of using Lorem Ipsum is that it has a more-or-less normal distribution of letters, as opposed to using 'Content here, content here', making it look like readable English. Many desktop publishing packages and web page editors now use Lorem Ipsum as their default model text, and a search for 'lorem ipsum' will uncover many web sites still in their infancy. Various versions have evolved over the years, sometimes by accident, sometimes on purpose (injected humour and the like).",
    "Contrary to popular belief, Lorem Ipsum is not simply random text. It has roots in a piece of classical Latin literature from 45 BC, making it over 2000 years old. Richard McClintock, a Latin professor at Hampden-Sydney College in Virginia, looked up one of the more obscure Latin words, consectetur, from a Lorem Ipsum passage, and going through the cites of the word in classical literature, discovered the undoubtable source. Lorem Ipsum comes from sections 1.10.32 and 1.10.33 of 'de Finibus Bonorum et Malorum' (The Extremes of Good and Evil) by Cicero, written in 45 BC. This book is a treatise on the theory of ethics, very popular during the Renaissance. The first line of Lorem Ipsum, 'Lorem ipsum dolor sit amet..', comes from a line in section 1.10.32.",
    "There are many variations of passages of Lorem Ipsum available, but the majority have suffered alteration in some form, by injected humour, or randomised words which don't look even slightly believable. If you are going to use a passage of Lorem Ipsum, you need to be sure there isn't anything embarrassing hidden in the middle of text. All the Lorem Ipsum generators on the Internet tend to repeat predefined chunks as necessary, making this the first true generator on the Internet. It uses a dictionary of over 200 Latin words, combined with a handful of model sentence structures, to generate Lorem Ipsum which looks reasonable. The generated Lorem Ipsum is therefore always free from repetition, injected humour, or non-characteristic words etc.",
];

#[derive(Clone)]
pub struct Search {
    pub searcher: Searcher,
    pub parser: QueryParser,
    pub schema: Schema,
}

pub fn init_search() -> Search {
    let __path = std::env::var("SEARCH_INDEX_PATH").unwrap();

    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("body", TEXT);

    let schema = schema_builder.build();

    let index = Index::open_or_create(
        MmapDirectory::open(Path::new(&__path)).unwrap(),
        schema.clone(),
    )
    .unwrap();

    let mut index_writer: IndexWriter = index.writer(50_000_000).unwrap();

    let title = schema.get_field("title").unwrap();
    let body = schema.get_field("body").unwrap();

    for x in 0..=3 {
        index_writer
            .add_document(doc!(
            title => TITLE[x],
            body => BODY[x],
            ))
            .unwrap();
    }

    index_writer.commit().unwrap();

    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommitWithDelay)
        .try_into()
        .unwrap();

    let searcher = reader.searcher();
    let parser = QueryParser::for_index(&index, vec![title, body]);
    Search {
        searcher,
        parser,
        schema,
    }
}

pub async fn insert() -> Result<impl IntoResponse, AppError> {
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain")],
        "Hello World!",
    ))
}

pub async fn search(
    State(app_state): State<AppState>,
    AxumPath(query): AxumPath<String>,
) -> Result<impl IntoResponse, AppError> {
    let search = &app_state.search;

    let query = search.parser.parse_query(&query).unwrap();
    let top_docs = search
        .searcher
        .search(&query, &TopDocs::with_limit(1))
        .unwrap();

    for (_score, doc_address) in top_docs {
        let retrieved_doc: TantivyDocument = search.searcher.doc(doc_address).unwrap();
        println!("{}", retrieved_doc.to_json(&search.schema));
    }

    // let query = search
    //     .parser
    //     .parse_query("title:lorem^20 body:reader^70")
    //     .unwrap();

    // let (_score, doc_address) = search
    //     .searcher
    //     .search(&query, &TopDocs::with_limit(1))
    //     .unwrap()
    //     .into_iter()
    //     .next()
    //     .unwrap();

    // let explanation = query.explain(&search.searcher, doc_address).unwrap();
    // println!("{}", explanation.to_pretty_json());

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain")],
        "Hello",
    ))
}
