use std::collections::HashSet;

use diesel::prelude::*;
use error_stack::{Result, ResultExt};
use model::{
    AccountId, AccountIdInternal, ContentId, ContentIdInternal, ContentState,
    CurrentAccountMediaInternal, CurrentAccountMediaRaw, ContentSlot, MediaContentInternal,
    MediaContentRaw, MediaModerationRaw, ModerationQueueNumber, ModerationRequestContent,
    ModerationRequestId, ModerationRequestInternal, MediaModerationRequestRaw, ModerationRequestState,
};
use simple_backend_database::diesel_db::{ConnectionProvider, DieselDatabaseError};

use crate::IntoDatabaseError;

define_read_commands!(CurrentReadMedia, CurrentSyncReadMedia);

impl<C: ConnectionProvider> CurrentSyncReadMedia<C> {
    pub fn moderation_request(
        &mut self,
        request_creator: AccountIdInternal,
    ) -> Result<Option<ModerationRequestInternal>, DieselDatabaseError> {
        let conn = self.conn();
        let request: MediaModerationRequestRaw = {
            use crate::schema::media_moderation_request::dsl::*;

            let request: Option<MediaModerationRequestRaw> = media_moderation_request
                .filter(account_id.eq(request_creator.as_db_id()))
                .select(MediaModerationRequestRaw::as_select())
                .first::<MediaModerationRequestRaw>(conn)
                .optional()
                .into_db_error(DieselDatabaseError::Execute, request_creator)?;

            match request {
                None => return Ok(None),
                Some(r) => r,
            }
        };

        use crate::schema::media_moderation::dsl::*;
        let moderations: Vec<MediaModerationRaw> = media_moderation
            .filter(moderation_request_id.eq(request.id))
            .select(MediaModerationRaw::as_select())
            .load(conn)
            .into_db_error(DieselDatabaseError::Execute, (request_creator, request.id))?;

        let state = match moderations.first() {
            None => ModerationRequestState::Waiting,
            Some(first) => {
                let accepted = moderations
                    .iter()
                    .find(|r| r.state_number == ModerationRequestState::Accepted as i64);
                let denied = moderations
                    .iter()
                    .find(|r| r.state_number == ModerationRequestState::Denied as i64);

                if let Some(accepted) = accepted {
                    ModerationRequestState::Accepted
                } else if let Some(denied) = denied {
                    ModerationRequestState::Denied
                } else {
                    ModerationRequestState::InProgress
                }
            }
        };

        let data: ModerationRequestContent = request.to_moderation_request_content();

        Ok(Some(ModerationRequestInternal::new(
            request.id,
            request_creator.as_id(),
            state,
            data,
        )))
    }

    pub fn current_account_media(
        &mut self,
        media_owner_id: AccountIdInternal,
    ) -> Result<CurrentAccountMediaInternal, DieselDatabaseError> {
        let conn = self.conn();
        let current_media = {
            use crate::schema::current_account_media::dsl::*;

            current_account_media
                .filter(account_id.eq(media_owner_id.as_db_id()))
                .select(CurrentAccountMediaRaw::as_select())
                .first(conn)
                .into_db_error(DieselDatabaseError::Execute, media_owner_id)?
        };

        let security = if let Some(content_id) = current_media.security_content_id {
            use crate::schema::media_content::dsl::*;

            let content = media_content
                .filter(id.eq(content_id))
                .select(MediaContentRaw::as_select())
                .first(conn)
                .into_db_error(DieselDatabaseError::Execute, (media_owner_id, content_id))?;

            Some(content.to_content_id_internal())
        } else {
            None
        };

        let profile = if let Some(content_id) = current_media.profile_content_id_1 {
            use crate::schema::media_content::dsl::*;

            let content = media_content
                .filter(id.eq(content_id))
                .select(MediaContentRaw::as_select())
                .first(conn)
                .into_db_error(DieselDatabaseError::Execute, (media_owner_id, content_id))?;

            Some(content.to_content_id_internal())
        } else {
            None
        };

        Ok(CurrentAccountMediaInternal {
            security_content_id: security,
            profile_content_id: profile,
            grid_crop_size: current_media.grid_crop_size
                .unwrap_or(CurrentAccountMediaInternal::GRID_CROP_SIZE_DEFAULT),
            grid_crop_x: current_media.grid_crop_x
                .unwrap_or(CurrentAccountMediaInternal::GRID_CROP_X_DEFAULT),
            grid_crop_y: current_media.grid_crop_y
                .unwrap_or(CurrentAccountMediaInternal::GRID_CROP_Y_DEFAULT),
        })
    }

    pub fn get_media_content_raw(
        &mut self,
        content_id: ContentId,
    ) -> Result<MediaContentRaw, DieselDatabaseError> {
        use crate::schema::media_content::dsl::*;
        let content = media_content
            .filter(uuid.eq(content_id))
            .select(MediaContentRaw::as_select())
            .first(self.conn())
            .into_db_error(DieselDatabaseError::Execute, content_id)?;
        Ok(content)
    }

    pub fn get_account_media(
        &mut self,
        media_owner_id: AccountIdInternal,
    ) -> Result<Vec<MediaContentInternal>, DieselDatabaseError> {
        let data: Vec<MediaContentRaw> = {
            use crate::schema::media_content::dsl::*;

            media_content
                .filter(account_id.eq(media_owner_id.as_db_id()))
                .select(MediaContentRaw::as_select())
                .load(self.conn())
                .into_db_error(DieselDatabaseError::Execute, media_owner_id)?
        };

        let content = data
            .into_iter()
            .map(|r| {
                MediaContentInternal {
                    content_id: ContentIdInternal {
                        content_id: r.uuid,
                        content_row_id: r.id,
                    },
                    state: r.content_state,
                    slot_number: r.slot_number,
                    secure_capture: r.secure_capture,
                    contains_face: r.contains_face,
                }
            })
            .collect();

        Ok(content)
    }

    pub fn get_content_id_from_slot(
        &mut self,
        slot_owner: AccountIdInternal,
        slot: ContentSlot,
    ) -> Result<Option<ContentIdInternal>, DieselDatabaseError> {
        let required_state = ContentState::InSlot as i64;
        let required_slot = slot as i64;

        let data: Option<MediaContentRaw> = {
            use crate::schema::media_content::dsl::*;

            media_content
                .filter(account_id.eq(slot_owner.as_db_id()))
                .filter(content_state.eq(required_state))
                .filter(slot_number.eq(required_slot))
                .select(MediaContentRaw::as_select())
                .first(self.conn())
                .optional()
                .into_db_error(DieselDatabaseError::Execute, (slot_owner, slot))?
        };

        Ok(data.map(|data| data.to_content_id_internal()))
    }

    /// Validate moderation request content.
    ///
    /// Returns `Err(DieselDatabaseError::ModerationRequestContentInvalid)` if the
    /// content is invalid.
    pub fn content_validate_moderation_request_content(
        &mut self,
        content_owner: AccountIdInternal,
        request_content: &ModerationRequestContent,
    ) -> Result<(), DieselDatabaseError> {
        let requested_content_set: HashSet<ContentId> = request_content.content().collect();

        let required_state = ContentState::InSlot as i64;
        let data: Vec<MediaContentRaw> = {
            use crate::schema::media_content::dsl::*;

            media_content
                .filter(account_id.eq(content_owner.as_db_id()))
                .filter(content_state.eq(required_state))
                .select(MediaContentRaw::as_select())
                .load(self.conn())
                .into_db_error(DieselDatabaseError::Execute, content_owner)?
        };

        let database_content_set: HashSet<ContentId> = data.into_iter().map(|r| r.uuid).collect();

        if requested_content_set == database_content_set {
            Ok(())
        } else {
            Err(DieselDatabaseError::ModerationRequestContentInvalid)
                .with_info((content_owner, request_content))
        }
    }

    pub fn get_moderation_request_content(
        &mut self,
        owner_id: ModerationRequestId,
    ) -> Result<(MediaModerationRequestRaw, ModerationQueueNumber, AccountId), DieselDatabaseError>
    {
        let (request, account_id) = {
            use crate::schema::{
                account_id, media_moderation_request, media_moderation_request::dsl::*,
            };

            media_moderation_request::table
                .inner_join(account_id::table)
                .filter(id.eq(owner_id.request_row_id))
                .select((
                    MediaModerationRequestRaw::as_select(),
                    AccountIdInternal::as_select(),
                ))
                .first(self.conn())
                .into_db_error(DieselDatabaseError::Execute, owner_id)?
        };

        Ok((
            request.clone(),
            ModerationQueueNumber(request.queue_number),
            account_id.uuid,
        ))
    }
}

// async fn get_content_id_from_row_id(
//     &self,
//     id: i64,
// ) -> error_stack::Result<ContentIdInternal, SqliteDatabaseError> {
//     let request = sqlx::query!(
//         r#"
//         SELECT uuid as "content_id: uuid::Uuid"
//         FROM media_content
//         WHERE account_id = ?
//         "#,
//         id,
//     )
//     .fetch_one(self.pool())
//     .await
//     .map(|r| ContentIdInternal {
//         content_id: r.content_id,
//         content_row_id: id,
//     })
//     .change_context(SqliteDatabaseError::Fetch)?;

//     Ok(request)
// }
