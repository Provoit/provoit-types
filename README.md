# Provoit types

This crate holds Provoit's shared types.

## Features

- **Default**

By default, data types are provided with serialization and de-serialization using serde.  
There are types for a complete record, a new one or an updating one.

- **diesel**

`diesel` feature enables *derive* attributes on models for querying. It also enables the `schema` module for compile time query validation.  
Finally, this enables a `migrations` module which embed the migrations for later run.

## Tools

This project needs the rust tooling, including cargo.  
The diesel cli is needed, you can install it with the following command.

```sh
cargo install diesel_cli --no-default-features --features mysql
```

This project also needs a **MySQL** database, the easiest way to have one is with docker.

## Contributing

1. Run `diesel migration generate create_<table>` to create the migrations
2. Complete the newly created `up.sql` and `down.sql`
3. Create a file named after your model in `src/models/`
4. Create a *struct* with the model name and the corresponding fields
5. This *struct* derives from `Debug`, `Clone`, 'Serialize' & `Deserialize`
6. With *diesel* feature, it derives from `Queryable`. You can also specify the table name if needed
7. The model file should also contains `struct` for a new model and an updating model


You can quickly setup a database and run the migration with the following commands. This is actually required to generate the schema file.

```sh
docker run -e MYSQL_ROOT_PASSWORD=root -e MYSQL_DATABASE=provoit -d -p 3306:3306 --rm mysql:latest
diesel migration run --database-url mysql://root:root@127.0.0.1/provoit
```

Take example on other models.
