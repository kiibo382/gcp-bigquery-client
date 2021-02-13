use crate::error::BQError;
use crate::model::get_iam_policy_request::GetIamPolicyRequest;
use crate::model::policy::Policy;
use crate::model::set_iam_policy_request::SetIamPolicyRequest;
use crate::model::table::Table;
use crate::model::table_list::TableList;
use crate::model::test_iam_permissions_request::TestIamPermissionsRequest;
use crate::model::test_iam_permissions_response::TestIamPermissionsResponse;
use crate::{process_response, urlencode};
use reqwest::Client;

pub struct TableApi {
    client: Client,
    access_token: String,
}

impl TableApi {
    pub(crate) fn new(client: Client, access_token: String) -> Self {
        Self { client, access_token }
    }

    /// Creates a new, empty table in the dataset.
    /// # Arguments
    /// * project_id - Project ID of the table to delete
    /// * dataset_id - Dataset ID of the table to delete
    /// * table - The request body contains an instance of Table.
    pub async fn create(&self, project_id: &str, dataset_id: &str, table: Table) -> Result<Table, BQError> {
        let req_url = &format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/datasets/{dataset_id}/tables",
            project_id = urlencode(project_id),
            dataset_id = urlencode(dataset_id)
        );

        let request = self
            .client
            .post(req_url.as_str())
            .bearer_auth(&self.access_token)
            .json(&table)
            .build()?;

        let response = self.client.execute(request).await?;

        process_response(response).await
    }

    /// Deletes the table specified by tableId from the dataset. If the table contains data, all the data will be deleted.
    /// # Arguments
    /// * project_id - Project ID of the table to delete
    /// * dataset_id - Dataset ID of the table to delete
    /// * table_id - Table ID of the table to delete
    pub async fn delete(&self, project_id: &str, dataset_id: &str, table_id: &str) -> Result<(), BQError> {
        let req_url = &format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/datasets/{dataset_id}/tables/{table_id}",
            project_id = urlencode(project_id),
            dataset_id = urlencode(dataset_id),
            table_id = urlencode(table_id)
        );

        let request = self
            .client
            .delete(req_url.as_str())
            .bearer_auth(&self.access_token)
            .build()?;

        let response = self.client.execute(request).await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(BQError::ResponseError {
                error: response.json().await?,
            })
        }
    }

    /// Gets the specified table resource by table ID. This method does not return the data in the table, it only
    /// returns the table resource, which describes the structure of this table.
    /// # Arguments
    /// * project_id - Project ID of the table to delete
    /// * dataset_id - Dataset ID of the table to delete
    /// * table_id - Table ID of the table to delete
    /// * selected_fields - tabledata.list of table schema fields to return (comma-separated). If unspecified, all
    /// fields are returned. A fieldMask cannot be used here because the fields will automatically be converted from
    /// camelCase to snake_case and the conversion will fail if there are underscores. Since these are fields in
    /// BigQuery table schemas, underscores are allowed.
    pub async fn get(
        &self,
        project_id: &str,
        dataset_id: &str,
        table_id: &str,
        selected_fields: Option<Vec<&str>>,
    ) -> Result<Table, BQError> {
        let req_url = &format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/datasets/{dataset_id}/tables/{table_id}",
            project_id = urlencode(project_id),
            dataset_id = urlencode(dataset_id),
            table_id = urlencode(table_id)
        );

        let mut request_builder = self.client.get(req_url.as_str()).bearer_auth(&self.access_token);
        if let Some(selected_fields) = selected_fields {
            let selected_fields = selected_fields.join(",");
            request_builder = request_builder.query(&[("selectedFields", selected_fields)]);
        }

        let request = request_builder.build()?;

        let response = self.client.execute(request).await?;

        process_response(response).await
    }

    /// Lists all tables in the specified dataset. Requires the READER dataset role.
    /// # Arguments
    /// * project_id - Project ID of the table to delete
    /// * dataset_id - Dataset ID of the table to delete
    /// * options - Options
    pub async fn list(&self, project_id: &str, dataset_id: &str, options: ListOptions) -> Result<TableList, BQError> {
        let req_url = &format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/datasets/{dataset_id}/tables",
            project_id = urlencode(project_id),
            dataset_id = urlencode(dataset_id)
        );

        let mut request = self.client.get(req_url).bearer_auth(&self.access_token);

        // process options
        if let Some(max_results) = options.max_results {
            request = request.query(&[("maxResults", max_results.to_string())]);
        }
        if let Some(page_token) = options.page_token {
            request = request.query(&[("pageToken", page_token)]);
        }

        let request = request.build()?;
        let response = self.client.execute(request).await?;

        process_response(response).await
    }

    /// Updates information in an existing table. The update method replaces the entire table resource, whereas the
    /// patch method only replaces fields that are provided in the submitted table resource. This method supports
    /// RFC5789 patch semantics.
    /// # Arguments
    /// * project_id - Project ID of the table to delete
    /// * dataset_id - Dataset ID of the table to delete
    /// * table_id - Table ID of the table to delete
    /// * table - Table to patch
    pub async fn patch(
        &self,
        project_id: &str,
        dataset_id: &str,
        table_id: &str,
        table: Table,
    ) -> Result<Table, BQError> {
        let req_url = &format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/datasets/{dataset_id}/tables/{table_id}",
            project_id = urlencode(project_id),
            dataset_id = urlencode(dataset_id),
            table_id = urlencode(table_id)
        );

        let request = self
            .client
            .patch(req_url)
            .bearer_auth(&self.access_token)
            .json(&table)
            .build()?;
        let response = self.client.execute(request).await?;

        process_response(response).await
    }

    /// Updates information in an existing table. The update method replaces the entire Table resource, whereas the
    /// patch method only replaces fields that are provided in the submitted Table resource.
    /// # Arguments
    /// * project_id - Project ID of the table to delete
    /// * dataset_id - Dataset ID of the table to delete
    /// * table_id - Table ID of the table to delete
    /// * table - Table to update
    pub async fn update(
        &self,
        project_id: &str,
        dataset_id: &str,
        table_id: &str,
        table: Table,
    ) -> Result<Table, BQError> {
        let req_url = &format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{project_id}/datasets/{dataset_id}/tables/{table_id}",
            project_id = urlencode(project_id),
            dataset_id = urlencode(dataset_id),
            table_id = urlencode(table_id)
        );

        let request = self
            .client
            .put(req_url)
            .bearer_auth(&self.access_token)
            .json(&table)
            .build()?;
        let response = self.client.execute(request).await?;

        process_response(response).await
    }

    /// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have
    /// a policy set.
    /// # Argument
    /// * `resource` - The resource for which the policy is being requested. See the operation documentation for the
    /// appropriate value for this field.
    pub async fn get_iam_policy(
        &self,
        resource: &str,
        get_iam_policy_request: GetIamPolicyRequest,
    ) -> Result<Policy, BQError> {
        let req_url = &format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{resource}/:getIamPolicy",
            resource = urlencode(resource)
        );

        let request = self
            .client
            .post(req_url.as_str())
            .bearer_auth(&self.access_token)
            .json(&get_iam_policy_request)
            .build()?;

        let response = self.client.execute(request).await?;

        process_response(response).await
    }

    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`,
    /// `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// # Argument
    /// * `resource` - The resource for which the policy is being specified. See the operation documentation for the appropriate value for this field.
    pub async fn set_iam_policy(
        &self,
        resource: &str,
        set_iam_policy_request: SetIamPolicyRequest,
    ) -> Result<Policy, BQError> {
        let req_url = &format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{resource}/:setIamPolicy",
            resource = urlencode(resource)
        );

        let request = self
            .client
            .post(req_url.as_str())
            .bearer_auth(&self.access_token)
            .json(&set_iam_policy_request)
            .build()?;

        let response = self.client.execute(request).await?;

        process_response(response).await
    }

    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will
    /// return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for
    /// building permission-aware UIs and command-line tools, not for authorization checking. This operation may
    /// \"fail open\" without warning.
    /// # Argument
    /// * `resource` - The resource for which the policy detail is being requested. See the operation documentation for
    /// the appropriate value for this field.
    pub async fn test_iam_permissions(
        &self,
        resource: &str,
        test_iam_permissions_request: TestIamPermissionsRequest,
    ) -> Result<TestIamPermissionsResponse, BQError> {
        let req_url = &format!(
            "https://bigquery.googleapis.com/bigquery/v2/projects/{resource}/:testIamPermissions",
            resource = urlencode(resource)
        );

        let request = self
            .client
            .post(req_url.as_str())
            .bearer_auth(&self.access_token)
            .json(&test_iam_permissions_request)
            .build()?;

        let response = self.client.execute(request).await?;

        process_response(response).await
    }
}

pub struct ListOptions {
    max_results: Option<u64>,
    page_token: Option<String>,
}

impl ListOptions {
    /// The maximum number of results to return in a single response page. Leverage the page tokens to iterate
    /// through the entire collection.
    pub fn max_results(mut self, value: u64) -> Self {
        self.max_results = Some(value);
        self
    }

    /// Page token, returned by a previous call, to request the next page of results
    pub fn page_token(mut self, value: String) -> Self {
        self.page_token = Some(value);
        self
    }
}

impl Default for ListOptions {
    fn default() -> Self {
        Self {
            max_results: None,
            page_token: None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::error::BQError;
    use crate::model::dataset::Dataset;
    use crate::model::field_type::FieldType;
    use crate::model::table::Table;
    use crate::model::table_field_schema::TableFieldSchema;
    use crate::model::table_schema::TableSchema;
    use crate::table::ListOptions;
    use crate::tests::{DATASET_ID, PROJECT_ID, SA_KEY, TABLE_ID};
    use crate::Client;
    use std::rc::Rc;

    #[tokio::test]
    async fn test() -> Result<(), BQError> {
        let client = Client::new(SA_KEY).await;

        // Create dataset
        let created_dataset = client.dataset().create(PROJECT_ID, Dataset::new(DATASET_ID)).await?;
        assert_eq!(created_dataset.id, Some(format!("{}:{}", PROJECT_ID, DATASET_ID)));

        // Create table
        let table = Table::new(
            PROJECT_ID,
            DATASET_ID,
            TABLE_ID,
            TableSchema::new(vec![
                TableFieldSchema::new("col1", FieldType::String),
                TableFieldSchema::new("col2", FieldType::Int64),
                TableFieldSchema::new("col3", FieldType::Boolean),
                TableFieldSchema::new("col4", FieldType::Datetime),
            ]),
        );
        let created_table = client.table().create(PROJECT_ID, DATASET_ID, table).await?;
        assert_eq!(created_table.table_reference.table_id, TABLE_ID.to_string());

        let table = client.table().get(PROJECT_ID, DATASET_ID, TABLE_ID, None).await?;
        assert_eq!(table.table_reference.table_id, TABLE_ID.to_string());

        let table = client.table().update(PROJECT_ID, DATASET_ID, TABLE_ID, table).await?;
        assert_eq!(table.table_reference.table_id, TABLE_ID.to_string());

        let table = client.table().patch(PROJECT_ID, DATASET_ID, TABLE_ID, table).await?;
        assert_eq!(table.table_reference.table_id, TABLE_ID.to_string());

        // List tables
        let tables = client
            .table()
            .list(PROJECT_ID, DATASET_ID, ListOptions::default())
            .await?;
        let mut created_table_found = false;
        for table_list_tables in tables.tables.unwrap().iter() {
            if &table_list_tables.table_reference.dataset_id == DATASET_ID {
                created_table_found = true;
            }
        }
        assert!(created_table_found);

        client.table().delete(PROJECT_ID, DATASET_ID, TABLE_ID).await?;

        // Delete dataset
        client.dataset().delete(PROJECT_ID, DATASET_ID, true).await?;

        Ok(())
    }
}