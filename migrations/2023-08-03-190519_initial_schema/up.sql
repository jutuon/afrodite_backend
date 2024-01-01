-- Your SQL goes here

---------- Tables for server component common ----------

-- UUID for account
CREATE TABLE IF NOT EXISTS account_id(
    id    INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    uuid  BLOB                              NOT NULL  UNIQUE
);

-- API access token for account
CREATE TABLE IF NOT EXISTS access_token(
    account_id   INTEGER PRIMARY KEY NOT NULL,
    token        TEXT                          UNIQUE,
    FOREIGN KEY (account_id)
        REFERENCES account_id (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
);

-- API refresh token for account
CREATE TABLE IF NOT EXISTS refresh_token(
    account_id    INTEGER PRIMARY KEY NOT NULL,
    token         BLOB                          UNIQUE,
    FOREIGN KEY (account_id)
        REFERENCES account_id (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
);

-- Account capabilities are shared between server components.
-- If the data is located in this table it should be set through account
-- server as it propagates the changes to other components.
CREATE TABLE IF NOT EXISTS account_capabilities(
    account_id    INTEGER PRIMARY KEY NOT NULL,
    admin_modify_capabilities                    BOOLEAN NOT NULL DEFAULT 0,
    admin_moderate_profiles                      BOOLEAN NOT NULL DEFAULT 0,
    admin_moderate_images                        BOOLEAN NOT NULL DEFAULT 0,
    admin_view_all_profiles                      BOOLEAN NOT NULL DEFAULT 0,
    admin_view_private_info                      BOOLEAN NOT NULL DEFAULT 0,
    admin_view_profile_history                   BOOLEAN NOT NULL DEFAULT 0,
    admin_server_maintenance_view_info           BOOLEAN NOT NULL DEFAULT 0,
    admin_server_maintenance_view_backend_config BOOLEAN NOT NULL DEFAULT 0,
    admin_server_maintenance_update_software     BOOLEAN NOT NULL DEFAULT 0,
    admin_server_maintenance_reset_data          BOOLEAN NOT NULL DEFAULT 0,
    admin_server_maintenance_reboot_backend      BOOLEAN NOT NULL DEFAULT 0,
    admin_server_maintenance_save_backend_config BOOLEAN NOT NULL DEFAULT 0,
    user_view_public_profiles                    BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY (account_id)
        REFERENCES account_id (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
);

-- Shared state between server components.
-- If the data is located in this table it should be set through account
-- server as it propagates the changes to other components.
CREATE TABLE IF NOT EXISTS shared_state(
    account_id              INTEGER PRIMARY KEY NOT NULL,
    -- initial setup = 0
    -- normal = 1
    -- banned = 2
    -- pending deletion = 3
    account_state_number    INTEGER             NOT NULL DEFAULT 0,
    is_profile_public       BOOLEAN             NOT NULL DEFAULT 0,
    FOREIGN KEY (account_id)
        REFERENCES account_id (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
);

-- All next new queue numbers are stored here.
CREATE TABLE IF NOT EXISTS next_queue_number(
    -- Queue type number: 0 = media moderation
    -- Queue type number: 1 = initial media moderation
    queue_type_number       INTEGER PRIMARY KEY     NOT NULL,
    -- Next unused queue number
    next_number             INTEGER                 NOT NULL DEFAULT 0
);

-- Table for storing active queue entries.
-- Only active queue entries are stored here.
CREATE TABLE IF NOT EXISTS queue_entry(
    -- Queue number from next_queue_number table.
    -- The number in that table is incremented when
    -- new queue entry is created.
    queue_number      INTEGER                        NOT NULL,
    -- Queue entry type number. Check next_queue_number table for
    -- available queue type numbers.
    queue_type_number INTEGER                        NOT NULL,
    -- Associate queue entry with account.
    account_id        INTEGER                        NOT NULL,
    PRIMARY KEY (queue_number, queue_type_number),
    FOREIGN KEY (account_id)
        REFERENCES account_id (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
);

---------- Tables for server component account ----------

-- Sign in with related IDs for account
CREATE TABLE IF NOT EXISTS sign_in_with_info(
    account_id         INTEGER PRIMARY KEY NOT NULL,
    google_account_id  TEXT                          UNIQUE,
    FOREIGN KEY (account_id)
        REFERENCES account_id (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
);

-- Account information which can change
CREATE TABLE IF NOT EXISTS account(
    account_id   INTEGER PRIMARY KEY NOT NULL,
    email        TEXT                NOT NULL  DEFAULT '',
    FOREIGN KEY (account_id)
        REFERENCES account_id (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
);

-- Account information which can not change after account initial setup completes
CREATE TABLE IF NOT EXISTS account_setup(
    account_id  INTEGER PRIMARY KEY NOT NULL,
    name        TEXT                NOT NULL  DEFAULT '',
    birthdate   TEXT                NOT NULL  DEFAULT '',
    FOREIGN KEY (account_id)
        REFERENCES account_id (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
);

---------- Tables for server component profile ----------

CREATE TABLE IF NOT EXISTS profile(
    account_id      INTEGER PRIMARY KEY NOT NULL,
    version_uuid    BLOB                NOT NULL,
    name            TEXT                NOT NULL    DEFAULT '',
    profile_text    TEXT                NOT NULL    DEFAULT '',
    FOREIGN KEY (account_id)
        REFERENCES account_id (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS profile_location(
    account_id      INTEGER PRIMARY KEY NOT NULL,
    latitude        DOUBLE              NOT NULL    DEFAULT 0.0,
    longitude       DOUBLE              NOT NULL    DEFAULT 0.0,
    FOREIGN KEY (account_id)
        REFERENCES account_id (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS favorite_profile(
    -- Account which marked the profile as a favorite.
    account_id          INTEGER               NOT NULL,
    -- Account which profile is marked as a favorite.
    favorite_account_id INTEGER               NOT NULL,
    -- Unix timestamp when favorite was added.
    unix_time           INTEGER               NOT NULL,
    PRIMARY KEY (account_id, favorite_account_id),
    FOREIGN KEY (account_id)
        REFERENCES account_id (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE,
    FOREIGN KEY (favorite_account_id)
        REFERENCES account_id (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
);

---------- Tables for server component media ----------

-- Currently selected images for account.
-- Contains profile editing related pending profile image info.
CREATE TABLE IF NOT EXISTS current_account_media(
    account_id           INTEGER PRIMARY KEY NOT NULL,
    security_content_id  INTEGER,
    profile_content_id_0 INTEGER,
    profile_content_id_1 INTEGER,
    profile_content_id_2 INTEGER,
    profile_content_id_3 INTEGER,
    profile_content_id_4 INTEGER,
    profile_content_id_5 INTEGER,
    -- Image's max square size multipler.
    -- Value 1.0 is the max size and the size of the original image.
    grid_crop_size       DOUBLE,
    -- X coordinate for square top left corner.
    -- Counted from top left corner of the original image.
    grid_crop_x          DOUBLE,
    -- Y coordinate for square top left corner.
    -- Counted from top left corner of the original image.
    grid_crop_y          DOUBLE,
    pending_security_content_id  INTEGER,
    pending_profile_content_id_0 INTEGER,
    pending_profile_content_id_1 INTEGER,
    pending_profile_content_id_2 INTEGER,
    pending_profile_content_id_3 INTEGER,
    pending_profile_content_id_4 INTEGER,
    pending_profile_content_id_5 INTEGER,
    pending_grid_crop_size       DOUBLE,
    pending_grid_crop_x          DOUBLE,
    pending_grid_crop_y          DOUBLE,
    FOREIGN KEY (account_id)
        REFERENCES account_id (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE,
    FOREIGN KEY (security_content_id)
        REFERENCES media_content (id)
            ON DELETE SET NULL
            ON UPDATE CASCADE,
    FOREIGN KEY (profile_content_id_0)
        REFERENCES media_content (id)
            ON DELETE SET NULL
            ON UPDATE CASCADE,
    FOREIGN KEY (profile_content_id_1)
        REFERENCES media_content (id)
            ON DELETE SET NULL
            ON UPDATE CASCADE,
    FOREIGN KEY (profile_content_id_2)
        REFERENCES media_content (id)
            ON DELETE SET NULL
            ON UPDATE CASCADE,
    FOREIGN KEY (profile_content_id_3)
        REFERENCES media_content (id)
            ON DELETE SET NULL
            ON UPDATE CASCADE,
    FOREIGN KEY (profile_content_id_4)
        REFERENCES media_content (id)
            ON DELETE SET NULL
            ON UPDATE CASCADE,
    FOREIGN KEY (profile_content_id_5)
        REFERENCES media_content (id)
            ON DELETE SET NULL
            ON UPDATE CASCADE,
    FOREIGN KEY (pending_security_content_id)
        REFERENCES media_content (id)
            ON DELETE SET NULL
            ON UPDATE CASCADE,
    FOREIGN KEY (pending_profile_content_id_0)
        REFERENCES media_content (id)
            ON DELETE SET NULL
            ON UPDATE CASCADE,
    FOREIGN KEY (pending_profile_content_id_1)
        REFERENCES media_content (id)
            ON DELETE SET NULL
            ON UPDATE CASCADE,
    FOREIGN KEY (pending_profile_content_id_2)
        REFERENCES media_content (id)
            ON DELETE SET NULL
            ON UPDATE CASCADE,
    FOREIGN KEY (pending_profile_content_id_3)
        REFERENCES media_content (id)
            ON DELETE SET NULL
            ON UPDATE CASCADE,
    FOREIGN KEY (pending_profile_content_id_4)
        REFERENCES media_content (id)
            ON DELETE SET NULL
            ON UPDATE CASCADE,
    FOREIGN KEY (pending_profile_content_id_5)
        REFERENCES media_content (id)
            ON DELETE SET NULL
            ON UPDATE CASCADE
);

-- Information about uploaded media content
CREATE TABLE IF NOT EXISTS media_content(
    id                  INTEGER PRIMARY KEY NOT NULL,
    uuid                BLOB                NOT NULL   UNIQUE,
    account_id          INTEGER             NOT NULL,
    -- InSlot = 0, If user uploads new content to slot the current will be removed.
    -- InModeration = 1, Content is in moderation. User can not remove the content.
    -- InModeration = 2, Content is moderated as accepted. User can not remove the content until
    --                   specific time elapses.
    -- ModeratedAsDenied = 3, Content is moderated as denied. Making new moderation request removes
    --                        the content.
    content_state       INTEGER             NOT NULL,
    -- Client captured this media
    secure_capture      BOOLEAN             NOT NULL,
    -- JpegImage = 0, Jpeg image
    content_type_number INTEGER             NOT NULL,
    -- Numbers from 0 to 6.
    slot_number         INTEGER             NOT NULL,
    FOREIGN KEY (account_id)
        REFERENCES account_id (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
);

-- User made moderation request.
-- If media moderation for one row exists, then prevent
-- modifications to that row.
CREATE TABLE IF NOT EXISTS media_moderation_request(
    id                  INTEGER PRIMARY KEY NOT NULL,
    -- Request owner Account ID. One request per account.
    account_id          INTEGER             NOT NULL  UNIQUE,
    -- Queue number which this media_moderation_request has.
    queue_number        INTEGER             NOT NULL,
    content_id_0        BLOB                NOT NULL,
    content_id_1        BLOB,
    content_id_2        BLOB,
    content_id_3        BLOB,
    content_id_4        BLOB,
    content_id_5        BLOB,
    content_id_6        BLOB,
    FOREIGN KEY (account_id)
        REFERENCES account_id (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
);

-- Admin made moderation
CREATE TABLE IF NOT EXISTS media_moderation(
    -- What admin account is moderating
    account_id              INTEGER NOT NULL,
    -- What request is in moderation
    moderation_request_id   INTEGER NOT NULL,
    -- State of the moderation
    state_number            INTEGER NOT NULL,
    PRIMARY KEY (account_id, moderation_request_id),
    FOREIGN KEY (account_id)
        REFERENCES account_id (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE,
    FOREIGN KEY (moderation_request_id)
        REFERENCES media_moderation_request (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
);

---------- Tables for server component chat ----------

-- Lookup table for finding interaction ID for a pair of accounts.
-- One account pair has two rows in this table, so accessing
-- with (a1, a2) and (a2, a1) is possible.
CREATE TABLE IF NOT EXISTS account_interaction_index(
    account_id_first               INTEGER NOT NULL,
    account_id_second              INTEGER NOT NULL,
    interaction_id                 INTEGER NOT NULL,
    PRIMARY KEY (account_id_first, account_id_second),
    FOREIGN KEY (account_id_first)
        REFERENCES account_id (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE,
    FOREIGN KEY (account_id_second)
        REFERENCES account_id (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE,
    FOREIGN KEY (interaction_id)
        REFERENCES account_interaction (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
);

-- Current relationship between accounts
CREATE TABLE IF NOT EXISTS account_interaction(
    id                  INTEGER PRIMARY KEY NOT NULL,
    -- 0 = no interaction
    -- 1 = like
    -- 2 = match
    -- 3 = block
    state_number                    INTEGER NOT NULL DEFAULT 0,
    -- The account which started the interaction (e.g. sent a like).
    -- Can be null for example if a like is removed afterwards.
    account_id_sender               INTEGER,
    -- The target of the interaction (e.g. received a like).
    -- Can be null for example if a like is removed afterwards.
    account_id_receiver             INTEGER,
    -- Incrementing counter for getting order number for conversation messages.
    message_counter                 INTEGER NOT NULL DEFAULT 0,
    -- Sender's latest viewed message number in the conversation.
    -- Can be null for example if account is blocked.
    sender_latest_viewed_message    INTEGER,
    -- Receivers's latest viewed message number in the conversation.
    -- Can be null for example if account is blocked.
    receiver_latest_viewed_message    INTEGER,
    FOREIGN KEY (account_id_sender)
        REFERENCES account_id (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE,
    FOREIGN KEY (account_id_receiver)
        REFERENCES account_id (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
);

-- Messages received from clients which are pending for sending.
CREATE TABLE IF NOT EXISTS pending_messages(
    id                  INTEGER PRIMARY KEY NOT NULL,
    -- The account which sent the message.
    account_id_sender               INTEGER NOT NULL,
    -- The account which will receive the message.
    account_id_receiver             INTEGER NOT NULL,
    -- Receiving time of the message.
    unix_time                       INTEGER NOT NULL,
    -- Order number for the message in the conversation.
    message_number                  INTEGER NOT NULL,
    -- Message text.
    message_text                    TEXT    NOT NULL,
    FOREIGN KEY (account_id_sender)
        REFERENCES account_id (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE,
    FOREIGN KEY (account_id_receiver)
        REFERENCES account_id (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
);

---------- History tables for server component account ----------

CREATE TABLE IF NOT EXISTS history_account(
    id         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    account_id INTEGER                           NOT NULL,
    unix_time  INTEGER                           NOT NULL,
    json_text  TEXT                              NOT NULL,
    FOREIGN KEY (account_id)
        REFERENCES account_id (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
);

-- TODO: Can be removed as account_setup state does not change after
-- initial setup?
CREATE TABLE IF NOT EXISTS history_account_setup(
    id         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    account_id INTEGER                           NOT NULL,
    unix_time  INTEGER                           NOT NULL,
    json_text  TEXT                              NOT NULL,
    FOREIGN KEY (account_id)
        REFERENCES account_id (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
);

---------- History tables for server component profile ----------

CREATE TABLE IF NOT EXISTS history_profile(
    id     INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    account_id INTEGER                           NOT NULL,
    unix_time  INTEGER                           NOT NULL,
    json_text  TEXT                              NOT NULL,
    FOREIGN KEY (account_id)
        REFERENCES account_id (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
);

---------- History tables for server component media ----------

-- TODO: History for new media tables.

-- Deletion is just ignored as it happens automatically when new
-- request is created.
CREATE TABLE IF NOT EXISTS history_media_moderation_request(
    id                    INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    account_id            INTEGER                           NOT NULL,
    unix_time             INTEGER                           NOT NULL,
    moderation_request_id INTEGER                           NOT NULL,
    state_number          INTEGER                           NOT NULL,
    json_text             TEXT                              NOT NULL,
    FOREIGN KEY (account_id)
        REFERENCES account_id (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
);
