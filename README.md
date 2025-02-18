<div align="center">
  <img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/sellershut/categories-service/check.yaml?label=build">
  
 <a href="https://codecov.io/github/sellershut/categories-service" > 
 <img src="https://codecov.io/github/sellershut/categories-service/graph/badge.svg?token=ydM1VPPQqw"/> 
 </a>
</div>
<h1 align="center">categories-service</h1>
<p align="center">
A gRPC server for interacting with platform categories
<br />

## Build Dependencies
- `protoc`
- [`sqlx-cli`](https://github.com/launchbadge/sqlx)

## Services
- `postgres` - The main database, included in the development [docker stack](contrib/compose.yaml)

## Features

### Queries
- `category_by_id` - Get a category with a specified id
- `categories` - Get categories
- `sub_categories` - Get sub-categories of a provided category (gets top-level if no category is provided)

Queries implement cursor-based pagination

### Mutations
- `create` - Add a category to the database
- `upsert` - Perform upsert operations on categories
- `delete` - Delete a category with the specified id

## Usage

- Clone the repository:
```sh
git clone https://github.com/sellershut/categories-service.git
cd categories-service
```

- Start your docker stack:
```sh
docker compose -f contrib/compose.yaml up -d
```

- Run database migrations:
```sh
cp .env.example .env
cargo sqlx migrate run
```

> [!IMPORTANT]  
> The [config file](categories.toml) use the defaults in the [docker stack](contrib/compose.yaml). If you update either, ensure they are both aligned

- Run the application
```sh
cargo run -- --help
```
