CREATE TABLE users (
    id INTEGER NOT NULL PRIMARY KEY AUTO_INCREMENT,
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    password_less_login BOOLEAN DEFAULT false,
    password_reset_token VARCHAR(255),
    can_login BOOLEAN DEFAULT true,
    is_active BOOLEAN DEFAULT true,
    created INTEGER UNSIGNED NOT NULL,
    created_by VARCHAR(50) DEFAULT 'sql',
    updated INTEGER UNSIGNED NOT NULL,
    updated_by VARCHAR(50) DEFAULT 'sql'
)