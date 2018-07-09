use uuid::{Uuid, UuidVersion};

/// Product schema structure
#[derive(Clone, GraphQLObject, Debug, Queryable)]
#[graphql(description="Product structure")]
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub description: String,
    pub available: bool,
}

/// Product schema structure for input data (mutations)
#[derive(Clone, GraphQLInputObject)]
#[graphql(description="Input product structure")]
pub struct ProductInput {
    name: String,
    description: String,
    price: f64,
    available: bool,
}

impl ProductInput {
    /// Generate a Product instance with a UUID from an input product
    fn to_product(&self) -> Product {
        let uuid = Uuid::new(UuidVersion::Sha1).unwrap().to_string();

        Product {
            id: uuid,
            name: self.name.clone(),
            price: self.price.clone(),
            description: self.description.clone(),
            available: self.available.clone()
        }
    }
}


/// Country schema structure
#[derive(Clone, GraphQLObject, Queryable)]
#[graphql(description="Country structure")]
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
    fn _to_country(&self) -> Country {
        Country {
            full_name: self.full_name.clone(),
            continent: self.continent.clone(),
            short_name: self.short_name.clone()
        }
    }
}