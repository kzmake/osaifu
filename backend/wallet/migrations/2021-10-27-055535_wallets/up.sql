CREATE TABLE wallets (
    id VARCHAR(64) PRIMARY KEY,
    owner VARCHAR(64) NOT NULL,
    balance TEXT NOT NULL,
    create_at timestamptz NOT NULL DEFAULT current_timestamp,
    update_at timestamptz NOT NULL DEFAULT current_timestamp,
    delete_at timestamptz DEFAULT NULL
);
INSERT INTO wallets (id, owner, balance)
VALUES (
        '0123456789ABCDEFGHJKMNPQRSTVWXYZ',
        'alice',
        '12345678'
    );
