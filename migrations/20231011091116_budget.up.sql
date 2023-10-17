-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS income (
        id SERIAL PRIMARY KEY NOT NULL,
        name VARCHAR(255) NOT NULL,
        amount INT8 NOT NULL,
        done BOOLEAN DEFAULT false
    );

CREATE TABLE
    IF NOT EXISTS expenses(
        id SERIAL PRIMARY KEY NOT NULL,
        name VARCHAR(255) NOT NULL,
        amount INT8 NOT NULL,
        done BOOLEAN DEFAULT false
    );

CREATE TABLE
    IF NOT EXISTS savings(
        id SERIAL PRIMARY KEY NOT NULL,
        name VARCHAR(255) NOT NULL,
        VALUE INT8 NOT NULL,
        yld INT8 NOT NULL
    );

CREATE TABLE
    IF NOT EXISTS assets(
        id SERIAL PRIMARY KEY NOT NULL,
        name VARCHAR(255) NOT NULL,
        value INT8 NOT NULL,
        yld INT8 NOT NULL
    );