# Firestore Query

This is a quick tool to query firestore on a more granular level that the UI doesn't allow.

## Build

This project needs Protobuf installed to work. Usually this is bundled in with the dependencies, but for certain
architectures (e.g. Arm Macs) it needs to be explicitly installed.

### Install Protobuf

```bash
brew install protobuf
```

### Compile

```bash
cargo build
```

## Run

Requires at least an access token (using `GCP_ACCESS_TOKEN` or `--access_token`) and a project ID (using `GCP_PROJECT_ID` or `--project_id <project ID>`).

Example:
```bash
GCP_ACCESS_TOKEN=$(gcloud auth print-access-token) cargo run -- --project_id <project ID>
```
