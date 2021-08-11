// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_create_job_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateJobInput,
) {
    if let Some(var_1) = &input.input {
        let mut object_2 = object.key("Input").start_object();
        crate::json_ser::serialize_structure_job_input(&mut object_2, var_1);
        object_2.finish();
    }
    if let Some(var_3) = &input.inputs {
        let mut array_4 = object.key("Inputs").start_array();
        for item_5 in var_3 {
            {
                let mut object_6 = array_4.value().start_object();
                crate::json_ser::serialize_structure_job_input(&mut object_6, item_5);
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.output {
        let mut object_8 = object.key("Output").start_object();
        crate::json_ser::serialize_structure_create_job_output(&mut object_8, var_7);
        object_8.finish();
    }
    if let Some(var_9) = &input.output_key_prefix {
        object.key("OutputKeyPrefix").string(var_9);
    }
    if let Some(var_10) = &input.outputs {
        let mut array_11 = object.key("Outputs").start_array();
        for item_12 in var_10 {
            {
                let mut object_13 = array_11.value().start_object();
                crate::json_ser::serialize_structure_create_job_output(&mut object_13, item_12);
                object_13.finish();
            }
        }
        array_11.finish();
    }
    if let Some(var_14) = &input.pipeline_id {
        object.key("PipelineId").string(var_14);
    }
    if let Some(var_15) = &input.playlists {
        let mut array_16 = object.key("Playlists").start_array();
        for item_17 in var_15 {
            {
                let mut object_18 = array_16.value().start_object();
                crate::json_ser::serialize_structure_create_job_playlist(&mut object_18, item_17);
                object_18.finish();
            }
        }
        array_16.finish();
    }
    if let Some(var_19) = &input.user_metadata {
        let mut object_20 = object.key("UserMetadata").start_object();
        for (key_21, value_22) in var_19 {
            {
                object_20.key(key_21).string(value_22);
            }
        }
        object_20.finish();
    }
}

pub fn serialize_structure_create_pipeline_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreatePipelineInput,
) {
    if let Some(var_23) = &input.aws_kms_key_arn {
        object.key("AwsKmsKeyArn").string(var_23);
    }
    if let Some(var_24) = &input.content_config {
        let mut object_25 = object.key("ContentConfig").start_object();
        crate::json_ser::serialize_structure_pipeline_output_config(&mut object_25, var_24);
        object_25.finish();
    }
    if let Some(var_26) = &input.input_bucket {
        object.key("InputBucket").string(var_26);
    }
    if let Some(var_27) = &input.name {
        object.key("Name").string(var_27);
    }
    if let Some(var_28) = &input.notifications {
        let mut object_29 = object.key("Notifications").start_object();
        crate::json_ser::serialize_structure_notifications(&mut object_29, var_28);
        object_29.finish();
    }
    if let Some(var_30) = &input.output_bucket {
        object.key("OutputBucket").string(var_30);
    }
    if let Some(var_31) = &input.role {
        object.key("Role").string(var_31);
    }
    if let Some(var_32) = &input.thumbnail_config {
        let mut object_33 = object.key("ThumbnailConfig").start_object();
        crate::json_ser::serialize_structure_pipeline_output_config(&mut object_33, var_32);
        object_33.finish();
    }
}

pub fn serialize_structure_create_preset_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreatePresetInput,
) {
    if let Some(var_34) = &input.audio {
        let mut object_35 = object.key("Audio").start_object();
        crate::json_ser::serialize_structure_audio_parameters(&mut object_35, var_34);
        object_35.finish();
    }
    if let Some(var_36) = &input.container {
        object.key("Container").string(var_36);
    }
    if let Some(var_37) = &input.description {
        object.key("Description").string(var_37);
    }
    if let Some(var_38) = &input.name {
        object.key("Name").string(var_38);
    }
    if let Some(var_39) = &input.thumbnails {
        let mut object_40 = object.key("Thumbnails").start_object();
        crate::json_ser::serialize_structure_thumbnails(&mut object_40, var_39);
        object_40.finish();
    }
    if let Some(var_41) = &input.video {
        let mut object_42 = object.key("Video").start_object();
        crate::json_ser::serialize_structure_video_parameters(&mut object_42, var_41);
        object_42.finish();
    }
}

pub fn serialize_structure_test_role_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TestRoleInput,
) {
    if let Some(var_43) = &input.input_bucket {
        object.key("InputBucket").string(var_43);
    }
    if let Some(var_44) = &input.output_bucket {
        object.key("OutputBucket").string(var_44);
    }
    if let Some(var_45) = &input.role {
        object.key("Role").string(var_45);
    }
    if let Some(var_46) = &input.topics {
        let mut array_47 = object.key("Topics").start_array();
        for item_48 in var_46 {
            {
                array_47.value().string(item_48);
            }
        }
        array_47.finish();
    }
}

pub fn serialize_structure_update_pipeline_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdatePipelineInput,
) {
    if let Some(var_49) = &input.aws_kms_key_arn {
        object.key("AwsKmsKeyArn").string(var_49);
    }
    if let Some(var_50) = &input.content_config {
        let mut object_51 = object.key("ContentConfig").start_object();
        crate::json_ser::serialize_structure_pipeline_output_config(&mut object_51, var_50);
        object_51.finish();
    }
    if let Some(var_52) = &input.input_bucket {
        object.key("InputBucket").string(var_52);
    }
    if let Some(var_53) = &input.name {
        object.key("Name").string(var_53);
    }
    if let Some(var_54) = &input.notifications {
        let mut object_55 = object.key("Notifications").start_object();
        crate::json_ser::serialize_structure_notifications(&mut object_55, var_54);
        object_55.finish();
    }
    if let Some(var_56) = &input.role {
        object.key("Role").string(var_56);
    }
    if let Some(var_57) = &input.thumbnail_config {
        let mut object_58 = object.key("ThumbnailConfig").start_object();
        crate::json_ser::serialize_structure_pipeline_output_config(&mut object_58, var_57);
        object_58.finish();
    }
}

pub fn serialize_structure_update_pipeline_notifications_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdatePipelineNotificationsInput,
) {
    if let Some(var_59) = &input.notifications {
        let mut object_60 = object.key("Notifications").start_object();
        crate::json_ser::serialize_structure_notifications(&mut object_60, var_59);
        object_60.finish();
    }
}

pub fn serialize_structure_update_pipeline_status_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdatePipelineStatusInput,
) {
    if let Some(var_61) = &input.status {
        object.key("Status").string(var_61);
    }
}

pub fn serialize_structure_job_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::JobInput,
) {
    if let Some(var_62) = &input.key {
        object.key("Key").string(var_62);
    }
    if let Some(var_63) = &input.frame_rate {
        object.key("FrameRate").string(var_63);
    }
    if let Some(var_64) = &input.resolution {
        object.key("Resolution").string(var_64);
    }
    if let Some(var_65) = &input.aspect_ratio {
        object.key("AspectRatio").string(var_65);
    }
    if let Some(var_66) = &input.interlaced {
        object.key("Interlaced").string(var_66);
    }
    if let Some(var_67) = &input.container {
        object.key("Container").string(var_67);
    }
    if let Some(var_68) = &input.encryption {
        let mut object_69 = object.key("Encryption").start_object();
        crate::json_ser::serialize_structure_encryption(&mut object_69, var_68);
        object_69.finish();
    }
    if let Some(var_70) = &input.time_span {
        let mut object_71 = object.key("TimeSpan").start_object();
        crate::json_ser::serialize_structure_time_span(&mut object_71, var_70);
        object_71.finish();
    }
    if let Some(var_72) = &input.input_captions {
        let mut object_73 = object.key("InputCaptions").start_object();
        crate::json_ser::serialize_structure_input_captions(&mut object_73, var_72);
        object_73.finish();
    }
    if let Some(var_74) = &input.detected_properties {
        let mut object_75 = object.key("DetectedProperties").start_object();
        crate::json_ser::serialize_structure_detected_properties(&mut object_75, var_74);
        object_75.finish();
    }
}

pub fn serialize_structure_create_job_output(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::output::CreateJobOutput,
) {
    if let Some(var_76) = &input.job {
        let mut object_77 = object.key("Job").start_object();
        crate::json_ser::serialize_structure_job(&mut object_77, var_76);
        object_77.finish();
    }
}

pub fn serialize_structure_create_job_playlist(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CreateJobPlaylist,
) {
    if let Some(var_78) = &input.name {
        object.key("Name").string(var_78);
    }
    if let Some(var_79) = &input.format {
        object.key("Format").string(var_79);
    }
    if let Some(var_80) = &input.output_keys {
        let mut array_81 = object.key("OutputKeys").start_array();
        for item_82 in var_80 {
            {
                array_81.value().string(item_82);
            }
        }
        array_81.finish();
    }
    if let Some(var_83) = &input.hls_content_protection {
        let mut object_84 = object.key("HlsContentProtection").start_object();
        crate::json_ser::serialize_structure_hls_content_protection(&mut object_84, var_83);
        object_84.finish();
    }
    if let Some(var_85) = &input.play_ready_drm {
        let mut object_86 = object.key("PlayReadyDrm").start_object();
        crate::json_ser::serialize_structure_play_ready_drm(&mut object_86, var_85);
        object_86.finish();
    }
}

pub fn serialize_structure_pipeline_output_config(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PipelineOutputConfig,
) {
    if let Some(var_87) = &input.bucket {
        object.key("Bucket").string(var_87);
    }
    if let Some(var_88) = &input.storage_class {
        object.key("StorageClass").string(var_88);
    }
    if let Some(var_89) = &input.permissions {
        let mut array_90 = object.key("Permissions").start_array();
        for item_91 in var_89 {
            {
                let mut object_92 = array_90.value().start_object();
                crate::json_ser::serialize_structure_permission(&mut object_92, item_91);
                object_92.finish();
            }
        }
        array_90.finish();
    }
}

pub fn serialize_structure_notifications(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Notifications,
) {
    if let Some(var_93) = &input.progressing {
        object.key("Progressing").string(var_93);
    }
    if let Some(var_94) = &input.completed {
        object.key("Completed").string(var_94);
    }
    if let Some(var_95) = &input.warning {
        object.key("Warning").string(var_95);
    }
    if let Some(var_96) = &input.error {
        object.key("Error").string(var_96);
    }
}

pub fn serialize_structure_audio_parameters(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AudioParameters,
) {
    if let Some(var_97) = &input.codec {
        object.key("Codec").string(var_97);
    }
    if let Some(var_98) = &input.sample_rate {
        object.key("SampleRate").string(var_98);
    }
    if let Some(var_99) = &input.bit_rate {
        object.key("BitRate").string(var_99);
    }
    if let Some(var_100) = &input.channels {
        object.key("Channels").string(var_100);
    }
    if let Some(var_101) = &input.audio_packing_mode {
        object.key("AudioPackingMode").string(var_101);
    }
    if let Some(var_102) = &input.codec_options {
        let mut object_103 = object.key("CodecOptions").start_object();
        crate::json_ser::serialize_structure_audio_codec_options(&mut object_103, var_102);
        object_103.finish();
    }
}

pub fn serialize_structure_thumbnails(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Thumbnails,
) {
    if let Some(var_104) = &input.format {
        object.key("Format").string(var_104);
    }
    if let Some(var_105) = &input.interval {
        object.key("Interval").string(var_105);
    }
    if let Some(var_106) = &input.resolution {
        object.key("Resolution").string(var_106);
    }
    if let Some(var_107) = &input.aspect_ratio {
        object.key("AspectRatio").string(var_107);
    }
    if let Some(var_108) = &input.max_width {
        object.key("MaxWidth").string(var_108);
    }
    if let Some(var_109) = &input.max_height {
        object.key("MaxHeight").string(var_109);
    }
    if let Some(var_110) = &input.sizing_policy {
        object.key("SizingPolicy").string(var_110);
    }
    if let Some(var_111) = &input.padding_policy {
        object.key("PaddingPolicy").string(var_111);
    }
}

pub fn serialize_structure_video_parameters(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::VideoParameters,
) {
    if let Some(var_112) = &input.codec {
        object.key("Codec").string(var_112);
    }
    if let Some(var_113) = &input.codec_options {
        let mut object_114 = object.key("CodecOptions").start_object();
        for (key_115, value_116) in var_113 {
            {
                object_114.key(key_115).string(value_116);
            }
        }
        object_114.finish();
    }
    if let Some(var_117) = &input.keyframes_max_dist {
        object.key("KeyframesMaxDist").string(var_117);
    }
    if let Some(var_118) = &input.fixed_gop {
        object.key("FixedGOP").string(var_118);
    }
    if let Some(var_119) = &input.bit_rate {
        object.key("BitRate").string(var_119);
    }
    if let Some(var_120) = &input.frame_rate {
        object.key("FrameRate").string(var_120);
    }
    if let Some(var_121) = &input.max_frame_rate {
        object.key("MaxFrameRate").string(var_121);
    }
    if let Some(var_122) = &input.resolution {
        object.key("Resolution").string(var_122);
    }
    if let Some(var_123) = &input.aspect_ratio {
        object.key("AspectRatio").string(var_123);
    }
    if let Some(var_124) = &input.max_width {
        object.key("MaxWidth").string(var_124);
    }
    if let Some(var_125) = &input.max_height {
        object.key("MaxHeight").string(var_125);
    }
    if let Some(var_126) = &input.display_aspect_ratio {
        object.key("DisplayAspectRatio").string(var_126);
    }
    if let Some(var_127) = &input.sizing_policy {
        object.key("SizingPolicy").string(var_127);
    }
    if let Some(var_128) = &input.padding_policy {
        object.key("PaddingPolicy").string(var_128);
    }
    if let Some(var_129) = &input.watermarks {
        let mut array_130 = object.key("Watermarks").start_array();
        for item_131 in var_129 {
            {
                let mut object_132 = array_130.value().start_object();
                crate::json_ser::serialize_structure_preset_watermark(&mut object_132, item_131);
                object_132.finish();
            }
        }
        array_130.finish();
    }
}

pub fn serialize_structure_encryption(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Encryption,
) {
    if let Some(var_133) = &input.mode {
        object.key("Mode").string(var_133);
    }
    if let Some(var_134) = &input.key {
        object.key("Key").string(var_134);
    }
    if let Some(var_135) = &input.key_md5 {
        object.key("KeyMd5").string(var_135);
    }
    if let Some(var_136) = &input.initialization_vector {
        object.key("InitializationVector").string(var_136);
    }
}

pub fn serialize_structure_time_span(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TimeSpan,
) {
    if let Some(var_137) = &input.start_time {
        object.key("StartTime").string(var_137);
    }
    if let Some(var_138) = &input.duration {
        object.key("Duration").string(var_138);
    }
}

pub fn serialize_structure_input_captions(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InputCaptions,
) {
    if let Some(var_139) = &input.merge_policy {
        object.key("MergePolicy").string(var_139);
    }
    if let Some(var_140) = &input.caption_sources {
        let mut array_141 = object.key("CaptionSources").start_array();
        for item_142 in var_140 {
            {
                let mut object_143 = array_141.value().start_object();
                crate::json_ser::serialize_structure_caption_source(&mut object_143, item_142);
                object_143.finish();
            }
        }
        array_141.finish();
    }
}

pub fn serialize_structure_detected_properties(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DetectedProperties,
) {
    if let Some(var_144) = &input.width {
        object.key("Width").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_144).into()),
        );
    }
    if let Some(var_145) = &input.height {
        object.key("Height").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_145).into()),
        );
    }
    if let Some(var_146) = &input.frame_rate {
        object.key("FrameRate").string(var_146);
    }
    if let Some(var_147) = &input.file_size {
        object.key("FileSize").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_147).into()),
        );
    }
    if let Some(var_148) = &input.duration_millis {
        object.key("DurationMillis").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_148).into()),
        );
    }
}

pub fn serialize_structure_job(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Job,
) {
    if let Some(var_149) = &input.id {
        object.key("Id").string(var_149);
    }
    if let Some(var_150) = &input.arn {
        object.key("Arn").string(var_150);
    }
    if let Some(var_151) = &input.pipeline_id {
        object.key("PipelineId").string(var_151);
    }
    if let Some(var_152) = &input.input {
        let mut object_153 = object.key("Input").start_object();
        crate::json_ser::serialize_structure_job_input(&mut object_153, var_152);
        object_153.finish();
    }
    if let Some(var_154) = &input.inputs {
        let mut array_155 = object.key("Inputs").start_array();
        for item_156 in var_154 {
            {
                let mut object_157 = array_155.value().start_object();
                crate::json_ser::serialize_structure_job_input(&mut object_157, item_156);
                object_157.finish();
            }
        }
        array_155.finish();
    }
    if let Some(var_158) = &input.output {
        let mut object_159 = object.key("Output").start_object();
        crate::json_ser::serialize_structure_job_output(&mut object_159, var_158);
        object_159.finish();
    }
    if let Some(var_160) = &input.outputs {
        let mut array_161 = object.key("Outputs").start_array();
        for item_162 in var_160 {
            {
                let mut object_163 = array_161.value().start_object();
                crate::json_ser::serialize_structure_job_output(&mut object_163, item_162);
                object_163.finish();
            }
        }
        array_161.finish();
    }
    if let Some(var_164) = &input.output_key_prefix {
        object.key("OutputKeyPrefix").string(var_164);
    }
    if let Some(var_165) = &input.playlists {
        let mut array_166 = object.key("Playlists").start_array();
        for item_167 in var_165 {
            {
                let mut object_168 = array_166.value().start_object();
                crate::json_ser::serialize_structure_playlist(&mut object_168, item_167);
                object_168.finish();
            }
        }
        array_166.finish();
    }
    if let Some(var_169) = &input.status {
        object.key("Status").string(var_169);
    }
    if let Some(var_170) = &input.user_metadata {
        let mut object_171 = object.key("UserMetadata").start_object();
        for (key_172, value_173) in var_170 {
            {
                object_171.key(key_172).string(value_173);
            }
        }
        object_171.finish();
    }
    if let Some(var_174) = &input.timing {
        let mut object_175 = object.key("Timing").start_object();
        crate::json_ser::serialize_structure_timing(&mut object_175, var_174);
        object_175.finish();
    }
}

pub fn serialize_structure_hls_content_protection(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HlsContentProtection,
) {
    if let Some(var_176) = &input.method {
        object.key("Method").string(var_176);
    }
    if let Some(var_177) = &input.key {
        object.key("Key").string(var_177);
    }
    if let Some(var_178) = &input.key_md5 {
        object.key("KeyMd5").string(var_178);
    }
    if let Some(var_179) = &input.initialization_vector {
        object.key("InitializationVector").string(var_179);
    }
    if let Some(var_180) = &input.license_acquisition_url {
        object.key("LicenseAcquisitionUrl").string(var_180);
    }
    if let Some(var_181) = &input.key_storage_policy {
        object.key("KeyStoragePolicy").string(var_181);
    }
}

pub fn serialize_structure_play_ready_drm(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PlayReadyDrm,
) {
    if let Some(var_182) = &input.format {
        object.key("Format").string(var_182);
    }
    if let Some(var_183) = &input.key {
        object.key("Key").string(var_183);
    }
    if let Some(var_184) = &input.key_md5 {
        object.key("KeyMd5").string(var_184);
    }
    if let Some(var_185) = &input.key_id {
        object.key("KeyId").string(var_185);
    }
    if let Some(var_186) = &input.initialization_vector {
        object.key("InitializationVector").string(var_186);
    }
    if let Some(var_187) = &input.license_acquisition_url {
        object.key("LicenseAcquisitionUrl").string(var_187);
    }
}

pub fn serialize_structure_permission(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Permission,
) {
    if let Some(var_188) = &input.grantee_type {
        object.key("GranteeType").string(var_188);
    }
    if let Some(var_189) = &input.grantee {
        object.key("Grantee").string(var_189);
    }
    if let Some(var_190) = &input.access {
        let mut array_191 = object.key("Access").start_array();
        for item_192 in var_190 {
            {
                array_191.value().string(item_192);
            }
        }
        array_191.finish();
    }
}

pub fn serialize_structure_audio_codec_options(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AudioCodecOptions,
) {
    if let Some(var_193) = &input.profile {
        object.key("Profile").string(var_193);
    }
    if let Some(var_194) = &input.bit_depth {
        object.key("BitDepth").string(var_194);
    }
    if let Some(var_195) = &input.bit_order {
        object.key("BitOrder").string(var_195);
    }
    if let Some(var_196) = &input.signed {
        object.key("Signed").string(var_196);
    }
}

pub fn serialize_structure_preset_watermark(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PresetWatermark,
) {
    if let Some(var_197) = &input.id {
        object.key("Id").string(var_197);
    }
    if let Some(var_198) = &input.max_width {
        object.key("MaxWidth").string(var_198);
    }
    if let Some(var_199) = &input.max_height {
        object.key("MaxHeight").string(var_199);
    }
    if let Some(var_200) = &input.sizing_policy {
        object.key("SizingPolicy").string(var_200);
    }
    if let Some(var_201) = &input.horizontal_align {
        object.key("HorizontalAlign").string(var_201);
    }
    if let Some(var_202) = &input.horizontal_offset {
        object.key("HorizontalOffset").string(var_202);
    }
    if let Some(var_203) = &input.vertical_align {
        object.key("VerticalAlign").string(var_203);
    }
    if let Some(var_204) = &input.vertical_offset {
        object.key("VerticalOffset").string(var_204);
    }
    if let Some(var_205) = &input.opacity {
        object.key("Opacity").string(var_205);
    }
    if let Some(var_206) = &input.target {
        object.key("Target").string(var_206);
    }
}

pub fn serialize_structure_caption_source(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CaptionSource,
) {
    if let Some(var_207) = &input.key {
        object.key("Key").string(var_207);
    }
    if let Some(var_208) = &input.language {
        object.key("Language").string(var_208);
    }
    if let Some(var_209) = &input.time_offset {
        object.key("TimeOffset").string(var_209);
    }
    if let Some(var_210) = &input.label {
        object.key("Label").string(var_210);
    }
    if let Some(var_211) = &input.encryption {
        let mut object_212 = object.key("Encryption").start_object();
        crate::json_ser::serialize_structure_encryption(&mut object_212, var_211);
        object_212.finish();
    }
}

pub fn serialize_structure_job_output(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::JobOutput,
) {
    if let Some(var_213) = &input.id {
        object.key("Id").string(var_213);
    }
    if let Some(var_214) = &input.key {
        object.key("Key").string(var_214);
    }
    if let Some(var_215) = &input.thumbnail_pattern {
        object.key("ThumbnailPattern").string(var_215);
    }
    if let Some(var_216) = &input.thumbnail_encryption {
        let mut object_217 = object.key("ThumbnailEncryption").start_object();
        crate::json_ser::serialize_structure_encryption(&mut object_217, var_216);
        object_217.finish();
    }
    if let Some(var_218) = &input.rotate {
        object.key("Rotate").string(var_218);
    }
    if let Some(var_219) = &input.preset_id {
        object.key("PresetId").string(var_219);
    }
    if let Some(var_220) = &input.segment_duration {
        object.key("SegmentDuration").string(var_220);
    }
    if let Some(var_221) = &input.status {
        object.key("Status").string(var_221);
    }
    if let Some(var_222) = &input.status_detail {
        object.key("StatusDetail").string(var_222);
    }
    if let Some(var_223) = &input.duration {
        object.key("Duration").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_223).into()),
        );
    }
    if let Some(var_224) = &input.width {
        object.key("Width").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_224).into()),
        );
    }
    if let Some(var_225) = &input.height {
        object.key("Height").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_225).into()),
        );
    }
    if let Some(var_226) = &input.frame_rate {
        object.key("FrameRate").string(var_226);
    }
    if let Some(var_227) = &input.file_size {
        object.key("FileSize").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_227).into()),
        );
    }
    if let Some(var_228) = &input.duration_millis {
        object.key("DurationMillis").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_228).into()),
        );
    }
    if let Some(var_229) = &input.watermarks {
        let mut array_230 = object.key("Watermarks").start_array();
        for item_231 in var_229 {
            {
                let mut object_232 = array_230.value().start_object();
                crate::json_ser::serialize_structure_job_watermark(&mut object_232, item_231);
                object_232.finish();
            }
        }
        array_230.finish();
    }
    if let Some(var_233) = &input.album_art {
        let mut object_234 = object.key("AlbumArt").start_object();
        crate::json_ser::serialize_structure_job_album_art(&mut object_234, var_233);
        object_234.finish();
    }
    if let Some(var_235) = &input.composition {
        let mut array_236 = object.key("Composition").start_array();
        for item_237 in var_235 {
            {
                let mut object_238 = array_236.value().start_object();
                crate::json_ser::serialize_structure_clip(&mut object_238, item_237);
                object_238.finish();
            }
        }
        array_236.finish();
    }
    if let Some(var_239) = &input.captions {
        let mut object_240 = object.key("Captions").start_object();
        crate::json_ser::serialize_structure_captions(&mut object_240, var_239);
        object_240.finish();
    }
    if let Some(var_241) = &input.encryption {
        let mut object_242 = object.key("Encryption").start_object();
        crate::json_ser::serialize_structure_encryption(&mut object_242, var_241);
        object_242.finish();
    }
    if let Some(var_243) = &input.applied_color_space_conversion {
        object.key("AppliedColorSpaceConversion").string(var_243);
    }
}

pub fn serialize_structure_playlist(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Playlist,
) {
    if let Some(var_244) = &input.name {
        object.key("Name").string(var_244);
    }
    if let Some(var_245) = &input.format {
        object.key("Format").string(var_245);
    }
    if let Some(var_246) = &input.output_keys {
        let mut array_247 = object.key("OutputKeys").start_array();
        for item_248 in var_246 {
            {
                array_247.value().string(item_248);
            }
        }
        array_247.finish();
    }
    if let Some(var_249) = &input.hls_content_protection {
        let mut object_250 = object.key("HlsContentProtection").start_object();
        crate::json_ser::serialize_structure_hls_content_protection(&mut object_250, var_249);
        object_250.finish();
    }
    if let Some(var_251) = &input.play_ready_drm {
        let mut object_252 = object.key("PlayReadyDrm").start_object();
        crate::json_ser::serialize_structure_play_ready_drm(&mut object_252, var_251);
        object_252.finish();
    }
    if let Some(var_253) = &input.status {
        object.key("Status").string(var_253);
    }
    if let Some(var_254) = &input.status_detail {
        object.key("StatusDetail").string(var_254);
    }
}

pub fn serialize_structure_timing(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Timing,
) {
    if let Some(var_255) = &input.submit_time_millis {
        object.key("SubmitTimeMillis").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_255).into()),
        );
    }
    if let Some(var_256) = &input.start_time_millis {
        object.key("StartTimeMillis").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_256).into()),
        );
    }
    if let Some(var_257) = &input.finish_time_millis {
        object.key("FinishTimeMillis").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_257).into()),
        );
    }
}

pub fn serialize_structure_job_watermark(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::JobWatermark,
) {
    if let Some(var_258) = &input.preset_watermark_id {
        object.key("PresetWatermarkId").string(var_258);
    }
    if let Some(var_259) = &input.input_key {
        object.key("InputKey").string(var_259);
    }
    if let Some(var_260) = &input.encryption {
        let mut object_261 = object.key("Encryption").start_object();
        crate::json_ser::serialize_structure_encryption(&mut object_261, var_260);
        object_261.finish();
    }
}

pub fn serialize_structure_job_album_art(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::JobAlbumArt,
) {
    if let Some(var_262) = &input.merge_policy {
        object.key("MergePolicy").string(var_262);
    }
    if let Some(var_263) = &input.artwork {
        let mut array_264 = object.key("Artwork").start_array();
        for item_265 in var_263 {
            {
                let mut object_266 = array_264.value().start_object();
                crate::json_ser::serialize_structure_artwork(&mut object_266, item_265);
                object_266.finish();
            }
        }
        array_264.finish();
    }
}

pub fn serialize_structure_clip(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Clip,
) {
    if let Some(var_267) = &input.time_span {
        let mut object_268 = object.key("TimeSpan").start_object();
        crate::json_ser::serialize_structure_time_span(&mut object_268, var_267);
        object_268.finish();
    }
}

pub fn serialize_structure_captions(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Captions,
) {
    if let Some(var_269) = &input.merge_policy {
        object.key("MergePolicy").string(var_269);
    }
    if let Some(var_270) = &input.caption_sources {
        let mut array_271 = object.key("CaptionSources").start_array();
        for item_272 in var_270 {
            {
                let mut object_273 = array_271.value().start_object();
                crate::json_ser::serialize_structure_caption_source(&mut object_273, item_272);
                object_273.finish();
            }
        }
        array_271.finish();
    }
    if let Some(var_274) = &input.caption_formats {
        let mut array_275 = object.key("CaptionFormats").start_array();
        for item_276 in var_274 {
            {
                let mut object_277 = array_275.value().start_object();
                crate::json_ser::serialize_structure_caption_format(&mut object_277, item_276);
                object_277.finish();
            }
        }
        array_275.finish();
    }
}

pub fn serialize_structure_artwork(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Artwork,
) {
    if let Some(var_278) = &input.input_key {
        object.key("InputKey").string(var_278);
    }
    if let Some(var_279) = &input.max_width {
        object.key("MaxWidth").string(var_279);
    }
    if let Some(var_280) = &input.max_height {
        object.key("MaxHeight").string(var_280);
    }
    if let Some(var_281) = &input.sizing_policy {
        object.key("SizingPolicy").string(var_281);
    }
    if let Some(var_282) = &input.padding_policy {
        object.key("PaddingPolicy").string(var_282);
    }
    if let Some(var_283) = &input.album_art_format {
        object.key("AlbumArtFormat").string(var_283);
    }
    if let Some(var_284) = &input.encryption {
        let mut object_285 = object.key("Encryption").start_object();
        crate::json_ser::serialize_structure_encryption(&mut object_285, var_284);
        object_285.finish();
    }
}

pub fn serialize_structure_caption_format(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CaptionFormat,
) {
    if let Some(var_286) = &input.format {
        object.key("Format").string(var_286);
    }
    if let Some(var_287) = &input.pattern {
        object.key("Pattern").string(var_287);
    }
    if let Some(var_288) = &input.encryption {
        let mut object_289 = object.key("Encryption").start_object();
        crate::json_ser::serialize_structure_encryption(&mut object_289, var_288);
        object_289.finish();
    }
}