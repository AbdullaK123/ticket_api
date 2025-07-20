-- Add migration script here

CREATE TABLE tickets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(50) NOT NULL,
    description VARCHAR(500) NOT NULL,
    status VARCHAR NOT NULL CHECK (status IN ('To Do', 'In Progress', 'Completed')),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
)