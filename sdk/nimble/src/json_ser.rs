// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_accept_eulas_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AcceptEulasInput,
) {
    if let Some(var_1) = &input.eula_ids {
        let mut array_2 = object.key("eulaIds").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3);
            }
        }
        array_2.finish();
    }
}

pub fn serialize_structure_create_launch_profile_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateLaunchProfileInput,
) {
    if let Some(var_4) = &input.description {
        object.key("description").string(var_4);
    }
    if let Some(var_5) = &input.ec2_subnet_ids {
        let mut array_6 = object.key("ec2SubnetIds").start_array();
        for item_7 in var_5 {
            {
                array_6.value().string(item_7);
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.launch_profile_protocol_versions {
        let mut array_9 = object.key("launchProfileProtocolVersions").start_array();
        for item_10 in var_8 {
            {
                array_9.value().string(item_10);
            }
        }
        array_9.finish();
    }
    if let Some(var_11) = &input.name {
        object.key("name").string(var_11);
    }
    if let Some(var_12) = &input.stream_configuration {
        let mut object_13 = object.key("streamConfiguration").start_object();
        crate::json_ser::serialize_structure_stream_configuration_create(&mut object_13, var_12);
        object_13.finish();
    }
    if let Some(var_14) = &input.studio_component_ids {
        let mut array_15 = object.key("studioComponentIds").start_array();
        for item_16 in var_14 {
            {
                array_15.value().string(item_16);
            }
        }
        array_15.finish();
    }
    if let Some(var_17) = &input.tags {
        let mut object_18 = object.key("tags").start_object();
        for (key_19, value_20) in var_17 {
            {
                object_18.key(key_19).string(value_20);
            }
        }
        object_18.finish();
    }
}

pub fn serialize_structure_create_streaming_image_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateStreamingImageInput,
) {
    if let Some(var_21) = &input.description {
        object.key("description").string(var_21);
    }
    if let Some(var_22) = &input.ec2_image_id {
        object.key("ec2ImageId").string(var_22);
    }
    if let Some(var_23) = &input.name {
        object.key("name").string(var_23);
    }
    if let Some(var_24) = &input.tags {
        let mut object_25 = object.key("tags").start_object();
        for (key_26, value_27) in var_24 {
            {
                object_25.key(key_26).string(value_27);
            }
        }
        object_25.finish();
    }
}

pub fn serialize_structure_create_streaming_session_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateStreamingSessionInput,
) {
    if let Some(var_28) = &input.ec2_instance_type {
        object.key("ec2InstanceType").string(var_28.as_str());
    }
    if let Some(var_29) = &input.launch_profile_id {
        object.key("launchProfileId").string(var_29);
    }
    if let Some(var_30) = &input.streaming_image_id {
        object.key("streamingImageId").string(var_30);
    }
    if let Some(var_31) = &input.tags {
        let mut object_32 = object.key("tags").start_object();
        for (key_33, value_34) in var_31 {
            {
                object_32.key(key_33).string(value_34);
            }
        }
        object_32.finish();
    }
}

pub fn serialize_structure_create_streaming_session_stream_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateStreamingSessionStreamInput,
) {
    if input.expiration_in_seconds != 0 {
        object.key("expirationInSeconds").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.expiration_in_seconds).into()),
        );
    }
}

pub fn serialize_structure_create_studio_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateStudioInput,
) {
    if let Some(var_35) = &input.admin_role_arn {
        object.key("adminRoleArn").string(var_35);
    }
    if let Some(var_36) = &input.display_name {
        object.key("displayName").string(var_36);
    }
    if let Some(var_37) = &input.studio_encryption_configuration {
        let mut object_38 = object.key("studioEncryptionConfiguration").start_object();
        crate::json_ser::serialize_structure_studio_encryption_configuration(
            &mut object_38,
            var_37,
        );
        object_38.finish();
    }
    if let Some(var_39) = &input.studio_name {
        object.key("studioName").string(var_39);
    }
    if let Some(var_40) = &input.tags {
        let mut object_41 = object.key("tags").start_object();
        for (key_42, value_43) in var_40 {
            {
                object_41.key(key_42).string(value_43);
            }
        }
        object_41.finish();
    }
    if let Some(var_44) = &input.user_role_arn {
        object.key("userRoleArn").string(var_44);
    }
}

pub fn serialize_structure_create_studio_component_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateStudioComponentInput,
) {
    if let Some(var_45) = &input.configuration {
        let mut object_46 = object.key("configuration").start_object();
        crate::json_ser::serialize_structure_studio_component_configuration(&mut object_46, var_45);
        object_46.finish();
    }
    if let Some(var_47) = &input.description {
        object.key("description").string(var_47);
    }
    if let Some(var_48) = &input.ec2_security_group_ids {
        let mut array_49 = object.key("ec2SecurityGroupIds").start_array();
        for item_50 in var_48 {
            {
                array_49.value().string(item_50);
            }
        }
        array_49.finish();
    }
    if let Some(var_51) = &input.initialization_scripts {
        let mut array_52 = object.key("initializationScripts").start_array();
        for item_53 in var_51 {
            {
                let mut object_54 = array_52.value().start_object();
                crate::json_ser::serialize_structure_studio_component_initialization_script(
                    &mut object_54,
                    item_53,
                );
                object_54.finish();
            }
        }
        array_52.finish();
    }
    if let Some(var_55) = &input.name {
        object.key("name").string(var_55);
    }
    if let Some(var_56) = &input.script_parameters {
        let mut array_57 = object.key("scriptParameters").start_array();
        for item_58 in var_56 {
            {
                let mut object_59 = array_57.value().start_object();
                crate::json_ser::serialize_structure_script_parameter_key_value(
                    &mut object_59,
                    item_58,
                );
                object_59.finish();
            }
        }
        array_57.finish();
    }
    if let Some(var_60) = &input.subtype {
        object.key("subtype").string(var_60.as_str());
    }
    if let Some(var_61) = &input.tags {
        let mut object_62 = object.key("tags").start_object();
        for (key_63, value_64) in var_61 {
            {
                object_62.key(key_63).string(value_64);
            }
        }
        object_62.finish();
    }
    if let Some(var_65) = &input.r#type {
        object.key("type").string(var_65.as_str());
    }
}

pub fn serialize_structure_put_launch_profile_members_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutLaunchProfileMembersInput,
) {
    if let Some(var_66) = &input.identity_store_id {
        object.key("identityStoreId").string(var_66);
    }
    if let Some(var_67) = &input.members {
        let mut array_68 = object.key("members").start_array();
        for item_69 in var_67 {
            {
                let mut object_70 = array_68.value().start_object();
                crate::json_ser::serialize_structure_new_launch_profile_member(
                    &mut object_70,
                    item_69,
                );
                object_70.finish();
            }
        }
        array_68.finish();
    }
}

pub fn serialize_structure_put_studio_members_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutStudioMembersInput,
) {
    if let Some(var_71) = &input.identity_store_id {
        object.key("identityStoreId").string(var_71);
    }
    if let Some(var_72) = &input.members {
        let mut array_73 = object.key("members").start_array();
        for item_74 in var_72 {
            {
                let mut object_75 = array_73.value().start_object();
                crate::json_ser::serialize_structure_new_studio_member(&mut object_75, item_74);
                object_75.finish();
            }
        }
        array_73.finish();
    }
}

pub fn serialize_structure_tag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_76) = &input.tags {
        let mut object_77 = object.key("tags").start_object();
        for (key_78, value_79) in var_76 {
            {
                object_77.key(key_78).string(value_79);
            }
        }
        object_77.finish();
    }
}

pub fn serialize_structure_update_launch_profile_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateLaunchProfileInput,
) {
    if let Some(var_80) = &input.description {
        object.key("description").string(var_80);
    }
    if let Some(var_81) = &input.launch_profile_protocol_versions {
        let mut array_82 = object.key("launchProfileProtocolVersions").start_array();
        for item_83 in var_81 {
            {
                array_82.value().string(item_83);
            }
        }
        array_82.finish();
    }
    if let Some(var_84) = &input.name {
        object.key("name").string(var_84);
    }
    if let Some(var_85) = &input.stream_configuration {
        let mut object_86 = object.key("streamConfiguration").start_object();
        crate::json_ser::serialize_structure_stream_configuration_create(&mut object_86, var_85);
        object_86.finish();
    }
    if let Some(var_87) = &input.studio_component_ids {
        let mut array_88 = object.key("studioComponentIds").start_array();
        for item_89 in var_87 {
            {
                array_88.value().string(item_89);
            }
        }
        array_88.finish();
    }
}

pub fn serialize_structure_update_launch_profile_member_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateLaunchProfileMemberInput,
) {
    if let Some(var_90) = &input.persona {
        object.key("persona").string(var_90.as_str());
    }
}

pub fn serialize_structure_update_streaming_image_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateStreamingImageInput,
) {
    if let Some(var_91) = &input.description {
        object.key("description").string(var_91);
    }
    if let Some(var_92) = &input.name {
        object.key("name").string(var_92);
    }
}

pub fn serialize_structure_update_studio_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateStudioInput,
) {
    if let Some(var_93) = &input.admin_role_arn {
        object.key("adminRoleArn").string(var_93);
    }
    if let Some(var_94) = &input.display_name {
        object.key("displayName").string(var_94);
    }
    if let Some(var_95) = &input.user_role_arn {
        object.key("userRoleArn").string(var_95);
    }
}

pub fn serialize_structure_update_studio_component_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateStudioComponentInput,
) {
    if let Some(var_96) = &input.configuration {
        let mut object_97 = object.key("configuration").start_object();
        crate::json_ser::serialize_structure_studio_component_configuration(&mut object_97, var_96);
        object_97.finish();
    }
    if let Some(var_98) = &input.description {
        object.key("description").string(var_98);
    }
    if let Some(var_99) = &input.ec2_security_group_ids {
        let mut array_100 = object.key("ec2SecurityGroupIds").start_array();
        for item_101 in var_99 {
            {
                array_100.value().string(item_101);
            }
        }
        array_100.finish();
    }
    if let Some(var_102) = &input.initialization_scripts {
        let mut array_103 = object.key("initializationScripts").start_array();
        for item_104 in var_102 {
            {
                let mut object_105 = array_103.value().start_object();
                crate::json_ser::serialize_structure_studio_component_initialization_script(
                    &mut object_105,
                    item_104,
                );
                object_105.finish();
            }
        }
        array_103.finish();
    }
    if let Some(var_106) = &input.name {
        object.key("name").string(var_106);
    }
    if let Some(var_107) = &input.script_parameters {
        let mut array_108 = object.key("scriptParameters").start_array();
        for item_109 in var_107 {
            {
                let mut object_110 = array_108.value().start_object();
                crate::json_ser::serialize_structure_script_parameter_key_value(
                    &mut object_110,
                    item_109,
                );
                object_110.finish();
            }
        }
        array_108.finish();
    }
    if let Some(var_111) = &input.subtype {
        object.key("subtype").string(var_111.as_str());
    }
    if let Some(var_112) = &input.r#type {
        object.key("type").string(var_112.as_str());
    }
}

pub fn serialize_structure_stream_configuration_create(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::StreamConfigurationCreate,
) {
    if let Some(var_113) = &input.clipboard_mode {
        object.key("clipboardMode").string(var_113.as_str());
    }
    if let Some(var_114) = &input.ec2_instance_types {
        let mut array_115 = object.key("ec2InstanceTypes").start_array();
        for item_116 in var_114 {
            {
                array_115.value().string(item_116.as_str());
            }
        }
        array_115.finish();
    }
    if input.max_session_length_in_minutes != 0 {
        object.key("maxSessionLengthInMinutes").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((input.max_session_length_in_minutes).into()),
        );
    }
    if let Some(var_117) = &input.streaming_image_ids {
        let mut array_118 = object.key("streamingImageIds").start_array();
        for item_119 in var_117 {
            {
                array_118.value().string(item_119);
            }
        }
        array_118.finish();
    }
}

pub fn serialize_structure_studio_encryption_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::StudioEncryptionConfiguration,
) {
    if let Some(var_120) = &input.key_arn {
        object.key("keyArn").string(var_120);
    }
    if let Some(var_121) = &input.key_type {
        object.key("keyType").string(var_121.as_str());
    }
}

pub fn serialize_structure_studio_component_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::StudioComponentConfiguration,
) {
    if let Some(var_122) = &input.active_directory_configuration {
        let mut object_123 = object.key("activeDirectoryConfiguration").start_object();
        crate::json_ser::serialize_structure_active_directory_configuration(
            &mut object_123,
            var_122,
        );
        object_123.finish();
    }
    if let Some(var_124) = &input.compute_farm_configuration {
        let mut object_125 = object.key("computeFarmConfiguration").start_object();
        crate::json_ser::serialize_structure_compute_farm_configuration(&mut object_125, var_124);
        object_125.finish();
    }
    if let Some(var_126) = &input.license_service_configuration {
        let mut object_127 = object.key("licenseServiceConfiguration").start_object();
        crate::json_ser::serialize_structure_license_service_configuration(
            &mut object_127,
            var_126,
        );
        object_127.finish();
    }
    if let Some(var_128) = &input.shared_file_system_configuration {
        let mut object_129 = object.key("sharedFileSystemConfiguration").start_object();
        crate::json_ser::serialize_structure_shared_file_system_configuration(
            &mut object_129,
            var_128,
        );
        object_129.finish();
    }
}

pub fn serialize_structure_studio_component_initialization_script(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::StudioComponentInitializationScript,
) {
    if let Some(var_130) = &input.launch_profile_protocol_version {
        object.key("launchProfileProtocolVersion").string(var_130);
    }
    if let Some(var_131) = &input.platform {
        object.key("platform").string(var_131.as_str());
    }
    if let Some(var_132) = &input.run_context {
        object.key("runContext").string(var_132.as_str());
    }
    if let Some(var_133) = &input.script {
        object.key("script").string(var_133);
    }
}

pub fn serialize_structure_script_parameter_key_value(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ScriptParameterKeyValue,
) {
    if let Some(var_134) = &input.key {
        object.key("key").string(var_134);
    }
    if let Some(var_135) = &input.value {
        object.key("value").string(var_135);
    }
}

pub fn serialize_structure_new_launch_profile_member(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NewLaunchProfileMember,
) {
    if let Some(var_136) = &input.persona {
        object.key("persona").string(var_136.as_str());
    }
    if let Some(var_137) = &input.principal_id {
        object.key("principalId").string(var_137);
    }
}

pub fn serialize_structure_new_studio_member(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NewStudioMember,
) {
    if let Some(var_138) = &input.persona {
        object.key("persona").string(var_138.as_str());
    }
    if let Some(var_139) = &input.principal_id {
        object.key("principalId").string(var_139);
    }
}

pub fn serialize_structure_active_directory_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ActiveDirectoryConfiguration,
) {
    if let Some(var_140) = &input.computer_attributes {
        let mut array_141 = object.key("computerAttributes").start_array();
        for item_142 in var_140 {
            {
                let mut object_143 = array_141.value().start_object();
                crate::json_ser::serialize_structure_active_directory_computer_attribute(
                    &mut object_143,
                    item_142,
                );
                object_143.finish();
            }
        }
        array_141.finish();
    }
    if let Some(var_144) = &input.directory_id {
        object.key("directoryId").string(var_144);
    }
    if let Some(var_145) = &input.organizational_unit_distinguished_name {
        object
            .key("organizationalUnitDistinguishedName")
            .string(var_145);
    }
}

pub fn serialize_structure_compute_farm_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ComputeFarmConfiguration,
) {
    if let Some(var_146) = &input.active_directory_user {
        object.key("activeDirectoryUser").string(var_146);
    }
    if let Some(var_147) = &input.endpoint {
        object.key("endpoint").string(var_147);
    }
}

pub fn serialize_structure_license_service_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LicenseServiceConfiguration,
) {
    if let Some(var_148) = &input.endpoint {
        object.key("endpoint").string(var_148);
    }
}

pub fn serialize_structure_shared_file_system_configuration(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SharedFileSystemConfiguration,
) {
    if let Some(var_149) = &input.endpoint {
        object.key("endpoint").string(var_149);
    }
    if let Some(var_150) = &input.file_system_id {
        object.key("fileSystemId").string(var_150);
    }
    if let Some(var_151) = &input.linux_mount_point {
        object.key("linuxMountPoint").string(var_151);
    }
    if let Some(var_152) = &input.share_name {
        object.key("shareName").string(var_152);
    }
    if let Some(var_153) = &input.windows_mount_drive {
        object.key("windowsMountDrive").string(var_153);
    }
}

pub fn serialize_structure_active_directory_computer_attribute(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ActiveDirectoryComputerAttribute,
) {
    if let Some(var_154) = &input.name {
        object.key("name").string(var_154);
    }
    if let Some(var_155) = &input.value {
        object.key("value").string(var_155);
    }
}