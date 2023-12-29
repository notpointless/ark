-- This table stores information about various identity roles.
CREATE TABLE role (
    id INTEGER GENERATED ALWAYS AS IDENTITY,
    role_name VARCHAR(255) NOT NULL UNIQUE,
    PRIMARY KEY(id)
);

-- This table stores the basic information about each permission.
CREATE TABLE permission (
    id INTEGER GENERATED ALWAYS AS IDENTITY,
    permission_name VARCHAR(255) NOT NULL UNIQUE,
    permission_key VARCHAR(255) NOT NULL UNIQUE,
    PRIMARY KEY (id)
);

-- This table links user with specific roles
CREATE TABLE user_role (
    user_id VARCHAR(255),
    role_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, role_id)
);

-- This table stores information about the user
CREATE TABLE users (
    id INTEGER,
    username VARCHAR(255) UNIQUE,
    email VARCHAR(255) UNIQUE,
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'UTC'),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'UTC'),
    PRIMARY KEY(id)
);

-- This table links roles with permissions, enabling a many-to-many 
-- relationship where a role can have multiple permissions, and a 
-- permission can belong to multiple roles.
CREATE TABLE role_permission (
    role_id INTEGER REFERENCES role(id),
    permission_id INTEGER REFERENCES permission(id),
    PRIMARY KEY (role_id, permission_id)
);

-- This table links identities with permissions, enabling a many-to-many 
-- relationship where a identity can specific permissions for themselves.
CREATE TABLE user_permission (
    user_id INTEGER REFERENCES users(id),
    permission_id INTEGER REFERENCES permission(id),
    PRIMARY KEY (user_id, permission_id)
);

/*
CREATE TABLE user_oauth (
    user_id INTEGER NOT NULL,
    oauth_id VARCHAR(255) NOT NULL,
    oauth_provider VARCHAR(255) NOT NULL,
    UNIQUE(oauth_id, oauth_provider),
    FOREIGN KEY (user_id) REFERENCES users(id),
    PRIMARY KEY(user_id)
);
*/
