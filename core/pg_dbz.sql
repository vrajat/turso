SELECT
    change_id AS lsn,
    pg_dbz_event(
        change_id,
        change_time,
        change_type,
        table_name,
        CASE
            WHEN before IS NULL THEN NULL
            ELSE bin_record_json_object(table_columns_json_array(table_name), before)
        END,
        CASE
            WHEN after IS NULL THEN NULL
            ELSE bin_record_json_object(table_columns_json_array(table_name), after)
        END,
        change_txn_id,
        CAST(unixepoch('subsec') * 1000 AS INTEGER)
    ) AS event
FROM turso_cdc
WHERE change_id > :after_lsn
  AND change_type IN (-1, 0, 1)
  AND table_name NOT IN ('sqlite_schema', 'sqlite_master')
ORDER BY change_id
