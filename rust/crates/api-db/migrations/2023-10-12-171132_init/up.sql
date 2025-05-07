-- Your SQL goes here

create TABLE
    province(
        id SERIAL PRIMARY KEY NOT NULL,
        name VARCHAR(255) NOT NULL
    );

create TABLE
    city(
        id SERIAL PRIMARY KEY NOT NULL,
        name VARCHAR(255) NOT NULL,
        province_id INTEGER NOT NULL,
        FOREIGN KEY (province_id) REFERENCES province(id)
    );

create TABLE
    role(
        id SERIAL PRIMARY KEY NOT NULL,
        name VARCHAR(255) NOT NULL
    );

create TABLE
    user_group(
        id SERIAL PRIMARY KEY NOT NULL,
        name VARCHAR(255) NOT NULL,
        description VARCHAR(255),
        parent_id INTEGER
    );

create TABLE
    users(
        id SERIAL PRIMARY KEY NOT NULL,
        name VARCHAR(255) NOT NULL,
        email VARCHAR(255) NOT NULL,
        password VARCHAR(255) NOT NULL,
        image_url VARCHAR(255),
        role_id INTEGER NOT NULL,
        user_group_id INTEGER,
        device_ids INTEGER [] NOT NULL,
        city_id INTEGER,
        FOREIGN KEY (role_id) REFERENCES role(id),
        FOREIGN KEY (user_group_id) REFERENCES user_group(id),
        FOREIGN KEY (city_id) REFERENCES city(id)
    );

create TABLE
    package(
        id SERIAL PRIMARY KEY NOT NULL,
        name VARCHAR(255) NOT NULL,
        max_devices INTEGER NOT NULL
    );

create TABLE
    subscription(
        id SERIAL PRIMARY KEY NOT NULL,
        user_id INTEGER NOT NULL,
        package_id INTEGER NOT NULL,
        order_date TIMESTAMP NOT NULL,
        expire_date TIMESTAMP NOT NULL,
        FOREIGN KEY (user_id) REFERENCES users(id),
        FOREIGN KEY (package_id) REFERENCES package(id)
    );

create TABLE
    blacklist_token (
        id SERIAL PRIMARY KEY,
        token TEXT UNIQUE NOT NULL
    );

create TABLE
    avs (
        id SERIAL PRIMARY KEY,
        unique_id VARCHAR(255) UNIQUE NOT NULL,
        status INTEGER NOT NULL,
        lat DOUBLE PRECISION,
        lng DOUBLE PRECISION,
        address VARCHAR(255),
        description VARCHAR(255),
        kind INTEGER NOT NULL
    );

create TABLE
    avs_port (
        id SERIAL PRIMARY KEY,
        avs_id INTEGER NOT NULL,
        name VARCHAR(255) NOT NULL,
        speaker INTEGER NOT NULL,
        FOREIGN KEY (avs_id) REFERENCES avs(id)
    );

create TABLE
    web_session (
        id SERIAL PRIMARY KEY,
        user_id INTEGER NOT NULL,
        token TEXT UNIQUE NOT NULL
    );

create TABLE
    web_session_pending(
        id SERIAL PRIMARY KEY,
        user_id INTEGER NOT NULL,
        token TEXT UNIQUE NOT NULL
    );

create TABLE
    mobile_session (
        id SERIAL PRIMARY KEY,
        user_id INTEGER NOT NULL,
        token TEXT UNIQUE NOT NULL
    );

create TABLE
    mobile_session_pending(
        id SERIAL PRIMARY KEY,
        user_id INTEGER NOT NULL,
        token TEXT UNIQUE NOT NULL
    );

create TABLE
    verify(
        id SERIAL PRIMARY KEY,
        uuid TEXT UNIQUE NOT NULL,
        mobile INTEGER NOT NULL,
        session_id INTEGER NOT NULL,
        created_at TIMESTAMP NOT NULL
    );

create TABLE
    docs_credentials(
        id SERIAL PRIMARY KEY,
        username VARCHAR(255) UNIQUE NOT NULL,
        password VARCHAR(255) NOT NULL
    );

create TABLE
    forgot_password(
        id SERIAL PRIMARY KEY,
        uuid VARCHAR(255) UNIQUE NOT NULL,
        user_id INTEGER NOT NULL,
        created_at TIMESTAMP NOT NULL
    );

insert into role (id, name) values (1, 'SuperAdmin');

insert into role (id, name) values (2, 'Admin');

insert into package (name, max_devices) values ('Lite', 30);

insert into package (name, max_devices) values ('Basic', 50);

insert into package (name, max_devices) values ('Premium', 90);

insert into package (name, max_devices) values ('Professional', 150);