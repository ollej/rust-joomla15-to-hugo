# rust-joomla15-to-hugo

A Rust program to convert a Joomla 1.5 database to Hugo content.

## Usage

Update the `DATABASE_URL` in `src/main.rs` to the MySQL/MariaDB Joomla 1.5
database. The Hugo content files will be created in the directory `content`.

## Setup MariaDB locally

Some helpful commands to setup a local MariaDB database server using Docker.

### Setup MariaDB docker

```bash
docker run --detach --name joomla-db -p 3306:3306 --env MARIADB_USER=joomla-user --env MARIADB_PASSWORD=password --env MARIADB_ROOT_PASSWORD=password mariadb:latest
```

### Create MariaDB database

```bash
docker exec -i joomla-db sh -c 'exec mariadb -uroot -p"password" -e "create database joomladb"'
```

### Import MariaDB database

```bash
docker exec -i joomla-db sh -c 'exec mariadb -uroot -p"password" -D joomladb' < joomla.sql
```

### Connect to MariaDB database

```bash
docker exec -it joomla-db sh -c 'exec mariadb -uroot -p"password" -D joomladb'
```

