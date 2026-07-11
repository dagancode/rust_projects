use chrono::NaiveDate;
use sqlx::PgPool;
use tracing::{info, warn};
use uuid::Uuid;

use crate::models::domain::{PropertyDetail, PropertyListing};
use crate::models::error::ApiError;

pub async fn seed_sales_history(pool: &PgPool, records: &[PropertyDetail]) -> Result<(), ApiError> {
    // 1. check if table is empty
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM property_detail")
        .fetch_one(pool)
        .await?;

    // 2. if not empty, return early
    if count.0 > 0 {
        info!("property_detail table already has data, skipping seed");
        return Ok(());
    }

    // 3. begin transaction
    let mut tx = pool.begin().await?;

    // 4. for each PropertyDetail, insert property_detail row
    for record in records {
        sqlx::query!(
            r#"INSERT INTO property_detail 
                (id, street_number, street_name, neighbourhood, suburb, city, province, source_url)
            VALUES 
                ($1, $2, $3, $4, $5, $6, $7, $8)"#,
            record.property.id,
            record.property.location.street_number,
            record.property.location.street_name,
            record.property.location.neighbourhood,
            record.property.location.suburb,
            record.property.location.city,
            record.property.location.province,
            record.property.location.source_url,
        )
        .execute(&mut *tx)
        .await?;

        // 5. for each sale in sales_history, insert property_sales_history row
        for sale in &record.sales_history {
            sqlx::query!(
                r#"INSERT INTO property_sales_history 
                    (id, property_id, year, price)
                VALUES 
                    ($1, $2, $3, $4)"#,
                sale.id,
                record.property.id,
                sale.year as i32,
                sale.price,
            )
            .execute(&mut *tx)
            .await?;
        }
    }

    // 6. commit transaction
    tx.commit().await?;

    Ok(())
}

pub async fn seed_listings(pool: &PgPool, records: &[PropertyListing]) -> Result<(), ApiError> {
    // 1. check if table is empty
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM property_listings")
        .fetch_one(pool)
        .await?;

    // 2. if not empty, return early
    if count.0 > 0 {
        info!("property_listings table already has data, skipping seed");
        return Ok(());
    }

    // 3. begin transaction
    let mut tx = pool.begin().await?;

    // 4. for each PropertyDetail, insert property_listings row
    for record in records {
        sqlx::query!(
            r#"
            INSERT INTO property_listings (
                id, 
                source_url, 
                title, 
                price, 
                address, 
                property_type, 
                listing_date, 
                erf_size_m2, 
                floor_size_m2, 
                price_per_m2, 
                levies, 
                rates_and_taxes, 
                bedrooms, 
                bedroom_detail, 
                bathrooms, 
                kitchens, 
                lounges, 
                dining_rooms, 
                parking, 
                garage, 
                pool, 
                garden, 
                pet_friendly, 
                facing, 
                roof, 
                wall, 
                floor, 
                internet_access, 
                key_features
                )
            VALUES 
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29)
                "#,
            Uuid::new_v4(),
            record.source_url,
            record.title,
            record.price,
            record.address,
            record.property_type.to_string(),
            NaiveDate::try_from(record.listing_date).map_err(|e| warn!("Failed to parse listing date: {}", e)).ok(),
            record.erf_size_m2.map(|v| v as i32),
            record.floor_size_m2.map(|v| v as i32), 
            record.price_per_m2, 
            record.levies, 
            record.rates_and_taxes, 
            record.bedrooms.map(|v| v as i16), 
            record.bedroom_detail, 
            record.bathrooms.map(|v| v as i16), 
            record.kitchens.map(|v| v as i16), 
            record.lounges.map(|v| v as i16), 
            record.dining_rooms.map(|v| v as i16), 
            record.parking.map(|v| v as i16), 
            record.garage.map(|v| v as i16), 
            record.pool, 
            record.garden, 
            record.pet_friendly, 
            record.facing, 
            record.roof, 
            record.wall, 
            record.floor, 
            record.internet_access, 
            record.key_features
        )
        .execute(&mut *tx)
        .await?;
    }

    // 5. commit transaction
    tx.commit().await?;

    Ok(())
}

pub async fn truncate_listings(pool: &PgPool) -> Result<(), ApiError> {
    let mut tx = pool.begin().await?;

    sqlx::query!("TRUNCATE TABLE property_listings")
        .execute(&mut *tx)
        .await?;

    tx.commit()
        .await
        .map_err(|e| ApiError::ParseError(Some(e.to_string())))?;

    Ok(())
}

pub async fn truncate_sales_history(pool: &PgPool) -> Result<(), ApiError> {
    let mut tx = pool.begin().await?;

    sqlx::query!("TRUNCATE TABLE property_sales_history, property_detail")
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(())
}

pub async fn upsert_sales_history(
    pool: &PgPool,
    sales_history: &Vec<PropertyDetail>,
) -> Result<(), ApiError> {
    let mut tx = pool.begin().await?;

    for record in sales_history {
        let existing: Option<(uuid::Uuid,)> = sqlx::query_as(
            r#"
                SELECT id FROM property_detail 
                WHERE street_number = $1 
                    AND street_name = $2 
                    AND neighbourhood = $3 
                    AND suburb = $4 
                    AND city = $5 
                    AND province = $6
                "#,
        )
        .bind(&record.property.location.street_number)
        .bind(&record.property.location.street_name)
        .bind(&record.property.location.neighbourhood)
        .bind(&record.property.location.suburb)
        .bind(&record.property.location.city)
        .bind(&record.property.location.province)
        .fetch_optional(&mut *tx)
        .await?;

        match existing {
            Some(property_id) => {
                for sale in &record.sales_history {
                    let existing_sale: Option<(uuid::Uuid,)> = sqlx::query_as(
                    r#"
                        SELECT id FROM property_sales_history
                        WHERE property_id = $1
                            AND year = $2
                            AND price = $3
                        "#
                        )
                        .bind(property_id.0)
                        .bind(sale.year as i32)
                        .bind(sale.price)
                        .fetch_optional(&mut *tx)
                        .await?;
                    
                    match existing_sale {
                        Some(_) => continue,
                        None => {
                                    sqlx::query!(
                                        r#"
                                        INSERT INTO property_sales_history 
                                            (id, property_id, year, price)
                                        VALUES 
                                            ($1, $2, $3, $4)
                                        "#,
                                        sale.id,
                                        property_id.0,
                                        sale.year as i32,
                                        sale.price,
                                    )
                                    .execute(&mut *tx)
                                    .await?;
                        }
                    }
                }
            }
            None => {
                // 1. for each PropertyDetail, insert property_detail row
                sqlx::query!(
                    r#"
                    INSERT INTO property_detail 
                        (id, street_number, street_name, neighbourhood, suburb, city, province, source_url)
                    VALUES 
                        ($1, $2, $3, $4, $5, $6, $7, $8)
                    "#,
                    record.property.id,
                    record.property.location.street_number,
                    record.property.location.street_name,
                    record.property.location.neighbourhood,
                    record.property.location.suburb,
                    record.property.location.city,
                    record.property.location.province,
                    record.property.location.source_url,
                )
                .execute(&mut *tx)
                .await?;

                // 2. for each sale in sales_history, insert property_sales_history row
                for sale in &record.sales_history {
                    sqlx::query!(
                        r#"
                        INSERT INTO property_sales_history 
                            (id, property_id, year, price)
                        VALUES 
                            ($1, $2, $3, $4)
                        "#,
                        sale.id,
                        record.property.id,
                        sale.year as i32,
                        sale.price,
                    )
                    .execute(&mut *tx)
                    .await?;
                }
            }
        }
    }

    tx.commit().await?;

    Ok(())
}

pub async fn upsert_listings(pool: &PgPool, listings: &Vec<PropertyListing>) -> Result<(), ApiError> {
    let mut tx = pool.begin().await?;

    for record in listings {
        let existing_listing: Option<(uuid::Uuid,)> = sqlx::query_as(
        r#"
            SELECT id FROM property_listings
            WHERE source_url = $1
            "#
            )
            .bind(record.source_url.clone())
            .fetch_optional(&mut *tx)
            .await?;

        match existing_listing {
            Some(_) => continue,
            None => {
                sqlx::query!(
                    r#"
                    INSERT INTO property_listings (
                        id, 
                        source_url, 
                        title, 
                        price, 
                        address, 
                        property_type, 
                        listing_date, 
                        erf_size_m2, 
                        floor_size_m2, 
                        price_per_m2, 
                        levies, 
                        rates_and_taxes, 
                        bedrooms, 
                        bedroom_detail, 
                        bathrooms, 
                        kitchens, 
                        lounges, 
                        dining_rooms, 
                        parking, 
                        garage, 
                        pool, 
                        garden, 
                        pet_friendly, 
                        facing, 
                        roof, 
                        wall, 
                        floor, 
                        internet_access, 
                        key_features
                        )
                    VALUES 
                        ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29)
                        "#,
                    Uuid::new_v4(),
                    record.source_url,
                    record.title,
                    record.price,
                    record.address,
                    record.property_type.to_string(),
                    NaiveDate::try_from(record.listing_date).map_err(|e| warn!("Failed to parse listing date: {}", e)).ok(),
                    record.erf_size_m2.map(|v| v as i32),
                    record.floor_size_m2.map(|v| v as i32), 
                    record.price_per_m2, 
                    record.levies, 
                    record.rates_and_taxes, 
                    record.bedrooms.map(|v| v as i16), 
                    record.bedroom_detail, 
                    record.bathrooms.map(|v| v as i16), 
                    record.kitchens.map(|v| v as i16), 
                    record.lounges.map(|v| v as i16), 
                    record.dining_rooms.map(|v| v as i16), 
                    record.parking.map(|v| v as i16), 
                    record.garage.map(|v| v as i16), 
                    record.pool, 
                    record.garden, 
                    record.pet_friendly, 
                    record.facing, 
                    record.roof, 
                    record.wall, 
                    record.floor, 
                    record.internet_access, 
                    record.key_features
                )
                .execute(&mut *tx)
                .await?;
            },
        }
    }


    tx.commit().await?;

    Ok(())
}
