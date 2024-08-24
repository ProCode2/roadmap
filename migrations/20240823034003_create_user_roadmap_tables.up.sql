-- Add up migration script here
-- Table for storing users
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL, -- Email field with unique constraint
    password_digest VARCHAR(255) NOT NULL, -- Password digest (hashed password)
    name VARCHAR(255) NOT NULL, -- Name of the user
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, -- Timestamp with timezone
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP -- Automatically updated timestamp
);

-- Table for storing tags
CREATE TABLE tag (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL
);

-- Table for storing maps
CREATE TABLE map (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE SET NULL,
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    description TEXT NOT NULL,
    keywords VARCHAR(255)[], -- Array of strings for keywords
    content JSONB NOT NULL, -- JSON string for content
    sources VARCHAR(255)[], -- Array of strings for source links
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, -- Timestamp with timezone
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP -- Automatically updated timestamp
);

-- Table for storing map -> tag joins
CREATE TABLE map_tag (
    id SERIAL PRIMARY KEY,
    map_id INTEGER NOT NULL REFERENCES map(id) ON DELETE CASCADE,
    tag_id INTEGER NOT NULL REFERENCES tag(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Create a trigger to automatically update the updated_at column on update
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
   NEW.updated_at = NOW();
   RETURN NEW;
END;
$$ LANGUAGE 'plpgsql';

CREATE TRIGGER update_blog_post_updated_at
BEFORE UPDATE ON map 
FOR EACH ROW
EXECUTE PROCEDURE update_updated_at_column();

CREATE TRIGGER update_user_updated_at
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE PROCEDURE update_updated_at_column();
