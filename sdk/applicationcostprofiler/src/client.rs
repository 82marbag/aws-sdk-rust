// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(std::fmt::Debug)]
pub(crate) struct Handle<C = aws_hyper::DynConnector> {
    client: aws_hyper::Client<C>,
    conf: crate::Config,
}

#[derive(Clone, std::fmt::Debug)]
pub struct Client<C = aws_hyper::DynConnector> {
    handle: std::sync::Arc<Handle<C>>,
}
impl<C> Client<C> {
    pub fn from_conf_conn(conf: crate::Config, conn: C) -> Self {
        let client = aws_hyper::Client::new(conn);
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl Client {
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_env() -> Self {
        Self::from_conf(crate::Config::builder().build())
    }

    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let client = aws_hyper::Client::https();
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
impl<C> Client<C>
where
    C: aws_hyper::SmithyConnector,
{
    pub fn delete_report_definition(&self) -> fluent_builders::DeleteReportDefinition<C> {
        fluent_builders::DeleteReportDefinition::new(self.handle.clone())
    }
    pub fn get_report_definition(&self) -> fluent_builders::GetReportDefinition<C> {
        fluent_builders::GetReportDefinition::new(self.handle.clone())
    }
    pub fn import_application_usage(&self) -> fluent_builders::ImportApplicationUsage<C> {
        fluent_builders::ImportApplicationUsage::new(self.handle.clone())
    }
    pub fn list_report_definitions(&self) -> fluent_builders::ListReportDefinitions<C> {
        fluent_builders::ListReportDefinitions::new(self.handle.clone())
    }
    pub fn put_report_definition(&self) -> fluent_builders::PutReportDefinition<C> {
        fluent_builders::PutReportDefinition::new(self.handle.clone())
    }
    pub fn update_report_definition(&self) -> fluent_builders::UpdateReportDefinition<C> {
        fluent_builders::UpdateReportDefinition::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct DeleteReportDefinition<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::delete_report_definition_input::Builder,
    }
    impl<C> DeleteReportDefinition<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DeleteReportDefinitionOutput,
            smithy_http::result::SdkError<crate::error::DeleteReportDefinitionError>,
        >
        where
            C: aws_hyper::SmithyConnector,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>Required. ID of the report to delete.</p>
        pub fn report_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.report_id(input);
            self
        }
        pub fn set_report_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_report_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct GetReportDefinition<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::get_report_definition_input::Builder,
    }
    impl<C> GetReportDefinition<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GetReportDefinitionOutput,
            smithy_http::result::SdkError<crate::error::GetReportDefinitionError>,
        >
        where
            C: aws_hyper::SmithyConnector,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>ID of the report to retrieve.</p>
        pub fn report_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.report_id(input);
            self
        }
        pub fn set_report_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_report_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ImportApplicationUsage<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::import_application_usage_input::Builder,
    }
    impl<C> ImportApplicationUsage<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ImportApplicationUsageOutput,
            smithy_http::result::SdkError<crate::error::ImportApplicationUsageError>,
        >
        where
            C: aws_hyper::SmithyConnector,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>Amazon S3 location to import application usage data from.</p>
        pub fn source_s3_location(mut self, input: crate::model::SourceS3Location) -> Self {
            self.inner = self.inner.source_s3_location(input);
            self
        }
        pub fn set_source_s3_location(
            mut self,
            input: std::option::Option<crate::model::SourceS3Location>,
        ) -> Self {
            self.inner = self.inner.set_source_s3_location(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ListReportDefinitions<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::list_report_definitions_input::Builder,
    }
    impl<C> ListReportDefinitions<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ListReportDefinitionsOutput,
            smithy_http::result::SdkError<crate::error::ListReportDefinitionsError>,
        >
        where
            C: aws_hyper::SmithyConnector,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The token value from a previous call to access the next page of results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(input);
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>The maximum number of results to return.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct PutReportDefinition<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::put_report_definition_input::Builder,
    }
    impl<C> PutReportDefinition<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::PutReportDefinitionOutput,
            smithy_http::result::SdkError<crate::error::PutReportDefinitionError>,
        >
        where
            C: aws_hyper::SmithyConnector,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>Required. ID of the report. You can choose any valid string matching the pattern for the
        /// ID.</p>
        pub fn report_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.report_id(input);
            self
        }
        pub fn set_report_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_report_id(input);
            self
        }
        /// <p>Required. Description of the report.</p>
        pub fn report_description(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.report_description(input);
            self
        }
        pub fn set_report_description(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_report_description(input);
            self
        }
        /// <p>Required. The cadence to generate the report.</p>
        pub fn report_frequency(mut self, input: crate::model::ReportFrequency) -> Self {
            self.inner = self.inner.report_frequency(input);
            self
        }
        pub fn set_report_frequency(
            mut self,
            input: std::option::Option<crate::model::ReportFrequency>,
        ) -> Self {
            self.inner = self.inner.set_report_frequency(input);
            self
        }
        /// <p>Required. The format to use for the generated report.</p>
        pub fn format(mut self, input: crate::model::Format) -> Self {
            self.inner = self.inner.format(input);
            self
        }
        pub fn set_format(mut self, input: std::option::Option<crate::model::Format>) -> Self {
            self.inner = self.inner.set_format(input);
            self
        }
        /// <p>Required. Amazon Simple Storage Service (Amazon S3) location where Application Cost Profiler uploads the
        /// report.</p>
        pub fn destination_s3_location(mut self, input: crate::model::S3Location) -> Self {
            self.inner = self.inner.destination_s3_location(input);
            self
        }
        pub fn set_destination_s3_location(
            mut self,
            input: std::option::Option<crate::model::S3Location>,
        ) -> Self {
            self.inner = self.inner.set_destination_s3_location(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct UpdateReportDefinition<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::update_report_definition_input::Builder,
    }
    impl<C> UpdateReportDefinition<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::UpdateReportDefinitionOutput,
            smithy_http::result::SdkError<crate::error::UpdateReportDefinitionError>,
        >
        where
            C: aws_hyper::SmithyConnector,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>Required. ID of the report to update.</p>
        pub fn report_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.report_id(input);
            self
        }
        pub fn set_report_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_report_id(input);
            self
        }
        /// <p>Required. Description of the report.</p>
        pub fn report_description(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.report_description(input);
            self
        }
        pub fn set_report_description(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_report_description(input);
            self
        }
        /// <p>Required. The cadence to generate the report.</p>
        pub fn report_frequency(mut self, input: crate::model::ReportFrequency) -> Self {
            self.inner = self.inner.report_frequency(input);
            self
        }
        pub fn set_report_frequency(
            mut self,
            input: std::option::Option<crate::model::ReportFrequency>,
        ) -> Self {
            self.inner = self.inner.set_report_frequency(input);
            self
        }
        /// <p>Required. The format to use for the generated report.</p>
        pub fn format(mut self, input: crate::model::Format) -> Self {
            self.inner = self.inner.format(input);
            self
        }
        pub fn set_format(mut self, input: std::option::Option<crate::model::Format>) -> Self {
            self.inner = self.inner.set_format(input);
            self
        }
        /// <p>Required. Amazon Simple Storage Service (Amazon S3) location where Application Cost Profiler uploads the
        /// report.</p>
        pub fn destination_s3_location(mut self, input: crate::model::S3Location) -> Self {
            self.inner = self.inner.destination_s3_location(input);
            self
        }
        pub fn set_destination_s3_location(
            mut self,
            input: std::option::Option<crate::model::S3Location>,
        ) -> Self {
            self.inner = self.inner.set_destination_s3_location(input);
            self
        }
    }
}