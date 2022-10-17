// @generated automatically by Diesel CLI.

diesel::table! {
    contest_accessible_teams (pk) {
        pk -> Uuid,
        team_pk -> Uuid,
        contest_pk -> Uuid,
    }
}

diesel::table! {
    contest_accessible_users (pk) {
        pk -> Uuid,
        user_pk -> Uuid,
        contest_pk -> Uuid,
    }
}

diesel::table! {
    contest_tasks (pk) {
        pk -> Uuid,
        task_pk -> Uuid,
        contest_pk -> Uuid,
    }
}

diesel::table! {
    contests (pk) {
        pk -> Uuid,
        name -> Varchar,
        start_at -> Timestamptz,
        end_at -> Timestamptz,
        is_public -> Nullable<Bool>,
    }
}

diesel::table! {
    submissions (pk) {
        pk -> Uuid,
        user_pk -> Uuid,
        task_pk -> Uuid,
        lang_uuid -> Uuid,
        issued_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    tasks (pk) {
        pk -> Uuid,
        internal_task_uuid -> Uuid,
        name -> Varchar,
        code -> Varchar,
        is_public -> Nullable<Bool>,
    }
}

diesel::table! {
    team_users (pk) {
        pk -> Uuid,
        user_pk -> Uuid,
        team_pk -> Uuid,
    }
}

diesel::table! {
    teams (pk) {
        pk -> Uuid,
        name -> Varchar,
    }
}

diesel::table! {
    users (pk) {
        id -> Varchar,
        pass -> Varchar,
        permission -> Int4,
        timezone -> Varchar,
        email -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        preferred_language -> Uuid,
        pk -> Uuid,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    contest_accessible_teams,
    contest_accessible_users,
    contest_tasks,
    contests,
    submissions,
    tasks,
    team_users,
    teams,
    users,
);
