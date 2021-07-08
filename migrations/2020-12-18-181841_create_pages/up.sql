CREATE TABLE IF NOT EXISTS pages (
    id int NOT NULL PRIMARY KEY AUTO_INCREMENT,
    page_name varchar(500) NOT NULL,
    page_url varchar(100) NOT NULL,
    page_title varchar(500) NOT NULL,
    time_created TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

INSERT IGNORE INTO pages (page_name, page_url, page_title) VALUES ("index", "/", "Hello world.");

CREATE TABLE IF NOT EXISTS module_types (
    module_type_id int AUTO_INCREMENT PRIMARY KEY NOT NULL,
    title varchar(500) NOT NULL,
    module_desc varchar(500) NOT NULL
);

/* Doing each in a separate statement since it's more readable. */
INSERT IGNORE INTO module_types (title, module_desc) VALUES ('paragraph', 'A paragraph module for general text.');
INSERT IGNORE INTO module_types (title, module_desc) VALUES ('header', 'A header module for displaying things in large text.');
INSERT IGNORE INTO module_types (title, module_desc) VALUES ('image', 'Allows for inserting images into the page.');

CREATE TABLE IF NOT EXISTS modules (
    module_id int AUTO_INCREMENT PRIMARY KEY NOT NULL,
    module_type_id int NOT NULL,
    title varchar(100) NOT NULL,
    page_id int NOT NULL,
    content TEXT NOT NULL,
    FOREIGN KEY (page_id) REFERENCES pages(id) ON DELETE CASCADE,
    FOREIGN KEY (module_type_id) REFERENCES module_types(module_type_id) ON DELETE CASCADE
);

INSERT IGNORE INTO modules (module_type_id, title, page_id, content) VALUES (1, "Hello world!", 1, "Hello world!");

CREATE TABLE IF NOT EXISTS web_config (
    config_key VARCHAR(100) PRIMARY KEY NOT NULL,
    config_val VARCHAR(100) NOT NULL
);

INSERT IGNORE INTO web_config (config_key, config_val) VALUES ("setup", "start");