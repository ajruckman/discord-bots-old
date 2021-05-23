create table server
(
    id       varchar(20) not null,
    name     text        not null,
    url_icon text        not null,

    constraint server_pk primary key (id)
);

create table message
(
    id            varchar(20) not null,

    id_server     varchar(20) not null,
    id_channel    varchar(20) not null,
    id_author     varchar(20) not null,
    time          timestamp   not null default now(),
    time_edited   timestamp,
    content       text        not null,
    id_attachment text[],

    constraint message_pk primary key (id),
    constraint message_id_server_fk foreign key (id_server) references server (id)
);

create table attachment
(
    id              varchar(20)  not null,
    file_name       varchar(512) not null,
    file_size_bytes int          not null,
    url             varchar(512) not null,

    constraint attachment_pk primary key (id)
);
