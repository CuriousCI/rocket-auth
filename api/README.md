# avis-management-system
RESTful API to manage AVIS donors &amp; bookings 

## Setup
Create a **.env** file in your *project* directory, containing the following line:
```shell
DATABASE_URL=postgres://<user>:<password>@<ip>/<database>
```

Then create a **Rocket.toml** file in your *project* directory, containing the following code:
```shell	
[global.databases]
avis = postgres://<user>:<password>@<ip>/<database>
```
Next make sure you have installed a **Rust nightly toolchain** 
and set it up as the project's toolchain 
*(Nightly Rust is required for Rocket 4.10.0)*

```shell
rustup show

rustup install nightly
cd avis-management-system
rustup override set nightly
```

If the current nightly doesn't have 
[**rls**](https://rust-lang.github.io/rustup-components-history/)
you can install an old funcioning one

```shell
rustup show

rustup install nightly-2021-12-05
cd avis-management-system
rustup override set nightly-2021-12-05
```

Next, you have to add the **/public** folder to your project,
by building the website and copying the **/public** folder into
*avis-management-system*:

```bash
git clone https://github.com/CuriousCI/avis-civitavecchia

cd avis-civitavecchia
npm install
npm run build

cd ..
```

If you are using *PowerShell*, you can run:

```ps1
Copy-Item .\avis-civitavecchia\public\ -Destination .\avis-management-system\ -Recurse
```

Next you can build or run the project
```shell
cargo run
```

***use version 4 of [rocket's documentation](https://rocket.rs/v0.4/)***

## Database migrations
The project uses [Postgres](https://www.postgresql.org/) through 
[diesel](https://diesel.rs/). First of all you have to install **diesel_cli**
from *cargo*

```shell
cargo install diesel_cli --no-default-features --features postgres
```
Next, if you want to add a table, you should run
```shell
diesel migration generate create_table
```
and two files will be created
```shell
Creating migrations/20160815133237_create_table/up.sql
Creating migrations/20160815133237_create_table/down.sql
```
The **up.sql** file should contain the **CREATE Table** query, and
the **down.sql** file should contain the **DROP Table** query.

To apply the migration, you have to run
```shell
diesel migration run
```
To make sure that **down.sql** is correct, you should run
```shell
diesel migration redo
```
The **src/schemas.rs** will be updated, containing the new table.

## **Set the _Nullable< Int4 >_ option to your schema manually!**
```rust
id -> Nullable<Int4>,
```

Then in **src/models/** you have to add a new file: ***table_name*.rs**
and implement a struct following the other models' schema.

## Useful tools
https://github.com/akarika/crud-rocket-postgres/tree/master/src
https://docs.diesel.rs/1.4.x/diesel/query_dsl/methods/index.html