drop table if exists t_tasks_repeat_history;
drop table if exists t_tasks_repeat;
drop table if exists t_tasks_members;
drop table if exists t_tasks;
drop table if exists t_users;

create table t_users (
    id serial not null primary key,
    username varchar(255) not null,
    create_time timestamp not null default now(),
    update_time timestamp not null default now()
);

alter table t_users add constraint uix_t_users_username unique (username);

create table t_tasks (
    id serial not null primary key,
    name varchar(255) not null,
    description varchar(255) null,
    root_id integer not null,
    parent_id integer not null,
    type varchar(20) not null,
    deadline timestamp null,
    duration integer null,
    is_done boolean not null default false,
    create_time timestamp not null default now(),
    update_time timestamp not null default now()
);

create index idx_t_tasks_root_id_parent_id on t_tasks(root_id, parent_id);

create table t_tasks_members (
    id serial not null primary key,
    t_task_id integer not null,
    t_user_id integer not null,
    create_time timestamp not null default now(),
    update_time timestamp not null default now()
);

alter table t_tasks_members add constraint fk_t_task_id_t_task_id foreign key (t_task_id) references t_tasks(id);
create unique index uix_t_tasks_parent_id_name on t_tasks(parent_id, name);
create unique index uix_t_tasks_members_task_id_user_id on t_tasks_members(t_task_id, t_user_id);


-- foreign key 이슈로 이시점에서 생성 (1번 tasks 없이 fk 생성 불가.)
insert into public.t_tasks (id, name, parent_id, type, deadline, duration, is_done, create_time, update_time, root_id, description) values (1, 'root', 1, 'group', null, null, false, '2023-04-17 06:22:33.352783', '2023-04-17 06:22:33.352783', 1, null);
-- foreign key 이슈로 이시점에서 생성
alter table t_tasks add constraint fk_t_task_t_parent_t_task_id foreign key (parent_id) references t_tasks(id);

create table t_tasks_repeat (
    id serial not null primary key,
    t_task_id integer not null,
    type varchar(20) not null, -- daily, week, month, year
    all_day boolean not null default true,
    start_time timestamp,
    end_time timestamp,
    is_daily boolean not null default false,
    day_of_week smallint[] null, -- 0 ~ 6
    day_of_month smallint[] null, -- 1 ~ 31
    day_of_year varchar(8)[] null, -- ex ['20230101', '20230102', '20230103']
    create_time timestamp not null default now(),
    update_time timestamp not null default now()
);

alter table t_tasks_repeat add constraint idx_t_task_id_type_is_daily unique (t_task_id, type, is_daily);
alter table t_tasks_repeat add constraint idx_t_task_id_type_day_of_week unique (t_task_id, type, day_of_week);
alter table t_tasks_repeat add constraint idx_t_task_id_type_day_of_month unique (t_task_id, type, day_of_month);
alter table t_tasks_repeat add constraint idx_t_task_id_type_day_of_year unique (t_task_id, type, day_of_year);


create table t_tasks_repeat_history (
    id serial not null primary key,
    t_task_id integer not null,
    t_user_id integer not null,
    is_done boolean not null default false,
    memo text not null,
    create_time timestamp not null default now(),
    update_time timestamp not null default now()
);

alter table t_tasks_repeat_history add constraint idx_t_task_id_t_user_id unique (t_task_id, t_user_id);




-- DML
-- main task
insert into t_tasks (id, name, parent_id, type, deadline, duration, is_done, create_time, update_time, root_id, description) values (2, '부종민 개인', 1, 'group', null, null, false, '2023-04-17 06:22:52.345935', '2023-04-17 06:22:52.345935', 1, '개인업무');
insert into t_tasks (id, name, parent_id, type, deadline, duration, is_done, create_time, update_time, root_id, description) values (3, '정규직-회사1', 1, 'group', null, null, false, '2023-04-17 06:22:53.692836', '2023-04-17 06:22:53.692836', 1, '돈을 많이 버는게 회사의 존재 이유');
insert into t_tasks (id, name, parent_id, type, deadline, duration, is_done, create_time, update_time, root_id, description) values (4, '아르바이트1', 1, 'group', null, null, false, '2023-04-17 06:27:40.619289', '2023-04-17 06:27:40.619289', 1, '');
insert into t_tasks (id, name, parent_id, type, deadline, duration, is_done, create_time, update_time, root_id, description) values (5, '프리랜서-사이트외주', 1, 'group', null, null, false, '2023-04-17 06:27:40.640122', '2023-04-17 06:27:40.640122', 1, '샘플');
insert into t_tasks (id, name, parent_id, type, deadline, duration, is_done, create_time, update_time, root_id, description) values (6, 'task 앱 만들기', 2, 'range', null, null, false, '2023-04-17 07:11:27.691133', '2023-04-17 07:11:27.691133', 2, '만들어야한다.');
insert into t_tasks (id, name, parent_id, type, deadline, duration, is_done, create_time, update_time, root_id, description) values (7, '매주 토요일 화단 물주기', 2, 'repeat', null, null, false, '2023-04-17 07:11:27.725000', '2023-04-17 07:11:27.725000', 2, '살려야한다.');
insert into t_tasks (id, name, parent_id, type, deadline, duration, is_done, create_time, update_time, root_id, description) values (8, '구직', 2, 'end', null, null, false, '2023-04-18 08:28:14.917001', '2023-04-18 08:28:14.917001', 2, '노답');

-- sub tasks
insert into t_tasks_members (id, t_task_id, t_user_id, create_time, update_time) values (1, 2, 1, '2023-04-17 06:25:13.335002', '2023-04-17 06:25:13.335002');
insert into t_tasks_members (id, t_task_id, t_user_id, create_time, update_time) values (2, 3, 1, '2023-04-17 06:25:13.349688', '2023-04-17 06:25:13.349688');
insert into t_tasks_members (id, t_task_id, t_user_id, create_time, update_time) values (3, 2, 2, '2023-04-17 06:25:13.362560', '2023-04-17 06:25:13.362560');
insert into t_tasks_members (id, t_task_id, t_user_id, create_time, update_time) values (4, 4, 1, '2023-04-17 06:27:44.642548', '2023-04-17 06:27:44.642548');
insert into t_tasks_members (id, t_task_id, t_user_id, create_time, update_time) values (5, 5, 1, '2023-04-17 06:27:44.656554', '2023-04-17 06:27:44.656554');
insert into t_tasks_members (id, t_task_id, t_user_id, create_time, update_time) values (6, 6, 1, '2023-04-17 07:16:37.851336', '2023-04-17 07:16:37.851336');
insert into t_tasks_members (id, t_task_id, t_user_id, create_time, update_time) values (7, 7, 1, '2023-04-17 07:16:37.875360', '2023-04-17 07:16:37.875360');
insert into t_tasks_members (id, t_task_id, t_user_id, create_time, update_time) values (8, 8, 1, '2023-04-17 07:16:37.875366', '2023-04-17 07:16:37.875366');
