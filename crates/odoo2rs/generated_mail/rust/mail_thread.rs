//! Generado por odoo2rs desde `/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py` — NO EDITAR A MANO;
//! regenerar con `odoo2rs gen-rust`.
//! Modelo: `mail.thread`

use nexus_orm::prelude::*;

pub struct MailThreadFragment;

#[async_trait]
impl ModelFragment for MailThreadFragment {
    fn model_name(&self) -> &str {
        "mail.thread"
    }

    fn module(&self) -> &str {
        "mail"
    }

    fn build(&self, def: &mut ModelDef) {
        def.description = "Email Thread".into();
        def.add_field(FieldDef::boolean("message_is_follower").string("Is Follower").computed("_compute_message_is_follower", &["message_follower_ids"]).stored());
        def.add_field(FieldDef::one2many("message_follower_ids", "mail.followers", "res_id").string("Followers"));
        def.add_field(FieldDef::many2many("message_partner_ids", "res.partner").string("Followers (Partners)").computed("_compute_message_partner_ids", &["message_follower_ids"]).stored());
        def.add_field(FieldDef::one2many("message_ids", "mail.message", "res_id").string("Messages"));
        def.add_field(FieldDef::boolean("has_message").computed("_compute_has_message", &[]));
        def.add_field(FieldDef::boolean("message_needaction").string("Action Needed").computed("_compute_message_needaction", &[]).stored());
        def.add_field(FieldDef::integer("message_needaction_counter").string("Number of Actions").computed("_compute_message_needaction", &[]).stored());
        def.add_field(FieldDef::boolean("message_has_error").string("Message Delivery error").computed("_compute_message_has_error", &[]).stored());
        def.add_field(FieldDef::integer("message_has_error_counter").string("Number of errors").computed("_compute_message_has_error", &[]).stored());
        def.add_field(FieldDef::integer("message_attachment_count").string("Attachment Count").computed("_compute_message_attachment_count", &[]).stored());
    }

    fn methods(&self) -> Vec<&str> {
        vec!["_compute_message_partner_ids", "_inverse_message_partner_ids", "_search_message_partner_ids", "_compute_message_is_follower", "_search_message_is_follower", "_compute_has_message", "_search_has_message", "_compute_message_needaction", "_search_message_needaction", "_compute_message_has_error", "_search_message_has_error", "_compute_message_attachment_count", "create", "write", "unlink", "copy_data", "get_empty_list_help", "get_views", "_compute_field_value", "_creation_subtype", "_creation_message", "_valid_field_parameter", "_fallback_lang", "_check_can_update_message_content", "_track_prepare", "_track_discard", "_track_filter_for_display", "_track_finalize", "_track_set_author", "_track_post_template_finalize", "_track_set_log_message", "_track_get_default_log_message", "_track_get_fields", "_track_subtype", "_message_track", "_message_track_post_template", "_track_template", "_routing_warn", "_routing_create_bounce_email", "_routing_handle_bounce", "_routing_check_route", "_routing_reset_bounce", "_detect_is_bounce", "_detect_loop_sender_domain", "_detect_loop_sender", "_detect_loop_headers", "_detect_write_to_catchall", "message_route", "_message_route_process", "message_process", "message_new", "message_update", "_message_receive_bounce", "_message_reset_bounce", "_message_parse_extract_payload_postprocess", "_message_parse_extract_payload", "_message_parse_extract_bounce", "message_parse", "_message_parse_extract_from_parent", "_message_parse_post_process", "_get_bounced_message_data", "_get_parent_message", "_partner_find_from_emails_single", "_partner_find_from_emails", "_mail_find_user_for_gateway", "_mail_find_partner_from_emails", "_get_customer_information", "message_post", "_message_post_after_hook", "_message_mail_after_hook", "_process_attachments_for_post", "_create_attachments_for_post", "_process_attachments_for_template_post", "message_mail_with_source", "message_post_with_source", "message_notify", "_message_log_with_view", "_message_log", "_message_log_batch", "_message_compute_author", "_message_compute_real_author", "_message_compute_parent_id", "_message_compute_subject", "_message_create", "_get_message_create_valid_field_names", "_get_message_create_ignore_field_names", "_get_source_from_ref", "_get_notify_valid_parameters", "_is_notification_scheduled", "_raise_for_invalid_parameters", "_notify_cancel_by_type_generic", "notify_cancel_by_type", "_notify_thread", "_notify_thread_by_inbox", "_notify_thread_by_email", "_notify_get_classified_recipients_iterator", "_notify_by_email_prepare_rendering_context", "_notify_by_email_render_layout", "_notify_by_email_get_base_mail_values", "_notify_by_email_get_final_mail_values", "_notify_by_email_get_base_notification_values", "_notify_thread_by_web_push", "_web_push_get_partners_parameters", "_web_push_send_notification", "_notify_by_web_push_prepare_payload", "_notify_get_recipients", "_notify_get_recipients_groups", "_notify_get_recipients_groups_fillup", "_notify_get_recipients_classify", "_notify_get_recipients_for_extra_notifications", "_notify_get_action_link", "_notify_thread_with_out_of_office", "_notify_thread_with_out_of_office_get_additional_users", "_encode_link", "_get_action_link_params", "_generate_tracking_message", "_get_model_description", "_web_push_truncate_payload", "_truncate_payload_get_max_payload_length", "message_subscribe", "_message_subscribe", "message_unsubscribe", "_message_auto_subscribe_followers", "_message_auto_subscribe_notify", "_message_auto_subscribe", "message_get_followers", "_message_followers_to_store", "message_change_thread", "_message_update_content", "_clean_empty_message", "_get_store_message_update_extra_fields", "_thread_to_store", "_get_mail_thread_data_attachments", "_get_allowed_message_params", "_get_allowed_access_params", "_get_thread_with_access"]
    }

    async fn call(
        &self,
        env: &Env,
        ctx: &CallCtx,
        rs: &Recordset,
        args: &[OVal],
    ) -> OResult<OVal> {
        match ctx.method() {
            "_compute_message_partner_ids" => self._compute_message_partner_ids(env, ctx, rs, args).await,
            "_inverse_message_partner_ids" => self._inverse_message_partner_ids(env, ctx, rs, args).await,
            "_search_message_partner_ids" => self._search_message_partner_ids(env, ctx, rs, args).await,
            "_compute_message_is_follower" => self._compute_message_is_follower(env, ctx, rs, args).await,
            "_search_message_is_follower" => self._search_message_is_follower(env, ctx, rs, args).await,
            "_compute_has_message" => self._compute_has_message(env, ctx, rs, args).await,
            "_search_has_message" => self._search_has_message(env, ctx, rs, args).await,
            "_compute_message_needaction" => self._compute_message_needaction(env, ctx, rs, args).await,
            "_search_message_needaction" => self._search_message_needaction(env, ctx, rs, args).await,
            "_compute_message_has_error" => self._compute_message_has_error(env, ctx, rs, args).await,
            "_search_message_has_error" => self._search_message_has_error(env, ctx, rs, args).await,
            "_compute_message_attachment_count" => self._compute_message_attachment_count(env, ctx, rs, args).await,
            "create" => self.create(env, ctx, rs, args).await,
            "write" => self.write(env, ctx, rs, args).await,
            "unlink" => self.unlink(env, ctx, rs, args).await,
            "copy_data" => self.copy_data(env, ctx, rs, args).await,
            "get_empty_list_help" => self.get_empty_list_help(env, ctx, rs, args).await,
            "get_views" => self.get_views(env, ctx, rs, args).await,
            "_compute_field_value" => self._compute_field_value(env, ctx, rs, args).await,
            "_creation_subtype" => self._creation_subtype(env, ctx, rs, args).await,
            "_creation_message" => self._creation_message(env, ctx, rs, args).await,
            "_valid_field_parameter" => self._valid_field_parameter(env, ctx, rs, args).await,
            "_fallback_lang" => self._fallback_lang(env, ctx, rs, args).await,
            "_check_can_update_message_content" => self._check_can_update_message_content(env, ctx, rs, args).await,
            "_track_prepare" => self._track_prepare(env, ctx, rs, args).await,
            "_track_discard" => self._track_discard(env, ctx, rs, args).await,
            "_track_filter_for_display" => self._track_filter_for_display(env, ctx, rs, args).await,
            "_track_finalize" => self._track_finalize(env, ctx, rs, args).await,
            "_track_set_author" => self._track_set_author(env, ctx, rs, args).await,
            "_track_post_template_finalize" => self._track_post_template_finalize(env, ctx, rs, args).await,
            "_track_set_log_message" => self._track_set_log_message(env, ctx, rs, args).await,
            "_track_get_default_log_message" => self._track_get_default_log_message(env, ctx, rs, args).await,
            "_track_get_fields" => self._track_get_fields(env, ctx, rs, args).await,
            "_track_subtype" => self._track_subtype(env, ctx, rs, args).await,
            "_message_track" => self._message_track(env, ctx, rs, args).await,
            "_message_track_post_template" => self._message_track_post_template(env, ctx, rs, args).await,
            "_track_template" => self._track_template(env, ctx, rs, args).await,
            "_routing_warn" => self._routing_warn(env, ctx, rs, args).await,
            "_routing_create_bounce_email" => self._routing_create_bounce_email(env, ctx, rs, args).await,
            "_routing_handle_bounce" => self._routing_handle_bounce(env, ctx, rs, args).await,
            "_routing_check_route" => self._routing_check_route(env, ctx, rs, args).await,
            "_routing_reset_bounce" => self._routing_reset_bounce(env, ctx, rs, args).await,
            "_detect_is_bounce" => self._detect_is_bounce(env, ctx, rs, args).await,
            "_detect_loop_sender_domain" => self._detect_loop_sender_domain(env, ctx, rs, args).await,
            "_detect_loop_sender" => self._detect_loop_sender(env, ctx, rs, args).await,
            "_detect_loop_headers" => self._detect_loop_headers(env, ctx, rs, args).await,
            "_detect_write_to_catchall" => self._detect_write_to_catchall(env, ctx, rs, args).await,
            "message_route" => self.message_route(env, ctx, rs, args).await,
            "_message_route_process" => self._message_route_process(env, ctx, rs, args).await,
            "message_process" => self.message_process(env, ctx, rs, args).await,
            "message_new" => self.message_new(env, ctx, rs, args).await,
            "message_update" => self.message_update(env, ctx, rs, args).await,
            "_message_receive_bounce" => self._message_receive_bounce(env, ctx, rs, args).await,
            "_message_reset_bounce" => self._message_reset_bounce(env, ctx, rs, args).await,
            "_message_parse_extract_payload_postprocess" => self._message_parse_extract_payload_postprocess(env, ctx, rs, args).await,
            "_message_parse_extract_payload" => self._message_parse_extract_payload(env, ctx, rs, args).await,
            "_message_parse_extract_bounce" => self._message_parse_extract_bounce(env, ctx, rs, args).await,
            "message_parse" => self.message_parse(env, ctx, rs, args).await,
            "_message_parse_extract_from_parent" => self._message_parse_extract_from_parent(env, ctx, rs, args).await,
            "_message_parse_post_process" => self._message_parse_post_process(env, ctx, rs, args).await,
            "_get_bounced_message_data" => self._get_bounced_message_data(env, ctx, rs, args).await,
            "_get_parent_message" => self._get_parent_message(env, ctx, rs, args).await,
            "_partner_find_from_emails_single" => self._partner_find_from_emails_single(env, ctx, rs, args).await,
            "_partner_find_from_emails" => self._partner_find_from_emails(env, ctx, rs, args).await,
            "_mail_find_user_for_gateway" => self._mail_find_user_for_gateway(env, ctx, rs, args).await,
            "_mail_find_partner_from_emails" => self._mail_find_partner_from_emails(env, ctx, rs, args).await,
            "_get_customer_information" => self._get_customer_information(env, ctx, rs, args).await,
            "message_post" => self.message_post(env, ctx, rs, args).await,
            "_message_post_after_hook" => self._message_post_after_hook(env, ctx, rs, args).await,
            "_message_mail_after_hook" => self._message_mail_after_hook(env, ctx, rs, args).await,
            "_process_attachments_for_post" => self._process_attachments_for_post(env, ctx, rs, args).await,
            "_create_attachments_for_post" => self._create_attachments_for_post(env, ctx, rs, args).await,
            "_process_attachments_for_template_post" => self._process_attachments_for_template_post(env, ctx, rs, args).await,
            "message_mail_with_source" => self.message_mail_with_source(env, ctx, rs, args).await,
            "message_post_with_source" => self.message_post_with_source(env, ctx, rs, args).await,
            "message_notify" => self.message_notify(env, ctx, rs, args).await,
            "_message_log_with_view" => self._message_log_with_view(env, ctx, rs, args).await,
            "_message_log" => self._message_log(env, ctx, rs, args).await,
            "_message_log_batch" => self._message_log_batch(env, ctx, rs, args).await,
            "_message_compute_author" => self._message_compute_author(env, ctx, rs, args).await,
            "_message_compute_real_author" => self._message_compute_real_author(env, ctx, rs, args).await,
            "_message_compute_parent_id" => self._message_compute_parent_id(env, ctx, rs, args).await,
            "_message_compute_subject" => self._message_compute_subject(env, ctx, rs, args).await,
            "_message_create" => self._message_create(env, ctx, rs, args).await,
            "_get_message_create_valid_field_names" => self._get_message_create_valid_field_names(env, ctx, rs, args).await,
            "_get_message_create_ignore_field_names" => self._get_message_create_ignore_field_names(env, ctx, rs, args).await,
            "_get_source_from_ref" => self._get_source_from_ref(env, ctx, rs, args).await,
            "_get_notify_valid_parameters" => self._get_notify_valid_parameters(env, ctx, rs, args).await,
            "_is_notification_scheduled" => self._is_notification_scheduled(env, ctx, rs, args).await,
            "_raise_for_invalid_parameters" => self._raise_for_invalid_parameters(env, ctx, rs, args).await,
            "_notify_cancel_by_type_generic" => self._notify_cancel_by_type_generic(env, ctx, rs, args).await,
            "notify_cancel_by_type" => self.notify_cancel_by_type(env, ctx, rs, args).await,
            "_notify_thread" => self._notify_thread(env, ctx, rs, args).await,
            "_notify_thread_by_inbox" => self._notify_thread_by_inbox(env, ctx, rs, args).await,
            "_notify_thread_by_email" => self._notify_thread_by_email(env, ctx, rs, args).await,
            "_notify_get_classified_recipients_iterator" => self._notify_get_classified_recipients_iterator(env, ctx, rs, args).await,
            "_notify_by_email_prepare_rendering_context" => self._notify_by_email_prepare_rendering_context(env, ctx, rs, args).await,
            "_notify_by_email_render_layout" => self._notify_by_email_render_layout(env, ctx, rs, args).await,
            "_notify_by_email_get_base_mail_values" => self._notify_by_email_get_base_mail_values(env, ctx, rs, args).await,
            "_notify_by_email_get_final_mail_values" => self._notify_by_email_get_final_mail_values(env, ctx, rs, args).await,
            "_notify_by_email_get_base_notification_values" => self._notify_by_email_get_base_notification_values(env, ctx, rs, args).await,
            "_notify_thread_by_web_push" => self._notify_thread_by_web_push(env, ctx, rs, args).await,
            "_web_push_get_partners_parameters" => self._web_push_get_partners_parameters(env, ctx, rs, args).await,
            "_web_push_send_notification" => self._web_push_send_notification(env, ctx, rs, args).await,
            "_notify_by_web_push_prepare_payload" => self._notify_by_web_push_prepare_payload(env, ctx, rs, args).await,
            "_notify_get_recipients" => self._notify_get_recipients(env, ctx, rs, args).await,
            "_notify_get_recipients_groups" => self._notify_get_recipients_groups(env, ctx, rs, args).await,
            "_notify_get_recipients_groups_fillup" => self._notify_get_recipients_groups_fillup(env, ctx, rs, args).await,
            "_notify_get_recipients_classify" => self._notify_get_recipients_classify(env, ctx, rs, args).await,
            "_notify_get_recipients_for_extra_notifications" => self._notify_get_recipients_for_extra_notifications(env, ctx, rs, args).await,
            "_notify_get_action_link" => self._notify_get_action_link(env, ctx, rs, args).await,
            "_notify_thread_with_out_of_office" => self._notify_thread_with_out_of_office(env, ctx, rs, args).await,
            "_notify_thread_with_out_of_office_get_additional_users" => self._notify_thread_with_out_of_office_get_additional_users(env, ctx, rs, args).await,
            "_encode_link" => self._encode_link(env, ctx, rs, args).await,
            "_get_action_link_params" => self._get_action_link_params(env, ctx, rs, args).await,
            "_generate_tracking_message" => self._generate_tracking_message(env, ctx, rs, args).await,
            "_get_model_description" => self._get_model_description(env, ctx, rs, args).await,
            "_web_push_truncate_payload" => self._web_push_truncate_payload(env, ctx, rs, args).await,
            "_truncate_payload_get_max_payload_length" => self._truncate_payload_get_max_payload_length(env, ctx, rs, args).await,
            "message_subscribe" => self.message_subscribe(env, ctx, rs, args).await,
            "_message_subscribe" => self._message_subscribe(env, ctx, rs, args).await,
            "message_unsubscribe" => self.message_unsubscribe(env, ctx, rs, args).await,
            "_message_auto_subscribe_followers" => self._message_auto_subscribe_followers(env, ctx, rs, args).await,
            "_message_auto_subscribe_notify" => self._message_auto_subscribe_notify(env, ctx, rs, args).await,
            "_message_auto_subscribe" => self._message_auto_subscribe(env, ctx, rs, args).await,
            "message_get_followers" => self.message_get_followers(env, ctx, rs, args).await,
            "_message_followers_to_store" => self._message_followers_to_store(env, ctx, rs, args).await,
            "message_change_thread" => self.message_change_thread(env, ctx, rs, args).await,
            "_message_update_content" => self._message_update_content(env, ctx, rs, args).await,
            "_clean_empty_message" => self._clean_empty_message(env, ctx, rs, args).await,
            "_get_store_message_update_extra_fields" => self._get_store_message_update_extra_fields(env, ctx, rs, args).await,
            "_thread_to_store" => self._thread_to_store(env, ctx, rs, args).await,
            "_get_mail_thread_data_attachments" => self._get_mail_thread_data_attachments(env, ctx, rs, args).await,
            "_get_allowed_message_params" => self._get_allowed_message_params(env, ctx, rs, args).await,
            "_get_allowed_access_params" => self._get_allowed_access_params(env, ctx, rs, args).await,
            "_get_thread_with_access" => self._get_thread_with_access(env, ctx, rs, args).await,
            other => Err(OError::Internal(format!(
                "método '{other}' no implementado en este fragmento"
            ))),
        }
    }
}

impl MailThreadFragment {
    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:169`). Decoradores: api.depends('message_follower_ids').
    async fn _compute_message_partner_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._compute_message_partner_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:173`).
    async fn _inverse_message_partner_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._inverse_message_partner_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:193`). Decoradores: api.model.
    async fn _search_message_partner_ids(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._search_message_partner_ids".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:212`). Decoradores: api.depends('message_follower_ids').
    async fn _compute_message_is_follower(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._compute_message_is_follower".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:222`). Decoradores: api.model.
    async fn _search_message_is_follower(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._search_message_is_follower".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:232`).
    async fn _compute_has_message(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._compute_has_message".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:244`).
    async fn _search_has_message(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._search_has_message".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:249`).
    async fn _compute_message_needaction(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._compute_message_needaction".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:266`). Decoradores: api.model.
    async fn _search_message_needaction(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._search_message_needaction".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:269`).
    async fn _compute_message_has_error(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._compute_message_has_error".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:291`). Decoradores: api.model.
    async fn _search_message_has_error(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._search_message_has_error".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:297`).
    async fn _compute_message_attachment_count(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._compute_message_attachment_count".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:311`). Decoradores: api.model_create_multi.
    async fn create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread.create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:382`).
    async fn write(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread.write".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:397`).
    async fn unlink(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread.unlink".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:412`).
    async fn copy_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread.copy_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:417`). Decoradores: api.model.
    async fn get_empty_list_help(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread.get_empty_list_help".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:469`). Decoradores: api.model.
    async fn get_views(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread.get_views".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:479`).
    async fn _compute_field_value(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._compute_field_value".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:485`).
    async fn _creation_subtype(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._creation_subtype".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:492`).
    async fn _creation_message(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._creation_message".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:500`).
    async fn _valid_field_parameter(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._valid_field_parameter".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:504`).
    async fn _fallback_lang(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._fallback_lang".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:509`).
    async fn _check_can_update_message_content(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._check_can_update_message_content".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:525`).
    async fn _track_prepare(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._track_prepare".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:550`).
    async fn _track_discard(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._track_discard".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:560`).
    async fn _track_filter_for_display(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._track_filter_for_display".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:565`).
    async fn _track_finalize(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._track_finalize".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:581`).
    async fn _track_set_author(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._track_set_author".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:589`).
    async fn _track_post_template_finalize(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._track_post_template_finalize".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:593`).
    async fn _track_set_log_message(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._track_set_log_message".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:603`).
    async fn _track_get_default_log_message(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._track_get_default_log_message".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:614`). Decoradores: ormcache('self.env.uid', 'self.env.su').
    async fn _track_get_fields(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._track_get_fields".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:631`).
    async fn _track_subtype(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._track_subtype".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:643`).
    async fn _message_track(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._message_track".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:701`).
    async fn _message_track_post_template(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._message_track_post_template".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:741`).
    async fn _track_template(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._track_template".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:748`).
    async fn _routing_warn(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._routing_warn".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:758`).
    async fn _routing_create_bounce_email(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._routing_create_bounce_email".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:784`). Decoradores: api.model.
    async fn _routing_handle_bounce(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._routing_handle_bounce".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:845`). Decoradores: api.model.
    async fn _routing_check_route(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._routing_check_route".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:942`). Decoradores: api.model.
    async fn _routing_reset_bounce(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._routing_reset_bounce".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:957`). Decoradores: api.model.
    async fn _detect_is_bounce(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._detect_is_bounce".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:990`). Decoradores: api.model.
    async fn _detect_loop_sender_domain(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._detect_loop_sender_domain".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:1002`). Decoradores: api.model.
    async fn _detect_loop_sender(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._detect_loop_sender".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:1090`). Decoradores: api.model.
    async fn _detect_loop_headers(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._detect_loop_headers".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:1100`). Decoradores: api.model.
    async fn _detect_write_to_catchall(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._detect_write_to_catchall".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:1120`). Decoradores: api.model.
    async fn message_route(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread.message_route".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:1332`). Decoradores: api.model.
    async fn _message_route_process(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._message_route_process".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:1423`). Decoradores: api.model.
    async fn message_process(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread.message_process".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:1492`). Decoradores: api.model.
    async fn message_new(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread.message_new".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:1525`).
    async fn message_update(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread.message_update".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:1543`).
    async fn _message_receive_bounce(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._message_receive_bounce".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:1556`).
    async fn _message_reset_bounce(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._message_reset_bounce".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:1566`).
    async fn _message_parse_extract_payload_postprocess(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._message_parse_extract_payload_postprocess".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:1605`).
    async fn _message_parse_extract_payload(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._message_parse_extract_payload".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:1693`).
    async fn _message_parse_extract_bounce(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._message_parse_extract_bounce".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:1764`). Decoradores: api.model.
    async fn message_parse(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread.message_parse".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:1876`).
    async fn _message_parse_extract_from_parent(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._message_parse_extract_from_parent".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:1887`).
    async fn _message_parse_post_process(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._message_parse_post_process".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:1910`).
    async fn _get_bounced_message_data(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._get_bounced_message_data".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:1947`).
    async fn _get_parent_message(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._get_parent_message".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:1980`).
    async fn _partner_find_from_emails_single(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._partner_find_from_emails_single".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:1994`).
    async fn _partner_find_from_emails(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._partner_find_from_emails".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:2097`).
    async fn _mail_find_user_for_gateway(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._mail_find_user_for_gateway".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:2126`). Decoradores: api.model.
    async fn _mail_find_partner_from_emails(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._mail_find_partner_from_emails".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:2159`).
    async fn _get_customer_information(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._get_customer_information".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:2176`).
    async fn message_post(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread.message_post".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:2352`).
    async fn _message_post_after_hook(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._message_post_after_hook".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:2358`).
    async fn _message_mail_after_hook(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._message_mail_after_hook".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:2364`).
    async fn _process_attachments_for_post(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._process_attachments_for_post".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:2504`).
    async fn _create_attachments_for_post(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._create_attachments_for_post".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:2510`).
    async fn _process_attachments_for_template_post(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._process_attachments_for_template_post".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:2529`).
    async fn message_mail_with_source(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread.message_mail_with_source".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:2615`).
    async fn message_post_with_source(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread.message_post_with_source".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:2700`).
    async fn message_notify(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread.message_notify".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:2837`).
    async fn _message_log_with_view(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._message_log_with_view".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:2872`).
    async fn _message_log(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._message_log".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:2890`).
    async fn _message_log_batch(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._message_log_batch".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:2946`).
    async fn _message_compute_author(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._message_compute_author".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:2970`).
    async fn _message_compute_real_author(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._message_compute_real_author".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:2984`).
    async fn _message_compute_parent_id(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._message_compute_parent_id".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:3009`).
    async fn _message_compute_subject(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._message_compute_subject".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:3019`).
    async fn _message_create(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._message_create".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:3049`).
    async fn _get_message_create_valid_field_names(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._get_message_create_valid_field_names".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:3085`).
    async fn _get_message_create_ignore_field_names(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._get_message_create_ignore_field_names".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:3091`).
    async fn _get_source_from_ref(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._get_source_from_ref".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:3147`).
    async fn _get_notify_valid_parameters(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._get_notify_valid_parameters".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:3174`). Decoradores: api.model.
    async fn _is_notification_scheduled(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._is_notification_scheduled".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:3191`).
    async fn _raise_for_invalid_parameters(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._raise_for_invalid_parameters".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:3214`).
    async fn _notify_cancel_by_type_generic(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._notify_cancel_by_type_generic".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:3243`). Decoradores: api.model.
    async fn notify_cancel_by_type(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread.notify_cancel_by_type".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:3256`).
    async fn _notify_thread(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._notify_thread".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:3317`).
    async fn _notify_thread_by_inbox(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._notify_thread_by_inbox".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:3371`).
    async fn _notify_thread_by_email(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._notify_thread_by_email".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:3503`).
    async fn _notify_get_classified_recipients_iterator(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._notify_get_classified_recipients_iterator".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:3600`).
    async fn _notify_by_email_prepare_rendering_context(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._notify_by_email_prepare_rendering_context".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:3715`).
    async fn _notify_by_email_render_layout(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._notify_by_email_render_layout".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:3759`).
    async fn _notify_by_email_get_base_mail_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._notify_by_email_get_base_mail_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:3844`).
    async fn _notify_by_email_get_final_mail_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._notify_by_email_get_final_mail_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:3863`).
    async fn _notify_by_email_get_base_notification_values(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._notify_by_email_get_base_notification_values".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:3872`).
    async fn _notify_thread_by_web_push(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._notify_thread_by_web_push".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:3897`).
    async fn _web_push_get_partners_parameters(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._web_push_get_partners_parameters".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:3911`).
    async fn _web_push_send_notification(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._web_push_send_notification".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:3951`).
    async fn _notify_by_web_push_prepare_payload(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._notify_by_web_push_prepare_payload".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:4008`).
    async fn _notify_get_recipients(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._notify_get_recipients".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:4140`).
    async fn _notify_get_recipients_groups(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._notify_get_recipients_groups".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:4221`).
    async fn _notify_get_recipients_groups_fillup(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._notify_get_recipients_groups_fillup".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:4257`).
    async fn _notify_get_recipients_classify(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._notify_get_recipients_classify".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:4320`).
    async fn _notify_get_recipients_for_extra_notifications(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._notify_get_recipients_for_extra_notifications".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:4340`).
    async fn _notify_get_action_link(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._notify_get_action_link".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:4362`).
    async fn _notify_thread_with_out_of_office(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._notify_thread_with_out_of_office".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:4445`).
    async fn _notify_thread_with_out_of_office_get_additional_users(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._notify_thread_with_out_of_office_get_additional_users".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:4472`). Decoradores: api.model.
    async fn _encode_link(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._encode_link".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:4478`).
    async fn _get_action_link_params(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._get_action_link_params".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:4501`). Decoradores: api.model.
    async fn _generate_tracking_message(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._generate_tracking_message".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:4528`). Decoradores: api.model.
    async fn _get_model_description(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._get_model_description".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:4536`). Decoradores: api.model.
    async fn _web_push_truncate_payload(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._web_push_truncate_payload".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:4593`). Decoradores: staticmethod.
    async fn _truncate_payload_get_max_payload_length(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._truncate_payload_get_max_payload_length".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:4610`).
    async fn message_subscribe(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread.message_subscribe".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:4634`).
    async fn _message_subscribe(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._message_subscribe".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:4663`).
    async fn message_unsubscribe(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread.message_unsubscribe".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:4681`).
    async fn _message_auto_subscribe_followers(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._message_auto_subscribe_followers".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:4715`).
    async fn _message_auto_subscribe_notify(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._message_auto_subscribe_notify".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:4749`).
    async fn _message_auto_subscribe(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._message_auto_subscribe".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:4819`). Decoradores: api.readonly.
    async fn message_get_followers(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread.message_get_followers".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:4825`).
    async fn _message_followers_to_store(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._message_followers_to_store".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:4855`).
    async fn message_change_thread(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread.message_change_thread".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:4895`).
    async fn _message_update_content(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._message_update_content".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:4988`).
    async fn _clean_empty_message(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._clean_empty_message".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:4991`).
    async fn _get_store_message_update_extra_fields(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._get_store_message_update_extra_fields".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:4998`).
    async fn _thread_to_store(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._thread_to_store".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:5057`).
    async fn _get_mail_thread_data_attachments(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._get_mail_thread_data_attachments".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:5073`).
    async fn _get_allowed_message_params(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._get_allowed_message_params".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:5081`). Decoradores: api.model.
    async fn _get_allowed_access_params(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._get_allowed_access_params".into(),
        ))
    }

    /// TODO(odoo2rs FASE 3a): traducir el cuerpo original (`/home/ealvarez/workspace/erp/opt/odoo/server/nexustech/addons/mail/models/mail_thread.py:5085`). Decoradores: api.model.
    async fn _get_thread_with_access(
        &self,
        _env: &Env,
        _ctx: &CallCtx,
        _rs: &Recordset,
        _args: &[OVal],
    ) -> OResult<OVal> {
        Err(OError::Internal(
            "pendiente de transpilar (FASE 3): mail.thread._get_thread_with_access".into(),
        ))
    }

}
