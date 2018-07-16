use uuid::{Uuid, UuidVersion};
use super::schema::{products, countries};

/// Product schema structure
#[derive(Debug, PartialEq, GraphQLObject, Clone, Queryable, Insertable, AsChangeset)]
#[graphql(description="Product structure")]
#[table_name = "products"]
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub description: Option<String>,
    pub country: Option<String>,
}

/// Product schema structure for input data (mutations)
#[derive(Clone, GraphQLInputObject)]
#[graphql(description="Input product structure")]
pub struct ProductInput {
    name: String,
    description: Option<String>,
    price: f64,
    country: Option<String>,
}

impl ProductInput {
    /// Generate a Product instance with a UUID from an input product
    pub fn to_product(&self) -> Product {
        let uuid = Uuid::new(UuidVersion::Sha1).unwrap().to_string();

        Product {
            id: uuid,
            name: self.name.clone(),
            price: self.price.clone(),
            description: self.description.clone(),
            country: self.country.clone()
        }
    }
}


/// Country schema structure
#[derive(Debug, PartialEq, GraphQLObject, Clone, Queryable, Insertable, AsChangeset)]
#[graphql(description="Country structure")]
#[table_name = "countries"]
pub struct Country {
    full_name: String,
    continent: String,
    short_name: String,
}

/// Country schema structure for input data (mutations)
#[derive(Clone, GraphQLInputObject)]
#[graphql(description="Country structure")]
pub struct CountryInput {
    full_name: String,
    continent: String,
    short_name: String,
}

impl CountryInput {
    /// Generate a country object from a country input
    pub fn to_country(&self) -> Country {
        Country {
            full_name: self.full_name.clone(),
            continent: self.continent.clone(),
            short_name: self.short_name.clone()
        }
    }
}