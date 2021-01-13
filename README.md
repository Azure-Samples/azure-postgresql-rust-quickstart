# Use Rust to connect and query data in Azure Database for PostgreSQL - Single Server

You will learn how to use the [PostgreSQL driver for Rust](https://github.com/sfackler/rust-postgres) to interact with Azure Database for PostgreSQL by exploring CRUD (create, read, update, delete) operations implemented in the sample code. Finally, you can run the application locally to see it in action.

## Prerequisites
For this quickstart you need:

- An Azure account with an active subscription. [Create an account for free](https://azure.microsoft.com/free/?WT.mc_id=data-12578-abhishgu).
- Install [Rust](https://www.rust-lang.org/tools/install) (preferably a recent version)
- Create an Azure Database for PostgreSQL single server using [Azure portal](https://docs.microsoft.com/azure/postgresql/quickstart-create-server-database-portal?WT.mc_id=data-12578-abhishgu) <br/> or [Azure CLI](https://docs.microsoft.com/azure/postgresql/quickstart-create-server-database-azure-cli?WT.mc_id=data-12578-abhishgu).
- Based on whether you are using public or private access, complete **ONE** of the actions below to enable connectivity.

  |Action| Connectivity method|How-to guide|
  |:--------- |:--------- |:--------- |
  | **Configure firewall rules** | Public | [Portal](https://docs.microsoft.com/azure/postgresql/howto-manage-firewall-using-portal?WT.mc_id=data-12578-abhishgu) <br/> [CLI](https://docs.microsoft.com/azure/postgresql/howto-manage-firewall-using-cli?WT.mc_id=data-12578-abhishgu)|
  | **Configure Service Endpoint** | Public | [Portal](https://docs.microsoft.com/azure/postgresql/howto-manage-vnet-using-portal?WT.mc_id=data-12578-abhishgu) <br/> [CLI](https://docs.microsoft.com/azure/postgresql/howto-manage-vnet-using-cli?WT.mc_id=data-12578-abhishgu.md)|
  | **Configure private link** | Private | [Portal](https://docs.microsoft.com/azure/postgresql/howto-configure-privatelink-portal?WT.mc_id=data-12578-abhishgu) <br/> [CLI](https://docs.microsoft.com/azure/postgresql/howto-configure-privatelink-cli?WT.mc_id=data-12578-abhishgu) |

- Install [Git](https://git-scm.com/downloads)

## Get database connection information
Connecting to an Azure Database for PostgreSQL database requires the fully qualified server name and login credentials. You can get this information from the Azure portal.

1. In the [Azure portal](https://portal.azure.com/), search for and select your Azure Database for PostgreSQL server name.
1. On the server's **Overview** page, copy the fully qualified **Server name** and the **Admin username**. The fully qualified **Server name** is always of the form *\<my-server-name>.postgres.database.azure.com*, and the **Admin username** is always of the form *\<my-admin-username>@\<my-server-name>*.

## Run the application

To begin with, run the following command to clone the sample repository:

```bash
git clone https://github.com/Azure-Samples/azure-postgresql-rust-quickstart.git
```

Set the required environment variables with the values you copied from the Azure portal:

```bash
export POSTGRES_HOST=<server name e.g. my-server.postgres.database.azure.com>
export POSTGRES_USER=<admin username e.g. my-admin-user@my-server>
export POSTGRES_PASSWORD=<admin password>
export POSTGRES_DBNAME=<database name. it is optional and defaults to postgres>
```

To run the application, change into the directory where you cloned it and execute `cargo run`:

```bash
cd azure-postgresql-rust-quickstart
cargo run
```

You should see an output similar to this:

```bash
dropped 'inventory' table
inserted item with id 1
inserted item with id 2
inserted item with id 3 
inserted item with id 4 
inserted item with id 5 
quantity for item item-1 = 42
listing items...
item info: id = 1, name = item-1, quantity = 42 
item info: id = 2, name = item-2, quantity = 43 
item info: id = 3, name = item-3, quantity = 11 
item info: id = 4, name = item-4, quantity = 32 
item info: id = 5, name = item-5, quantity = 24 
updated item id 1 to quantity = 27
updated item id 2 to quantity = 14
updated item id 3 to quantity = 31
updated item id 4 to quantity = 16
updated item id 5 to quantity = 10
deleted item info: id = 4, name = item-4, quantity = 16 
```

## Clean up resources

To clean up all resources used during this quickstart, delete the resource group using the following command:

```azurecli
az group delete \
    --name $AZ_RESOURCE_GROUP \
    --yes
```

## Additional resources

- [Quickstart: Use Python to connect and query data in Azure Database for PostgreSQL - Single Server](https://docs.microsoft.com/azure/postgresql/connect-python?WT.mc_id=data-12578-abhishgu)
- [Tutorial: Deploy a Django web app with PostgreSQL in Azure App Service](https://docs.microsoft.com/azure/app-service/tutorial-python-postgresql-app?toc=https%3A%2F%2Fdocs.microsoft.com%2Fen-us%2Fazure%2Fpostgresql%2Ftoc.json&bc=https%3A%2F%2Fdocs.microsoft.com%2Fen-us%2Fazure%2Fbread%2Ftoc.json&tabs=bash%2Cclone&WT.mc_id=data-12578-abhishgu)
- [Connect and query overview for Azure database for PostgreSQL- Single Server](https://docs.microsoft.com/azure/postgresql/how-to-connect-query-guide?WT.mc_id=data-12578-abhishgu)
- [Auto grow storage using the Azure portal in Azure Database for PostgreSQL - Single Server](https://docs.microsoft.com/azure/postgresql/howto-auto-grow-storage-portal?WT.mc_id=data-12578-abhishgu)