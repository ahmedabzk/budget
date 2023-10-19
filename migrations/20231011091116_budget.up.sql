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

CREATE TABLE 
    IF NOT EXISTS users(
        id SERIAL PRIMARY KEY NOT NULL,
        email VARCHAR(255) NOT NULL UNIQUE,
        created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
        last_updated TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

CREATE TABLE
    IF NOT EXISTS user_sessions(
        id SERIAL PRIMARY KEY NOT NULL,
        user_id INT NOT NULL UNIQUE,
        session_token_p1 text NOT NULL,
        session_token_p2 text NOT NULL,
        expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
        FOREIGN KEY (user_id) REFERENCES users(id)
    );

CREATE TABLE "oauth2_state_storage" (
    id SERIAL NOT NULL PRIMARY KEY,
    csrf_state text NOT NULL,
    pkce_code_verifier text NOT NULL,
    return_url text NOT NULL
);