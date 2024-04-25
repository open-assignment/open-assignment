use crate::authentication_token::Claims;
use async_graphql::*;
use diesel::{Connection, PgConnection};
use uuid::Uuid;

use crate::authorization::{
    DocumentActionPermission, OrganizationActionPermission, SpaceActionPermission,
};
use crate::db::*;
use crate::error::{OpenExamError, OpenExamErrorExt};
use crate::helper::*;
use crate::util::get_now_as_secs;

#[derive(SimpleObject)]
pub struct SpaceWithAccessToken {
    pub space: Space,
    pub access_token: String,
}

#[derive(Default)]
pub struct SpaceMutation;

#[Object]
impl SpaceMutation {
    async fn space_create(&self, ctx: &Context<'_>, mut data: NewSpace) -> Result<Space> {
        let user = get_user_from_ctx(ctx).await?;
        let user_auth = get_user_auth_from_ctx(ctx).await?;
        organization_authorize(
            ctx,
            user.id,
            user_auth.org_id,
            OrganizationActionPermission::AddSpace,
        )
        .await?;

        data.creator_id = user.id;
        data.org_id = user_auth.org_id;
        let conn = get_conn_from_ctx(ctx).await?;

        Space::insert(&conn, data).format_err()
    }

    async fn space_duplicate(&self, ctx: &Context<'_>, space_id: i32) -> Result<Space> {
        let user_auth = get_user_auth_from_ctx(ctx).await?;
        space_quick_authorize(ctx, space_id, SpaceActionPermission::ManageSpaceSetting).await?;

        let conn = get_conn_from_ctx(ctx).await?;
        let new_class =
            duplicate_class(&conn, space_id, user_auth.org_id, user_auth.id).format_err()?;
        Ok(new_class)
    }

    async fn space_update(
        &self,
        ctx: &Context<'_>,
        space_id: i32,
        data: UpdateSpaceData,
    ) -> Result<bool> {
        space_quick_authorize(ctx, space_id, SpaceActionPermission::ManageSpaceSetting).await?;

        let conn = get_conn_from_ctx(ctx).await?;
        Space::update(&conn, space_id, data).format_err()?;
        Ok(true)
    }

    async fn space_restore(&self, ctx: &Context<'_>, space_id: i32) -> Result<Space> {
        space_quick_authorize(ctx, space_id, SpaceActionPermission::ManageSpaceSetting).await?;

        let conn = get_conn_from_ctx(ctx).await?;
        let class = Space::restore(&conn, space_id).format_err()?;
        Ok(class)
    }

    async fn space_soft_delete(&self, ctx: &Context<'_>, space_id: i32) -> Result<bool> {
        space_quick_authorize(ctx, space_id, SpaceActionPermission::ManageSpaceSetting).await?;

        let conn = get_conn_from_ctx(ctx).await?;
        Space::soft_remove(&conn, space_id).format_err()?;
        Ok(true)
    }

    async fn space_delete(&self, ctx: &Context<'_>, space_id: i32) -> Result<bool> {
        space_quick_authorize(ctx, space_id, SpaceActionPermission::ManageSpaceSetting).await?;

        let conn = get_conn_from_ctx(ctx).await?;
        Space::remove(&conn, space_id).format_err()?;
        Ok(true)
    }

    async fn space_duplicate_document(
        &self,
        ctx: &Context<'_>,
        space_id: i32,
        document_id: Uuid,
    ) -> Result<Vec<Document>> {
        let user_auth = get_user_auth_from_ctx(ctx).await?;
        space_quick_authorize(ctx, space_id, SpaceActionPermission::ManageSpaceContent).await?;

        let conn = get_conn_from_ctx(ctx).await?;

        let original_document = Document::find_by_id(&conn, document_id).format_err()?;
        if original_document.deleted_at.is_some() {
            return Err(OpenExamError::new_bad_request(
                "Cannot duplicate deleted document!",
            ))
            .format_err();
        }
        let last_index = Document::find_last_index(&conn, space_id, original_document.parent_id)?;
        let mut config = DocumentCloneConfig::new("Copy of ", true);
        config.set_index(last_index);
        config.set_parent(original_document.parent_id);
        config.set_org(user_auth.org_id);

        let doc = conn
            .transaction::<_, OpenExamError, _>(|| {
                original_document.deep_clone(
                    &conn,
                    user_auth.id,
                    config,
                    Some(space_id),
                    true,
                    None,
                    true,
                )
            })
            .format_err()?;

        let mut res: Vec<Document> = vec![];
        res.append(&mut get_all_documents_by_id(&conn, doc.id).format_err()?);
        res.push(doc);
        Ok(res)
    }

    async fn space_remove_document(&self, ctx: &Context<'_>, document_id: Uuid) -> Result<bool> {
        document_quick_authorize(ctx, document_id, DocumentActionPermission::ManageDocument)
            .await?;

        let conn = get_conn_from_ctx(ctx).await?;
        Document::delete(&conn, document_id).format_err()?;

        Ok(true)
    }

    async fn space_soft_delete_document(
        &self,
        ctx: &Context<'_>,
        space_id: i32,
        document_id: Uuid,
    ) -> Result<bool> {
        space_quick_authorize(ctx, space_id, SpaceActionPermission::ManageSpaceContent).await?;

        let conn = get_conn_from_ctx(ctx).await?;
        Document::soft_delete(&conn, document_id).format_err()?;

        Ok(true)
    }

    async fn space_soft_delete_multiple(
        &self,
        ctx: &Context<'_>,
        space_id: i32,
        document_ids: Vec<Uuid>,
    ) -> Result<bool> {
        space_quick_authorize(ctx, space_id, SpaceActionPermission::ManageSpaceContent).await?;

        let conn = get_conn_from_ctx(ctx).await?;
        let now = get_now_as_secs();
        Document::soft_delete_by_ids(&conn, document_ids, Some(now)).format_err()?;

        Ok(true)
    }

    async fn space_restore_soft_delete_document(
        &self,
        ctx: &Context<'_>,
        space_id: i32,
        document_ids: Vec<Uuid>,
    ) -> Result<bool> {
        space_quick_authorize(ctx, space_id, SpaceActionPermission::ManageSpaceContent).await?;

        let conn = get_conn_from_ctx(ctx).await?;
        Document::soft_delete_by_ids(&conn, document_ids, None).format_err()?;
        Ok(true)
    }

    async fn space_add_member(
        &self,
        ctx: &Context<'_>,
        space_id: i32,
        user_id: i32,
    ) -> Result<SpaceMember> {
        space_quick_authorize(ctx, space_id, SpaceActionPermission::ManageSpaceMember).await?;

        let conn = get_conn_from_ctx(ctx).await?;
        let space = Space::find_by_id(&conn, space_id).format_err()?;
        add_space_member(&conn, &space, user_id)
    }

    async fn space_add_members(
        &self,
        ctx: &Context<'_>,
        space_id: i32,
        user_ids: Vec<i32>,
    ) -> Result<Vec<SpaceMember>> {
        space_quick_authorize(ctx, space_id, SpaceActionPermission::ManageSpaceMember).await?;

        let conn = get_conn_from_ctx(ctx).await?;
        let space = Space::find_by_id(&conn, space_id).format_err()?;
        let mut members = vec![];
        for user_id in user_ids {
            if let Ok(member) = add_space_member(&conn, &space, user_id) {
                members.push(member);
            }
        }

        Ok(members)
    }

    async fn space_remove_member(
        &self,
        ctx: &Context<'_>,
        space_id: i32,
        user_id: i32,
    ) -> Result<bool> {
        space_quick_authorize(ctx, space_id, SpaceActionPermission::ManageSpaceMember).await?;

        let conn = get_conn_from_ctx(ctx).await?;
        SpaceMember::find(&conn, space_id, user_id).format_err()?;
        SpaceMember::remove(&conn, space_id, user_id).format_err()?;

        Ok(true)
    }

    async fn space_generate_invite_token(
        &self,
        ctx: &Context<'_>,
        data: SpaceInviteToken,
    ) -> Result<SpaceInviteToken> {
        space_quick_authorize(ctx, data.space_id, SpaceActionPermission::ManageSpaceMember).await?;

        let conn = get_conn_from_ctx(ctx).await?;
        let item = SpaceInviteToken::upsert(&conn, data).format_err()?;

        Ok(item)
    }

    async fn space_set_active_invite_token(
        &self,
        ctx: &Context<'_>,
        space_id: i32,
        token: String,
        is_active: bool,
    ) -> Result<SpaceInviteToken> {
        space_quick_authorize(ctx, space_id, SpaceActionPermission::ManageSpaceMember).await?;

        let conn = get_conn_from_ctx(ctx).await?;
        let item = SpaceInviteToken::set_active(&conn, space_id, token, is_active).format_err()?;

        Ok(item)
    }

    async fn space_join_by_invite_token(
        &self,
        ctx: &Context<'_>,
        email: String,
        space_id: i32,
        token: String,
    ) -> Result<SpaceWithAccessToken> {
        let conn = get_conn_from_ctx(ctx).await?;
        let space = Space::find_by_id(&conn, space_id).format_err()?;
        let space_invite_token = SpaceInviteToken::find(&conn, space_id, token).format_err()?;

        if !space_invite_token.is_active {
            return Err(OpenExamError::new_bad_request("Link is inactive!")).format_err();
        }

        if let Some(expire_at) = space_invite_token.expire_at {
            let now = get_now_as_secs();
            if expire_at < now {
                return Err(OpenExamError::new_bad_request("Link is expired!")).format_err();
            }
        }

        let user = if let Some(user) = User::find_by_email_opt(&conn, &email)? {
            user
        } else {
            let new_user = NewUser::new(email.clone(), email, "".into());
            User::insert(&conn, &new_user).format_err()?
        };

        if OrganizationMember::find(&conn, space.org_id, user.id).is_err() {
            let member =
                OrganizationMember::new(space.org_id, user.id, space_invite_token.inviting_role);
            OrganizationMember::upsert(&conn, member).format_err()?;
        }

        let space_member = add_space_member(&conn, &space, user.id)?;
        let claims = Claims::new(space_member.user_id);
        let access_token = claims.encode()?;

        Ok(SpaceWithAccessToken {
            space,
            access_token,
        })
    }
}

fn add_space_member(conn: &PgConnection, space: &Space, user_id: i32) -> Result<SpaceMember> {
    OrganizationMember::find(conn, space.org_id, user_id).format_err()?;
    let new_member = SpaceMember::new(space.id, user_id);
    let new_member = SpaceMember::upsert(conn, new_member).format_err()?;

    Ok(new_member)
}
