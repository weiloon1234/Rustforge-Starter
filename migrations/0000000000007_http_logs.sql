
CREATE TABLE IF NOT EXISTS webhook_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    request_url TEXT NOT NULL,
    request_method TEXT NOT NULL,
    request_headers JSONB,
    request_body TEXT,
    response_status INT,
    response_body TEXT,
    duration_ms INT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_webhook_logs_created_at ON webhook_logs(created_at);
CREATE INDEX IF NOT EXISTS idx_webhook_logs_url ON webhook_logs(request_url);
CREATE INDEX IF NOT EXISTS idx_webhook_logs_method ON webhook_logs(request_method);

CREATE TABLE IF NOT EXISTS http_client_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    request_url TEXT NOT NULL,
    request_method TEXT NOT NULL,
    request_headers JSONB,
    request_body TEXT,
    response_status INT,
    response_headers JSONB,
    response_body TEXT,
    duration_ms INT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_http_client_logs_created_at ON http_client_logs(created_at);
CREATE INDEX IF NOT EXISTS idx_http_client_logs_url ON http_client_logs(request_url);
CREATE INDEX IF NOT EXISTS idx_http_client_logs_method ON http_client_logs(request_method);
CREATE INDEX IF NOT EXISTS idx_http_client_logs_status ON http_client_logs(response_status);
