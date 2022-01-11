// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_describe_db_cluster_endpoints_output_marker(
    input: &crate::output::DescribeDbClusterEndpointsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.marker {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_db_cluster_parameter_groups_output_marker(
    input: &crate::output::DescribeDbClusterParameterGroupsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.marker {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_db_cluster_parameters_output_marker(
    input: &crate::output::DescribeDbClusterParametersOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.marker {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_db_clusters_output_marker(
    input: &crate::output::DescribeDbClustersOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.marker {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_db_cluster_snapshots_output_marker(
    input: &crate::output::DescribeDbClusterSnapshotsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.marker {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_db_engine_versions_output_marker(
    input: &crate::output::DescribeDbEngineVersionsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.marker {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_db_instances_output_marker(
    input: &crate::output::DescribeDbInstancesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.marker {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_db_parameter_groups_output_marker(
    input: &crate::output::DescribeDbParameterGroupsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.marker {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_db_parameters_output_marker(
    input: &crate::output::DescribeDbParametersOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.marker {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_db_subnet_groups_output_marker(
    input: &crate::output::DescribeDbSubnetGroupsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.marker {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_engine_default_parameters_output_engine_defaults_marker(
    input: &crate::output::DescribeEngineDefaultParametersOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.engine_defaults {
        None => return None,
        Some(t) => t,
    };
    let input = match &input.marker {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_events_output_marker(
    input: &crate::output::DescribeEventsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.marker {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_event_subscriptions_output_marker(
    input: &crate::output::DescribeEventSubscriptionsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.marker {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_orderable_db_instance_options_output_marker(
    input: &crate::output::DescribeOrderableDbInstanceOptionsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.marker {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_describe_pending_maintenance_actions_output_marker(
    input: &crate::output::DescribePendingMaintenanceActionsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.marker {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_db_cluster_endpoints_output_db_cluster_endpoints(
    input: crate::output::DescribeDbClusterEndpointsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::DbClusterEndpoint>> {
    let input = match input.db_cluster_endpoints {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_db_cluster_parameter_groups_output_db_cluster_parameter_groups(
    input: crate::output::DescribeDbClusterParameterGroupsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::DbClusterParameterGroup>> {
    let input = match input.db_cluster_parameter_groups {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_db_cluster_parameters_output_parameters(
    input: crate::output::DescribeDbClusterParametersOutput,
) -> std::option::Option<std::vec::Vec<crate::model::Parameter>> {
    let input = match input.parameters {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_db_clusters_output_db_clusters(
    input: crate::output::DescribeDbClustersOutput,
) -> std::option::Option<std::vec::Vec<crate::model::DbCluster>> {
    let input = match input.db_clusters {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_db_cluster_snapshots_output_db_cluster_snapshots(
    input: crate::output::DescribeDbClusterSnapshotsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::DbClusterSnapshot>> {
    let input = match input.db_cluster_snapshots {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_db_engine_versions_output_db_engine_versions(
    input: crate::output::DescribeDbEngineVersionsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::DbEngineVersion>> {
    let input = match input.db_engine_versions {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_db_instances_output_db_instances(
    input: crate::output::DescribeDbInstancesOutput,
) -> std::option::Option<std::vec::Vec<crate::model::DbInstance>> {
    let input = match input.db_instances {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_db_parameter_groups_output_db_parameter_groups(
    input: crate::output::DescribeDbParameterGroupsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::DbParameterGroup>> {
    let input = match input.db_parameter_groups {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_db_parameters_output_parameters(
    input: crate::output::DescribeDbParametersOutput,
) -> std::option::Option<std::vec::Vec<crate::model::Parameter>> {
    let input = match input.parameters {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_db_subnet_groups_output_db_subnet_groups(
    input: crate::output::DescribeDbSubnetGroupsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::DbSubnetGroup>> {
    let input = match input.db_subnet_groups {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_engine_default_parameters_output_engine_defaults_parameters(
    input: crate::output::DescribeEngineDefaultParametersOutput,
) -> std::option::Option<std::vec::Vec<crate::model::Parameter>> {
    let input = match input.engine_defaults {
        None => return None,
        Some(t) => t,
    };
    let input = match input.parameters {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_events_output_events(
    input: crate::output::DescribeEventsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::Event>> {
    let input = match input.events {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_event_subscriptions_output_event_subscriptions_list(
    input: crate::output::DescribeEventSubscriptionsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::EventSubscription>> {
    let input = match input.event_subscriptions_list {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_orderable_db_instance_options_output_orderable_db_instance_options(
    input: crate::output::DescribeOrderableDbInstanceOptionsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::OrderableDbInstanceOption>> {
    let input = match input.orderable_db_instance_options {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_describe_pending_maintenance_actions_output_pending_maintenance_actions(
    input: crate::output::DescribePendingMaintenanceActionsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::ResourcePendingMaintenanceActions>> {
    let input = match input.pending_maintenance_actions {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}