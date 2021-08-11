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
    pub fn analyze_document(&self) -> fluent_builders::AnalyzeDocument<C> {
        fluent_builders::AnalyzeDocument::new(self.handle.clone())
    }
    pub fn analyze_expense(&self) -> fluent_builders::AnalyzeExpense<C> {
        fluent_builders::AnalyzeExpense::new(self.handle.clone())
    }
    pub fn detect_document_text(&self) -> fluent_builders::DetectDocumentText<C> {
        fluent_builders::DetectDocumentText::new(self.handle.clone())
    }
    pub fn get_document_analysis(&self) -> fluent_builders::GetDocumentAnalysis<C> {
        fluent_builders::GetDocumentAnalysis::new(self.handle.clone())
    }
    pub fn get_document_text_detection(&self) -> fluent_builders::GetDocumentTextDetection<C> {
        fluent_builders::GetDocumentTextDetection::new(self.handle.clone())
    }
    pub fn start_document_analysis(&self) -> fluent_builders::StartDocumentAnalysis<C> {
        fluent_builders::StartDocumentAnalysis::new(self.handle.clone())
    }
    pub fn start_document_text_detection(&self) -> fluent_builders::StartDocumentTextDetection<C> {
        fluent_builders::StartDocumentTextDetection::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct AnalyzeDocument<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::analyze_document_input::Builder,
    }
    impl<C> AnalyzeDocument<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::AnalyzeDocumentOutput,
            smithy_http::result::SdkError<crate::error::AnalyzeDocumentError>,
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
        /// <p>The input document as base64-encoded bytes or an Amazon S3 object. If you use the AWS CLI
        /// to call Amazon Textract operations, you can't pass image bytes. The document must be an image
        /// in JPEG or PNG format.</p>
        /// <p>If you're using an AWS SDK to call Amazon Textract, you might not need to base64-encode
        /// image bytes that are passed using the <code>Bytes</code> field. </p>
        pub fn document(mut self, input: crate::model::Document) -> Self {
            self.inner = self.inner.document(input);
            self
        }
        pub fn set_document(mut self, input: std::option::Option<crate::model::Document>) -> Self {
            self.inner = self.inner.set_document(input);
            self
        }
        /// <p>A list of the types of analysis to perform. Add TABLES to the list to return information
        /// about the tables that are detected in the input document. Add FORMS to return detected form data.
        /// To perform both types of analysis, add TABLES and FORMS to
        /// <code>FeatureTypes</code>. All lines and words detected in the document are included in
        /// the response (including text that isn't related to the value of <code>FeatureTypes</code>). </p>
        pub fn feature_types(mut self, inp: impl Into<crate::model::FeatureType>) -> Self {
            self.inner = self.inner.feature_types(inp);
            self
        }
        pub fn set_feature_types(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::FeatureType>>,
        ) -> Self {
            self.inner = self.inner.set_feature_types(input);
            self
        }
        /// <p>Sets the configuration for the human in the loop workflow for analyzing documents.</p>
        pub fn human_loop_config(mut self, input: crate::model::HumanLoopConfig) -> Self {
            self.inner = self.inner.human_loop_config(input);
            self
        }
        pub fn set_human_loop_config(
            mut self,
            input: std::option::Option<crate::model::HumanLoopConfig>,
        ) -> Self {
            self.inner = self.inner.set_human_loop_config(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct AnalyzeExpense<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::analyze_expense_input::Builder,
    }
    impl<C> AnalyzeExpense<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::AnalyzeExpenseOutput,
            smithy_http::result::SdkError<crate::error::AnalyzeExpenseError>,
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
        /// <p>The input document, either as bytes or as an S3 object.</p>
        /// <p>You pass image bytes to an Amazon Textract API operation by using the <code>Bytes</code>
        /// property. For example, you would use the <code>Bytes</code> property to pass a document
        /// loaded from a local file system. Image bytes passed by using the <code>Bytes</code>
        /// property must be base64 encoded. Your code might not need to encode document file bytes if
        /// you're using an AWS SDK to call Amazon Textract API operations. </p>
        /// <p>You pass images stored in an S3 bucket to an Amazon Textract API operation by using the
        /// <code>S3Object</code> property. Documents stored in an S3 bucket don't need to be base64
        /// encoded.</p>
        /// <p>The AWS Region for the S3 bucket that contains the S3 object must match the AWS
        /// Region that you use for Amazon Textract operations.</p>
        /// <p>If you use the AWS CLI to call Amazon Textract operations, passing image bytes using
        /// the Bytes property isn't supported. You must first upload the document to an Amazon S3
        /// bucket, and then call the operation using the S3Object property.</p>
        /// <p>For Amazon Textract to process an S3 object, the user must have permission
        /// to access the S3 object. </p>
        pub fn document(mut self, input: crate::model::Document) -> Self {
            self.inner = self.inner.document(input);
            self
        }
        pub fn set_document(mut self, input: std::option::Option<crate::model::Document>) -> Self {
            self.inner = self.inner.set_document(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct DetectDocumentText<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::detect_document_text_input::Builder,
    }
    impl<C> DetectDocumentText<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DetectDocumentTextOutput,
            smithy_http::result::SdkError<crate::error::DetectDocumentTextError>,
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
        /// <p>The input document as base64-encoded bytes or an Amazon S3 object. If you use the AWS CLI
        /// to call Amazon Textract operations, you can't pass image bytes. The document must be an image
        /// in JPEG or PNG format.</p>
        /// <p>If you're using an AWS SDK to call Amazon Textract, you might not need to base64-encode
        /// image bytes that are passed using the <code>Bytes</code> field. </p>
        pub fn document(mut self, input: crate::model::Document) -> Self {
            self.inner = self.inner.document(input);
            self
        }
        pub fn set_document(mut self, input: std::option::Option<crate::model::Document>) -> Self {
            self.inner = self.inner.set_document(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct GetDocumentAnalysis<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::get_document_analysis_input::Builder,
    }
    impl<C> GetDocumentAnalysis<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GetDocumentAnalysisOutput,
            smithy_http::result::SdkError<crate::error::GetDocumentAnalysisError>,
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
        /// <p>A unique identifier for the text-detection job. The <code>JobId</code> is returned from
        /// <code>StartDocumentAnalysis</code>. A <code>JobId</code> value is only valid for 7 days.</p>
        pub fn job_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.job_id(input);
            self
        }
        pub fn set_job_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_job_id(input);
            self
        }
        /// <p>The maximum number of results to return per paginated call. The largest value that you
        /// can specify is 1,000. If you specify a value greater than 1,000, a maximum of 1,000 results
        /// is returned. The default value is 1,000.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>If the previous response was incomplete (because there are more blocks to retrieve), Amazon Textract returns a pagination
        /// token in the response. You can use this pagination token to retrieve the next set of blocks.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(input);
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct GetDocumentTextDetection<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::get_document_text_detection_input::Builder,
    }
    impl<C> GetDocumentTextDetection<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GetDocumentTextDetectionOutput,
            smithy_http::result::SdkError<crate::error::GetDocumentTextDetectionError>,
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
        /// <p>A unique identifier for the text detection job. The <code>JobId</code> is returned from
        /// <code>StartDocumentTextDetection</code>. A <code>JobId</code> value is only valid for 7 days.</p>
        pub fn job_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.job_id(input);
            self
        }
        pub fn set_job_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_job_id(input);
            self
        }
        /// <p>The maximum number of results to return per paginated call. The largest value you can
        /// specify is 1,000. If you specify a value greater than 1,000, a maximum of 1,000 results is
        /// returned. The default value is 1,000.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>If the previous response was incomplete (because there are more blocks to retrieve), Amazon Textract returns a pagination
        /// token in the response. You can use this pagination token to retrieve the next set of blocks.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(input);
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct StartDocumentAnalysis<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::start_document_analysis_input::Builder,
    }
    impl<C> StartDocumentAnalysis<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::StartDocumentAnalysisOutput,
            smithy_http::result::SdkError<crate::error::StartDocumentAnalysisError>,
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
        /// <p>The location of the document to be processed.</p>
        pub fn document_location(mut self, input: crate::model::DocumentLocation) -> Self {
            self.inner = self.inner.document_location(input);
            self
        }
        pub fn set_document_location(
            mut self,
            input: std::option::Option<crate::model::DocumentLocation>,
        ) -> Self {
            self.inner = self.inner.set_document_location(input);
            self
        }
        /// <p>A list of the types of analysis to perform. Add TABLES to the list to return information
        /// about the tables that are detected in the input document. Add FORMS to return detected
        /// form data. To perform both types of analysis, add TABLES
        /// and FORMS to <code>FeatureTypes</code>. All lines and words detected in the document are
        /// included in the response (including text that isn't related to the value of
        /// <code>FeatureTypes</code>). </p>
        pub fn feature_types(mut self, inp: impl Into<crate::model::FeatureType>) -> Self {
            self.inner = self.inner.feature_types(inp);
            self
        }
        pub fn set_feature_types(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::FeatureType>>,
        ) -> Self {
            self.inner = self.inner.set_feature_types(input);
            self
        }
        /// <p>The idempotent token that you use to identify the start request. If you use the same
        /// token with multiple <code>StartDocumentAnalysis</code> requests, the same
        /// <code>JobId</code> is returned. Use <code>ClientRequestToken</code> to prevent the same
        /// job from being accidentally started more than once. For more information, see
        /// <a href="https://docs.aws.amazon.com/textract/latest/dg/api-async.html">Calling Amazon Textract Asynchronous Operations</a>.</p>
        pub fn client_request_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.client_request_token(input);
            self
        }
        pub fn set_client_request_token(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_client_request_token(input);
            self
        }
        /// <p>An identifier that you specify that's included in the completion notification published
        /// to the Amazon SNS topic. For example, you can use <code>JobTag</code> to identify the type of
        /// document that the completion notification corresponds to (such as a tax form or a
        /// receipt).</p>
        pub fn job_tag(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.job_tag(input);
            self
        }
        pub fn set_job_tag(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_job_tag(input);
            self
        }
        /// <p>The Amazon SNS topic ARN that you want Amazon Textract to publish the completion status of the
        /// operation to. </p>
        pub fn notification_channel(mut self, input: crate::model::NotificationChannel) -> Self {
            self.inner = self.inner.notification_channel(input);
            self
        }
        pub fn set_notification_channel(
            mut self,
            input: std::option::Option<crate::model::NotificationChannel>,
        ) -> Self {
            self.inner = self.inner.set_notification_channel(input);
            self
        }
        /// <p>Sets if the output will go to a customer defined bucket. By default, Amazon Textract will save
        /// the results internally to be accessed by the GetDocumentAnalysis operation.</p>
        pub fn output_config(mut self, input: crate::model::OutputConfig) -> Self {
            self.inner = self.inner.output_config(input);
            self
        }
        pub fn set_output_config(
            mut self,
            input: std::option::Option<crate::model::OutputConfig>,
        ) -> Self {
            self.inner = self.inner.set_output_config(input);
            self
        }
        /// <p>The KMS key used to encrypt the inference results. This can be
        /// in either Key ID or Key Alias format. When a KMS key is provided, the
        /// KMS key will be used for server-side encryption of the objects in the
        /// customer bucket. When this parameter is not enabled, the result will
        /// be encrypted server side,using SSE-S3.</p>
        pub fn kms_key_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.kms_key_id(input);
            self
        }
        pub fn set_kms_key_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_kms_key_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct StartDocumentTextDetection<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::start_document_text_detection_input::Builder,
    }
    impl<C> StartDocumentTextDetection<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::StartDocumentTextDetectionOutput,
            smithy_http::result::SdkError<crate::error::StartDocumentTextDetectionError>,
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
        /// <p>The location of the document to be processed.</p>
        pub fn document_location(mut self, input: crate::model::DocumentLocation) -> Self {
            self.inner = self.inner.document_location(input);
            self
        }
        pub fn set_document_location(
            mut self,
            input: std::option::Option<crate::model::DocumentLocation>,
        ) -> Self {
            self.inner = self.inner.set_document_location(input);
            self
        }
        /// <p>The idempotent token that's used to identify the start request. If you use the same
        /// token with multiple <code>StartDocumentTextDetection</code> requests, the same
        /// <code>JobId</code> is returned. Use <code>ClientRequestToken</code> to prevent the same
        /// job from being accidentally started more than once. For more information, see
        /// <a href="https://docs.aws.amazon.com/textract/latest/dg/api-async.html">Calling Amazon Textract Asynchronous Operations</a>.</p>
        pub fn client_request_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.client_request_token(input);
            self
        }
        pub fn set_client_request_token(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_client_request_token(input);
            self
        }
        /// <p>An identifier that you specify that's included in the completion notification published
        /// to the Amazon SNS topic. For example, you can use <code>JobTag</code> to identify the type of
        /// document that the completion notification corresponds to (such as a tax form or a
        /// receipt).</p>
        pub fn job_tag(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.job_tag(input);
            self
        }
        pub fn set_job_tag(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_job_tag(input);
            self
        }
        /// <p>The Amazon SNS topic ARN that you want Amazon Textract to publish the completion status of the
        /// operation to. </p>
        pub fn notification_channel(mut self, input: crate::model::NotificationChannel) -> Self {
            self.inner = self.inner.notification_channel(input);
            self
        }
        pub fn set_notification_channel(
            mut self,
            input: std::option::Option<crate::model::NotificationChannel>,
        ) -> Self {
            self.inner = self.inner.set_notification_channel(input);
            self
        }
        /// <p>Sets if the output will go to a customer defined bucket. By default Amazon Textract will
        /// save the results internally to be accessed with the GetDocumentTextDetection operation.</p>
        pub fn output_config(mut self, input: crate::model::OutputConfig) -> Self {
            self.inner = self.inner.output_config(input);
            self
        }
        pub fn set_output_config(
            mut self,
            input: std::option::Option<crate::model::OutputConfig>,
        ) -> Self {
            self.inner = self.inner.set_output_config(input);
            self
        }
        /// <p>The KMS key used to encrypt the inference results. This can be
        /// in either Key ID or Key Alias format. When a KMS key is provided, the
        /// KMS key will be used for server-side encryption of the objects in the
        /// customer bucket. When this parameter is not enabled, the result will
        /// be encrypted server side,using SSE-S3.</p>
        pub fn kms_key_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.kms_key_id(input);
            self
        }
        pub fn set_kms_key_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_kms_key_id(input);
            self
        }
    }
}