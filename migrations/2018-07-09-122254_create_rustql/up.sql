CREATE TABLE IF NOT EXISTS countries (
    full_name varchar(100) NOT NULL,
    continent varchar(100) NOT NULL,
    short_name varchar(10) primary key
);

CREATE TABLE IF NOT EXISTS products (
    id varchar(100) primary key,
    name varchar(100) NOT NULL,
    price double precision NOT NULL,
    description varchar(250),
    country varchar(10)
);