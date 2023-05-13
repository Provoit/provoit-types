# Provoit types

This crate holds Provoit's shared types.

## Features

- **Default**

By default, data types are provided with serialization and de-serialization using serde.  
There are types for a complete record, a new one or an updating one.

- **diesel**

`diesel` feature enables *derive* attributes on models for querying. It also enables the `schema` module for compile time query validation.  
Finally, this enables a `migrations` module which embed the migrations for later run.

## Contributing

1. Run `diesel migration generate create_<table>` to create the migrations
2. Complete the newly created `up.sql` and `down.sql`
3. Create a file named after your model in `src/models/`
4. Create a *struct* with the model name and the corresponding fields
5. This *struct* derives from `Debug`, `Clone`, 'Serialize' & `Deserialize`
6. With *diesel* feature, it derives from `Queryable`. You can also specify the table name if needed
7. The model file should also contains `struct` for a new model and an updating model

Take example on other models.
