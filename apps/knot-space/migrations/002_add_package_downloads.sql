-- Package downloads analytics table
CREATE TABLE package_downloads (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    package_id UUID NOT NULL REFERENCES packages(id) ON DELETE CASCADE,
    package_name VARCHAR(100) NOT NULL,
    version VARCHAR(20) NOT NULL,
    ip_hash VARCHAR(64), -- Hashed IP for privacy
    user_agent TEXT,
    download_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Add indexes for efficient querying
CREATE INDEX idx_package_downloads_package_id ON package_downloads(package_id);
CREATE INDEX idx_package_downloads_package_name ON package_downloads(package_name);
CREATE INDEX idx_package_downloads_download_at ON package_downloads(download_at);
CREATE INDEX idx_package_downloads_package_id_download_at ON package_downloads(package_id, download_at);