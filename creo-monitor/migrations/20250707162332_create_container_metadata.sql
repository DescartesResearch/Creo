CREATE TABLE IF NOT EXISTS container_metadata (
    container_id BINARY(64) NOT NULL,
    machine_id BINARY(16) NOT NULL,
    hostname VARCHAR(64) NOT NULL,
    label_key VARCHAR(255) NOT NULL,
    label_value VARCHAR(255) NOT NULL,

    PRIMARY KEY (container_id, machine_id, label_key)
)
