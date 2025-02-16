create table category (
    id varchar(21) primary key,
    ap_id varchar unique not null,
    local boolean not null,
    name varchar not null,
    sub_categories varchar[] not null default '{}', -- array of ids
    image_url varchar, -- optional image url
    parent_id varchar, -- references category(ap_id) on delete cascade, -- foreign key to self
    created_at timestamptz default current_timestamp not null, -- timestamp for creation
    updated_at timestamptz default current_timestamp not null -- timestamp for last update
);

create index idx_category_id on category (id);
create index idx_category_ap_id on category (ap_id);
create index idx_category_name on category (name);
create index idx_category_parent_id on category (parent_id);
create index idx_category_id_parent_created on category (ap_id, parent_id, created_at);

create or replace function update_updated_at()
returns trigger as $$
begin
    new.updated_at = current_timestamp;
    return new;
end;
$$ language plpgsql;

create trigger set_updated_at
before update on category
for each row
execute function update_updated_at();
