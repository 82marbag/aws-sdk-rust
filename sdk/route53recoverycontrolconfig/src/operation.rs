// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Create a new cluster. A cluster is a set of redundant Regional endpoints against which you can run API calls to update or get the state of one or more routing controls. Each cluster has a name, status, Amazon Resource Name (ARN), and an array of the five cluster endpoints (one for each supported Amazon Web Services Region) that you can use with API calls to the Amazon Route 53 Application Recovery Controller cluster data plane.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateCluster {
    _private: (),
}
impl CreateCluster {
    /// Creates a new builder-style object to manufacture [`CreateClusterInput`](crate::input::CreateClusterInput)
    pub fn builder() -> crate::input::create_cluster_input::Builder {
        crate::input::create_cluster_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateCluster {
    type Output =
        std::result::Result<crate::output::CreateClusterOutput, crate::error::CreateClusterError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_cluster_error(response)
        } else {
            crate::operation_deser::parse_create_cluster_response(response)
        }
    }
}

/// <p>Creates a new control panel. A control panel represents a group of routing controls that can be changed together in a single transaction. You can use a control panel to centrally view the operational status of applications across your organization, and trigger multi-app failovers in a single transaction, for example, to fail over an Availability Zone or AWS Region.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateControlPanel {
    _private: (),
}
impl CreateControlPanel {
    /// Creates a new builder-style object to manufacture [`CreateControlPanelInput`](crate::input::CreateControlPanelInput)
    pub fn builder() -> crate::input::create_control_panel_input::Builder {
        crate::input::create_control_panel_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateControlPanel {
    type Output = std::result::Result<
        crate::output::CreateControlPanelOutput,
        crate::error::CreateControlPanelError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_control_panel_error(response)
        } else {
            crate::operation_deser::parse_create_control_panel_response(response)
        }
    }
}

/// <p>Creates a new routing control.</p> <p>A routing control has one of two states: ON and OFF. You can map the routing control state to the state of an Amazon Route 53 health check, which can be used to control traffic routing.</p> <p>To get or update the routing control state, see the Recovery Cluster (data plane) API actions for Amazon Route 53 Application Recovery Controller.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateRoutingControl {
    _private: (),
}
impl CreateRoutingControl {
    /// Creates a new builder-style object to manufacture [`CreateRoutingControlInput`](crate::input::CreateRoutingControlInput)
    pub fn builder() -> crate::input::create_routing_control_input::Builder {
        crate::input::create_routing_control_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateRoutingControl {
    type Output = std::result::Result<
        crate::output::CreateRoutingControlOutput,
        crate::error::CreateRoutingControlError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_routing_control_error(response)
        } else {
            crate::operation_deser::parse_create_routing_control_response(response)
        }
    }
}

/// <p>Creates a safety rule in a control panel. Safety rules let you add safeguards around enabling and disabling routing controls, to help prevent unexpected outcomes.</p> <p>There are two types of safety rules: assertion rules and gating rules.</p> <p>Assertion rule: An assertion rule enforces that, when a routing control state is changed, the criteria set by the rule configuration is met. Otherwise, the change to the routing control is not accepted.</p> <p>Gating rule: A gating rule verifies that a set of gating controls evaluates as true, based on a rule configuration that you specify. If the gating rule evaluates to true, Amazon Route 53 Application Recovery Controller allows a set of routing control state changes to run and complete against the set of target controls.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateSafetyRule {
    _private: (),
}
impl CreateSafetyRule {
    /// Creates a new builder-style object to manufacture [`CreateSafetyRuleInput`](crate::input::CreateSafetyRuleInput)
    pub fn builder() -> crate::input::create_safety_rule_input::Builder {
        crate::input::create_safety_rule_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateSafetyRule {
    type Output = std::result::Result<
        crate::output::CreateSafetyRuleOutput,
        crate::error::CreateSafetyRuleError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_safety_rule_error(response)
        } else {
            crate::operation_deser::parse_create_safety_rule_response(response)
        }
    }
}

/// <p>Delete a cluster.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteCluster {
    _private: (),
}
impl DeleteCluster {
    /// Creates a new builder-style object to manufacture [`DeleteClusterInput`](crate::input::DeleteClusterInput)
    pub fn builder() -> crate::input::delete_cluster_input::Builder {
        crate::input::delete_cluster_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteCluster {
    type Output =
        std::result::Result<crate::output::DeleteClusterOutput, crate::error::DeleteClusterError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_cluster_error(response)
        } else {
            crate::operation_deser::parse_delete_cluster_response(response)
        }
    }
}

/// <p>Deletes a control panel.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteControlPanel {
    _private: (),
}
impl DeleteControlPanel {
    /// Creates a new builder-style object to manufacture [`DeleteControlPanelInput`](crate::input::DeleteControlPanelInput)
    pub fn builder() -> crate::input::delete_control_panel_input::Builder {
        crate::input::delete_control_panel_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteControlPanel {
    type Output = std::result::Result<
        crate::output::DeleteControlPanelOutput,
        crate::error::DeleteControlPanelError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_control_panel_error(response)
        } else {
            crate::operation_deser::parse_delete_control_panel_response(response)
        }
    }
}

/// <p>Deletes a routing control.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteRoutingControl {
    _private: (),
}
impl DeleteRoutingControl {
    /// Creates a new builder-style object to manufacture [`DeleteRoutingControlInput`](crate::input::DeleteRoutingControlInput)
    pub fn builder() -> crate::input::delete_routing_control_input::Builder {
        crate::input::delete_routing_control_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteRoutingControl {
    type Output = std::result::Result<
        crate::output::DeleteRoutingControlOutput,
        crate::error::DeleteRoutingControlError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_routing_control_error(response)
        } else {
            crate::operation_deser::parse_delete_routing_control_response(response)
        }
    }
}

/// <p>Deletes a safety rule.</p>/&gt;
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteSafetyRule {
    _private: (),
}
impl DeleteSafetyRule {
    /// Creates a new builder-style object to manufacture [`DeleteSafetyRuleInput`](crate::input::DeleteSafetyRuleInput)
    pub fn builder() -> crate::input::delete_safety_rule_input::Builder {
        crate::input::delete_safety_rule_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteSafetyRule {
    type Output = std::result::Result<
        crate::output::DeleteSafetyRuleOutput,
        crate::error::DeleteSafetyRuleError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_safety_rule_error(response)
        } else {
            crate::operation_deser::parse_delete_safety_rule_response(response)
        }
    }
}

/// <p>Display the details about a cluster. The response includes the cluster name, endpoints, status, and Amazon Resource Name (ARN).</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeCluster {
    _private: (),
}
impl DescribeCluster {
    /// Creates a new builder-style object to manufacture [`DescribeClusterInput`](crate::input::DescribeClusterInput)
    pub fn builder() -> crate::input::describe_cluster_input::Builder {
        crate::input::describe_cluster_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeCluster {
    type Output = std::result::Result<
        crate::output::DescribeClusterOutput,
        crate::error::DescribeClusterError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_cluster_error(response)
        } else {
            crate::operation_deser::parse_describe_cluster_response(response)
        }
    }
}

/// <p>Displays details about a control panel.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeControlPanel {
    _private: (),
}
impl DescribeControlPanel {
    /// Creates a new builder-style object to manufacture [`DescribeControlPanelInput`](crate::input::DescribeControlPanelInput)
    pub fn builder() -> crate::input::describe_control_panel_input::Builder {
        crate::input::describe_control_panel_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeControlPanel {
    type Output = std::result::Result<
        crate::output::DescribeControlPanelOutput,
        crate::error::DescribeControlPanelError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_control_panel_error(response)
        } else {
            crate::operation_deser::parse_describe_control_panel_response(response)
        }
    }
}

/// <p>Displays details about a routing control. A routing control has one of two states: ON and OFF. You can map the routing control state to the state of an Amazon Route 53 health check, which can be used to control routing.</p> <p>To get or update the routing control state, see the Recovery Cluster (data plane) API actions for Amazon Route 53 Application Recovery Controller.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeRoutingControl {
    _private: (),
}
impl DescribeRoutingControl {
    /// Creates a new builder-style object to manufacture [`DescribeRoutingControlInput`](crate::input::DescribeRoutingControlInput)
    pub fn builder() -> crate::input::describe_routing_control_input::Builder {
        crate::input::describe_routing_control_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeRoutingControl {
    type Output = std::result::Result<
        crate::output::DescribeRoutingControlOutput,
        crate::error::DescribeRoutingControlError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_routing_control_error(response)
        } else {
            crate::operation_deser::parse_describe_routing_control_response(response)
        }
    }
}

/// <p>Describes the safety rules (that is, the assertion rules and gating rules) for the routing controls in a control panel.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeSafetyRule {
    _private: (),
}
impl DescribeSafetyRule {
    /// Creates a new builder-style object to manufacture [`DescribeSafetyRuleInput`](crate::input::DescribeSafetyRuleInput)
    pub fn builder() -> crate::input::describe_safety_rule_input::Builder {
        crate::input::describe_safety_rule_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeSafetyRule {
    type Output = std::result::Result<
        crate::output::DescribeSafetyRuleOutput,
        crate::error::DescribeSafetyRuleError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_safety_rule_error(response)
        } else {
            crate::operation_deser::parse_describe_safety_rule_response(response)
        }
    }
}

/// <p>Returns an array of all Amazon Route 53 health checks associated with a specific routing control.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListAssociatedRoute53HealthChecks {
    _private: (),
}
impl ListAssociatedRoute53HealthChecks {
    /// Creates a new builder-style object to manufacture [`ListAssociatedRoute53HealthChecksInput`](crate::input::ListAssociatedRoute53HealthChecksInput)
    pub fn builder() -> crate::input::list_associated_route53_health_checks_input::Builder {
        crate::input::list_associated_route53_health_checks_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListAssociatedRoute53HealthChecks {
    type Output = std::result::Result<
        crate::output::ListAssociatedRoute53HealthChecksOutput,
        crate::error::ListAssociatedRoute53HealthChecksError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_associated_route53_health_checks_error(response)
        } else {
            crate::operation_deser::parse_list_associated_route53_health_checks_response(response)
        }
    }
}

/// <p>Returns an array of all the clusters in an account.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListClusters {
    _private: (),
}
impl ListClusters {
    /// Creates a new builder-style object to manufacture [`ListClustersInput`](crate::input::ListClustersInput)
    pub fn builder() -> crate::input::list_clusters_input::Builder {
        crate::input::list_clusters_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListClusters {
    type Output =
        std::result::Result<crate::output::ListClustersOutput, crate::error::ListClustersError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_clusters_error(response)
        } else {
            crate::operation_deser::parse_list_clusters_response(response)
        }
    }
}

/// <p>Returns an array of control panels for a cluster.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListControlPanels {
    _private: (),
}
impl ListControlPanels {
    /// Creates a new builder-style object to manufacture [`ListControlPanelsInput`](crate::input::ListControlPanelsInput)
    pub fn builder() -> crate::input::list_control_panels_input::Builder {
        crate::input::list_control_panels_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListControlPanels {
    type Output = std::result::Result<
        crate::output::ListControlPanelsOutput,
        crate::error::ListControlPanelsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_control_panels_error(response)
        } else {
            crate::operation_deser::parse_list_control_panels_response(response)
        }
    }
}

/// <p>Returns an array of routing controls for a control panel. A routing control is an Amazon Route 53 Application Recovery Controller construct that has one of two states: ON and OFF. You can map the routing control state to the state of an Amazon Route 53 health check, which can be used to control routing.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListRoutingControls {
    _private: (),
}
impl ListRoutingControls {
    /// Creates a new builder-style object to manufacture [`ListRoutingControlsInput`](crate::input::ListRoutingControlsInput)
    pub fn builder() -> crate::input::list_routing_controls_input::Builder {
        crate::input::list_routing_controls_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListRoutingControls {
    type Output = std::result::Result<
        crate::output::ListRoutingControlsOutput,
        crate::error::ListRoutingControlsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_routing_controls_error(response)
        } else {
            crate::operation_deser::parse_list_routing_controls_response(response)
        }
    }
}

/// <p>List the safety rules (the assertion rules and gating rules) that you've defined for the routing controls in a control panel.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListSafetyRules {
    _private: (),
}
impl ListSafetyRules {
    /// Creates a new builder-style object to manufacture [`ListSafetyRulesInput`](crate::input::ListSafetyRulesInput)
    pub fn builder() -> crate::input::list_safety_rules_input::Builder {
        crate::input::list_safety_rules_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListSafetyRules {
    type Output = std::result::Result<
        crate::output::ListSafetyRulesOutput,
        crate::error::ListSafetyRulesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_safety_rules_error(response)
        } else {
            crate::operation_deser::parse_list_safety_rules_response(response)
        }
    }
}

/// <p>Updates a control panel. The only update you can make to a control panel is to change the name of the control panel.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateControlPanel {
    _private: (),
}
impl UpdateControlPanel {
    /// Creates a new builder-style object to manufacture [`UpdateControlPanelInput`](crate::input::UpdateControlPanelInput)
    pub fn builder() -> crate::input::update_control_panel_input::Builder {
        crate::input::update_control_panel_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateControlPanel {
    type Output = std::result::Result<
        crate::output::UpdateControlPanelOutput,
        crate::error::UpdateControlPanelError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_control_panel_error(response)
        } else {
            crate::operation_deser::parse_update_control_panel_response(response)
        }
    }
}

/// <p>Updates a routing control. You can only update the name of the routing control. To get or update the routing control state, see the Recovery Cluster (data plane) API actions for Amazon Route 53 Application Recovery Controller.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateRoutingControl {
    _private: (),
}
impl UpdateRoutingControl {
    /// Creates a new builder-style object to manufacture [`UpdateRoutingControlInput`](crate::input::UpdateRoutingControlInput)
    pub fn builder() -> crate::input::update_routing_control_input::Builder {
        crate::input::update_routing_control_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateRoutingControl {
    type Output = std::result::Result<
        crate::output::UpdateRoutingControlOutput,
        crate::error::UpdateRoutingControlError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_routing_control_error(response)
        } else {
            crate::operation_deser::parse_update_routing_control_response(response)
        }
    }
}

/// <p>Update a safety rule (an assertion rule or gating rule) for the routing controls in a control panel. You can only update the name and the waiting period for a safety rule. To make other updates, delete the safety rule and create a new safety rule.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateSafetyRule {
    _private: (),
}
impl UpdateSafetyRule {
    /// Creates a new builder-style object to manufacture [`UpdateSafetyRuleInput`](crate::input::UpdateSafetyRuleInput)
    pub fn builder() -> crate::input::update_safety_rule_input::Builder {
        crate::input::update_safety_rule_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateSafetyRule {
    type Output = std::result::Result<
        crate::output::UpdateSafetyRuleOutput,
        crate::error::UpdateSafetyRuleError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_safety_rule_error(response)
        } else {
            crate::operation_deser::parse_update_safety_rule_response(response)
        }
    }
}