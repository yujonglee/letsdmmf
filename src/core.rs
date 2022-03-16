use std::{fs, sync::Arc};

use datamodel_connector::ConnectorCapabilities;
use prisma_models::InternalDataModelBuilder;
use query_core::schema::QuerySchemaRef;
use query_core::schema_builder;
use request_handlers::dmmf;

use crate::{location, validate};

// https://github.com/prisma/prisma-engines/blob/c9f86866d2fb27b2066e5447ee7f6f65c46c5707/query-engine/query-engine-node-api/src/node_api/functions.rs#L30
pub fn get_dmmf(datamodel_string: String) -> Result<String, String> {
    let datamodel =
        datamodel::parse_datamodel(&datamodel_string).map_err(|errors| errors.to_string())?;

    let config =
        datamodel::parse_configuration(&datamodel_string).map_err(|errors| errors.to_string())?;

    let datasource = config.subject.datasources.first();

    let capabilities = datasource
        .map(|ds| ds.capabilities())
        .unwrap_or_else(ConnectorCapabilities::empty);

    let referential_integrity = datasource
        .map(|ds| ds.referential_integrity())
        .unwrap_or_default();

    let internal_data_model = InternalDataModelBuilder::from(&datamodel.subject).build("".into());

    let query_schema: QuerySchemaRef = Arc::new(schema_builder::build(
        internal_data_model,
        schema_builder::BuildMode::Modern,
        true,
        capabilities,
        config.subject.preview_features().iter().collect(),
        referential_integrity,
    ));

    let dmmf = dmmf::render_dmmf(&datamodel.subject, query_schema);

    // https://github.com/prisma/prisma/blob/6561b8adf4005a7762716cd73bb6df545ff0762e/packages/client/src/runtime/externalToInternalDmmf.ts#L22
    // Is getMappings really needed?
    let dmmf_string = serde_json::to_string(&dmmf).unwrap();

    Ok(dmmf_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot() {
        insta::assert_debug_snapshot!(get_dmmf(String::from(
            r#"
            // This is your Prisma schema file,
            // learn more about it in the docs: https://pris.ly/d/prisma-schema
            
            datasource db {
              provider = "sqlite"
              url      = "file:./db.sqlite"
            }
            
            generator client {
              provider = "prisma-client-js"
            }
            
            /// @seed
            model User {
              /// @createOnly
              /// @readOnly
              id        String   @id @default(cuid())
              /// @mock faker.name.firstName
              name      String
              /// @scalar Email
              email     String
              /// This is a password
              /// @writeOnly
              password  String
              /// @readOnly
              createdAt DateTime @default(now())
              /// @readOnly
              updatedAt DateTime @default(now())
              cart      Cart?
            }
            
            model Cart {
              /// @readOnly
              id        String    @id @default(cuid())
              /// @readOnly
              createdAt DateTime  @default(now())
              /// @readOnly
              updatedAt DateTime  @default(now())
              user      User      @relation(fields: [userId], references: [id])
              userId    String    @unique
              items     Product[]
              coupon    String?
            }
            
            model Product {
              /// @readOnly
              id        String   @id @default(cuid())
              name      String
              price     Int
              image     String
              /// @readOnly
              createdAt DateTime @default(now())
              /// @readOnly
              updatedAt DateTime @default(now())
              carts     Cart[]
            }
            "#
        )))
    }
}

pub fn get_schema(location: String) -> Result<String, String> {
    let location_type = location::new(&location);

    use location::Location;

    match location_type {
        Location::Path(path) => match validate::path(&path) {
            Ok(()) => {
                let schema = fs::read_to_string(path).expect("Failed to read schema from path");

                Ok(schema)
            }
            Err(message) => Err(message),
        },
        Location::Url(url) => match validate::url(&url) {
            Ok(url) => {
                let schema = reqwest::blocking::get(url)
                    .expect("Failed to get response")
                    .text()
                    .expect("Failed to convert response to text");

                Ok(schema)
            }
            Err(message) => Err(message),
        },
    }
}
