use std::fs::OpenOptions;
use std::io::Write;
use std::{borrow::Borrow, env};

use tonic::{
    codegen::InterceptedService,
    metadata::{Ascii, MetadataValue},
    service::Interceptor,
    transport::{Channel, ClientTlsConfig},
    Request, Status,
};

use lib::google::firestore::v1::{
    firestore_client::FirestoreClient,
    run_query_request::QueryType,
    structured_query::{
        field_filter::Operator, filter::FilterType, CollectionSelector, FieldFilter,
        FieldReference, Filter, Projection,
    },
    value::ValueType,
    ArrayValue, CreateDocumentRequest, Document, ListCollectionIdsRequest, ListDocumentsRequest,
    ListDocumentsResponse, MapValue, RunQueryRequest, StructuredQuery, Value,
};

mod lib;

const URL: &'static str = "https://firestore.googleapis.com";
const DOMAIN: &'static str = "firestore.googleapis.com";

pub type BoxError = Box<dyn std::error::Error + Sync + Send + 'static>;

// You can also use the `Interceptor` trait to create an interceptor type
// that is easy to name
struct MyInterceptor {
    token: MetadataValue<Ascii>,
}

impl Interceptor for MyInterceptor {
    fn call(&mut self, mut req: Request<()>) -> Result<Request<()>, Status> {
        req.metadata_mut()
            .insert("authorization", self.token.clone());
        Ok(req)
    }
}

async fn output(document: Document) -> Result<(), BoxError> {
    let mut results_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("results.json")?;

    let fields = document.fields;
    let serialized = serde_json::to_string_pretty(&fields)?;
    writeln!(results_file, "{}", serialized)?;
    Ok(())
}

async fn get_client(
    config: &Config,
) -> Result<FirestoreClient<InterceptedService<Channel, MyInterceptor>>, BoxError> {
    let endpoint =
        Channel::from_static(URL).tls_config(ClientTlsConfig::new().domain_name(DOMAIN))?;

    let bearer_token = format!("Bearer {}", config.access_token);
    let header_value = MetadataValue::from_str(&bearer_token)?;
    let channel = endpoint.connect().await?;
    let service = FirestoreClient::with_interceptor(
        channel,
        MyInterceptor {
            token: header_value,
        },
    );
    Ok(service)
}

async fn list_collection_ids(config: &Config) -> Result<Vec<String>, BoxError> {
    let parent = format!(
        "projects/{}/databases/(default)/documents/DxB-Dev-Norway/root",
        config.project_id
    );
    let res = get_client(config)
        .await?
        .list_collection_ids(ListCollectionIdsRequest {
            parent,
            page_size: 10,
            page_token: "".into(),
        })
        .await?;
    Ok(res.into_inner().collection_ids)
}

async fn list_documents(config: &Config) -> Result<Vec<Document>, BoxError> {
    let parent = format!(
        "projects/{}/databases/(default)/documents/DxB-Dev-Norway/root",
        config.project_id
    );
    let res = get_client(config)
        .await?
        .list_documents(ListDocumentsRequest {
            parent,
            collection_id: "products".into(),
            page_size: 10,
            page_token: "".into(),
            order_by: "".into(),
            mask: None,
            show_missing: false,
            consistency_selector: None,
        })
        .await?;
    Ok(res.into_inner().documents)
}

async fn run_query(config: &Config) -> Result<(), BoxError> {
    let parent = format!(
        "projects/{}/databases/(default)/documents/DxB-Dev-Norway/root",
        config.project_id
    );
    let res = get_client(config)
        .await?
        .run_query(RunQueryRequest {
            parent,
            query_type: Some(QueryType::StructuredQuery(StructuredQuery {
                select: Some(Projection {
                    fields: vec![
                        FieldReference {
                            field_path: "name".into(),
                        },
                    ],
                }),
                from: vec![CollectionSelector {
                    collection_id: "products".into(),
                    all_descendants: true,
                }],
                r#where: /*Some(Filter {
                    filter_type: Some(FilterType::FieldFilter(FieldFilter {
                        field: Some(FieldReference {
                            field_path: "name".into(),
                        }),
                        op: Operator::Equal as i32,
                        value: Some(Value {
                            value_type: Some(ValueType::StringValue(
                                "something".into(),
                            )),
                        }),
                    })),
                })*/ None,
                order_by: vec![],
                start_at: None,
                end_at: None,
                offset: 0,
                limit: Some(5),
            })),
            consistency_selector: None,
        })
        .await?;
    let mut stream = res.into_inner();
    let mut message = stream.message().await?;
    while message.is_some() {
        let document = message.unwrap();
        let value = document.document.unwrap();
        output(value).await?;
        message = stream.message().await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(args);

    run_query(&config).await?;
    Ok(())
}

struct Config {
    pub project_id: String,
    pub access_token: String,
}

fn parse_config(args: Vec<String>) -> Config {
    let mut project_id = env::var("GCP_PROJECT_ID").ok();
    let mut access_token = env::var("GCP_ACCESS_TOKEN").ok();
    for (index, arg) in args.iter().enumerate() {
        if arg == "--project_id" {
            if index == args.len() - 1 {
                panic!("project_id was provided without a value")
            }
            project_id = Some(args[index + 1].clone());
            continue;
        }

        if arg == "--access_token" {
            if index == args.len() - 1 {
                panic!("access_token was provided without a value")
            }
            access_token = Some(args[index + 1].clone());
            continue;
        }
    }
    Config {
        project_id: project_id.expect("GCP_PROJECT_ID or --project_id was not provided"),
        access_token: access_token.expect("GCP_ACCESS_TOKEN or --access_token was not provided"),
    }
}
