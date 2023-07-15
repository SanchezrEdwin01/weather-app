-- Your SQL goes here
CREATE TABLE weather (
    id SERIAL PRIMARY KEY,
    city VARCHAR(255) NOT NULL,
    temperature DECIMAL(5, 2) NOT NULL,
    description VARCHAR(255) NOT NULL,
    humidity DECIMAL(5, 2) NOT NULL,
    wind VARCHAR(255),
    visibility DECIMAL(5, 2),
    atmospheric_pressure DECIMAL(7, 2),
    sunrise TIME,
    sunset TIME,
    date_time TIMESTAMP NOT NULL
);
