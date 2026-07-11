
CREATE TABLE property_detail (
    id UUID NOT NULL,
    street_number VARCHAR(16) NOT NULL,
    street_name VARCHAR(128) NOT NULL,
    neighbourhood VARCHAR(128) NOT NULL,
    suburb VARCHAR(128) NOT NULL,
    city VARCHAR(128) NOT NULL,
    province VARCHAR(128) NOT NULL,
    source_url VARCHAR(256) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE property_sales_history (
    id UUID NOT NULL,
    property_id UUID NOT NULL,
    year INT NOT NULL,
    price DECIMAL(10, 2) NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (property_id) REFERENCES property_detail(id)
);

CREATE TABLE property_listings (
    id UUID NOT NULL,
    source_url VARCHAR(128) NOT NULL,
    title VARCHAR(128) NOT NULL,
    price DECIMAL(10, 2) NOT NULL,
    address VARCHAR(128) NOT NULL,
    property_type VARCHAR(128) NOT NULL,
    listing_date DATE NOT NULL,
    erf_size_m2 INT,
    floor_size_m2 INT,
    price_per_m2 DECIMAL(10, 2),
    levies DECIMAL(10, 2),
    rates_and_taxes DECIMAL(10, 2),
    bedrooms SMALLINT,
    bedroom_detail VARCHAR(128),
    bathrooms SMALLINT,
    kitchens SMALLINT,
    lounges SMALLINT,
    dining_rooms SMALLINT,
    parking SMALLINT,
    garage SMALLINT,
    pool BOOL,
    garden BOOL,
    pet_friendly BOOL,
    facing VARCHAR(128),
    roof VARCHAR(128),
    wall VARCHAR(128),
    floor VARCHAR(128),
    internet_access VARCHAR(128),
    key_features VARCHAR(128),
    PRIMARY KEY (id)
);