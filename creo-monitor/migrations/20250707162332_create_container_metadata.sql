CREATE TABLE IF NOT EXISTS container_metadata (
    container_id BINARAY(64) NOT NULL,
    machine_id BINARY(16) NOT NULL,
    hostname VARCHAR(64) NOT NULL,
    key VARCHAR(255) NOT NULL,
    value VARCHAR(255) NOT NULL,

    PRIMARY KEY (container_id, machine_id)
)
