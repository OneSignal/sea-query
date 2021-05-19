use crate::impl_schema_statement_builder;

impl_schema_statement_builder!(table_create_statement_builder, TableCreateStatement);
impl_schema_statement_builder!(table_drop_statement_builder, TableDropStatement);
impl_schema_statement_builder!(table_alter_statement_builder, TableAlterStatement);
impl_schema_statement_builder!(table_rename_statement_builder, TableRenameStatement);
impl_schema_statement_builder!(table_truncate_statement_builder, TableTruncateStatement);
