-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Teams table
CREATE TABLE teams (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(50) UNIQUE NOT NULL,
    description TEXT,
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Team members table
CREATE TABLE team_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role VARCHAR(20) NOT NULL DEFAULT 'member',
    joined_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(team_id, user_id)
);

-- Packages table
CREATE TABLE packages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL CHECK (name ~ '^@[a-z0-9][a-z0-9\-]*(/[a-z0-9][a-z0-9\-]*)?$'), -- Must start with @ and follow npm naming
    version VARCHAR(20) NOT NULL CHECK (version ~ '^[0-9]+\.[0-9]+\.[0-9]+.*$'), -- Semantic versioning
    description TEXT,
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    team_id UUID REFERENCES teams(id) ON DELETE SET NULL,
    download_url VARCHAR(500) NOT NULL,
    file_path VARCHAR(500) NOT NULL, -- Actual file path on disk
    file_size BIGINT NOT NULL CHECK (file_size > 0 AND file_size <= 104857600), -- Max 100MB
    checksum_sha256 VARCHAR(64) NOT NULL, -- SHA256 checksum
    downloads_count BIGINT DEFAULT 0,
    published_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    -- Ensure package name matches team ownership
    CONSTRAINT valid_team_package CHECK (
        (team_id IS NULL AND name NOT LIKE '@%/%') OR 
        (team_id IS NOT NULL AND name LIKE '@%/%')
    ),
    UNIQUE(name, version)
);

-- Indexes for better performance
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_teams_name ON teams(name);
CREATE INDEX idx_teams_owner ON teams(owner_id);
CREATE INDEX idx_team_members_team ON team_members(team_id);
CREATE INDEX idx_team_members_user ON team_members(user_id);
CREATE INDEX idx_packages_name ON packages(name);
CREATE INDEX idx_packages_owner ON packages(owner_id);
CREATE INDEX idx_packages_team ON packages(team_id);
CREATE INDEX idx_packages_published ON packages(published_at DESC);
CREATE INDEX idx_packages_downloads ON packages(downloads_count DESC);

-- Add constraints for team names (must be lowercase alphanumeric with hyphens)
ALTER TABLE teams ADD CONSTRAINT valid_team_name CHECK (name ~ '^[a-z0-9][a-z0-9\-]*$');

-- Add constraints for usernames 
ALTER TABLE users ADD CONSTRAINT valid_username CHECK (username ~ '^[a-zA-Z0-9][a-zA-Z0-9\-_]*$');

-- Add role constraints
ALTER TABLE team_members ADD CONSTRAINT valid_role CHECK (role IN ('owner', 'admin', 'member'));

-- Package tags table
CREATE TABLE package_tags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    package_id UUID NOT NULL REFERENCES packages(id) ON DELETE CASCADE,
    tag VARCHAR(50) NOT NULL CHECK (tag ~ '^[a-z0-9][a-z0-9\-]*$'), -- lowercase alphanumeric with hyphens
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(package_id, tag)
);

-- Indexes for tags
CREATE INDEX idx_package_tags_package ON package_tags(package_id);
CREATE INDEX idx_package_tags_tag ON package_tags(tag);
CREATE INDEX idx_package_tags_tag_lower ON package_tags(lower(tag));