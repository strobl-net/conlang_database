CREATE TYPE purpose_sub as ENUM (
    'personal', 'jokelang', 'story-based', 'conworld', 'geofictional', 'future', 'alternate_history', 'lostlang',
    'xenolang', 'pseudo_auxlang', 'global_auxlang', 'zonal_auxlang', 'other_auxlang', 'ideal', 'philosophical',
    'logical', 'experimental', 'conpidgin', 'spiritual', 'secret', 'other');

CREATE TYPE physical_mode as ENUM ('speech_writing', 'speech_only', 'writing_only', 'sign', 'other', 'unknown');
CREATE TYPE development_level as ENUM (
    'minimal', 'some', 'medium', 'well', 'learners', 'active_community', 'fluent_users', 'native_users', 'other');

CREATE TYPE vocabulary_source as ENUM ('priori', 'posteriori', 'mixture', 'other', 'unknown');

CREATE TABLE persons
(
    id   SERIAL       NOT NULL,
    name VARCHAR(256) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE scripts
(
    id   SERIAL       NOT NULL,
    name VARCHAR(256) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE groups
(
    id   SERIAL       NOT NULL,
    name VARCHAR(256) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE conlangs
(
    id                SERIAL           NOT NULL,
    name              TEXT             NOT NULL,
    native_name       TEXT,
    registry_code     VARCHAR(8),
    creators          INT[],
    links             TEXT[],
    start_year        TIMESTAMP,
    physical_mode     physical_mode     NOT NULL,
    scripts           INT[],
    groups            INT[],
    purpose           purpose_sub       NOT NULL,
    vocabulary_source vocabulary_source NOT NULL,
    development       development_level NOT NULL,
    notes             TEXT,
    PRIMARY KEY (id)
);