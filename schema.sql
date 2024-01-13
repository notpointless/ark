-- This table stores information about various identity roles.
CREATE TABLE iam_roles (
    id INTEGER,
    role_name VARCHAR(255) NOT NULL UNIQUE,
    PRIMARY KEY(id)
);

-- This table stores the basic information about each permission.
CREATE TABLE iam_permissions (
    id INTEGER,
    permission_name VARCHAR(255) NOT NULL UNIQUE,
    permission_key VARCHAR(255) NOT NULL UNIQUE,
    PRIMARY KEY (id)
);

-- This table links user with specific roles
CREATE TABLE iam_user_role (
    user_id VARCHAR(255),
    role_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, role_id)
);

-- This table stores information about the user
CREATE TABLE iam_users (
    id VARCHAR(255),
    username VARCHAR(255) UNIQUE,
    email VARCHAR(255) UNIQUE,
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    created_at BIGINT NOT NULL DEFAULT (EXTRACT(EPOCH FROM NOW()) * 1000)::BIGINT,
    updated_at BIGINT NOT NULL DEFAULT (EXTRACT(EPOCH FROM NOW()) * 1000)::BIGINT,
    PRIMARY KEY(id)
);

-- This table links roles with permissions, enabling a many-to-many 
-- relationship where a role can have multiple permissions, and a 
-- permission can belong to multiple roles.
CREATE TABLE iam_role_permission (
    role_id INTEGER REFERENCES iam_roles(id),
    permission_id INTEGER REFERENCES iam_permissions(id),
    PRIMARY KEY (role_id, permission_id)
);

-- This table links identities with permissions, enabling a many-to-many 
-- relationship where a identity can specific permissions for themselves.
CREATE TABLE iam_user_permission (
    user_id VARCHAR(255) REFERENCES iam_users(id),
    permission_id INTEGER REFERENCES iam_permissions(id),
    PRIMARY KEY (user_id, permission_id)
);


CREATE TABLE iam_user_oauth (
    user_id VARCHAR(255) NOT NULL,
    oauth_id VARCHAR(255) NOT NULL,
    oauth_provider VARCHAR(255) NOT NULL,
    UNIQUE(oauth_id, oauth_provider),
    FOREIGN KEY (user_id) REFERENCES iam_users(id),
    PRIMARY KEY(user_id)
);
